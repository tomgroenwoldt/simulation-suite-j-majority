use futures::executor::block_on;
use rand::{
    distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, seq::SliceRandom, Rng,
};
use std::{
    collections::HashMap,
    sync::{mpsc::SyncSender, Arc, Condvar, Mutex},
    thread,
};
use tokio::sync::broadcast::Receiver;
use tracing::{error, info};

use crate::{agent::Agent, error::AppError, Config};

#[derive(Debug, Default, Clone)]
pub struct OpinionDistribution {
    pub map: HashMap<u8, u64>,
}

impl OpinionDistribution {
    pub fn update(&mut self, old_opinion: Option<u8>, new_opinion: u8) -> u64 {
        if let Some(old_opinion) = old_opinion {
            self.map.entry(old_opinion).and_modify(|v| *v -= 1);
        }
        let updated_count = *self
            .map
            .entry(new_opinion)
            .and_modify(|v| *v += 1)
            .or_insert_with(|| 1);
        updated_count
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

#[derive(Debug)]
pub struct Simulation {
    /// Collection of agents.
    pub agents: Vec<Agent>,
    /// The sample size for each agent.
    pub j: u8,
    /// The amount of different opinions.
    pub k: u8,
    /// Counts the interactions of all agents.
    pub interaction_count: u64,
    /// Stores number of occurences for each opinion.
    pub opinion_distribution: OpinionDistribution,
    pub sender: SyncSender<SimulationMessage>,
    /// State and condition variable, which can block the executor thread
    /// of the simulation.
    pub state: Arc<(Mutex<SimulationState>, Condvar)>,
}

#[derive(Debug, PartialEq)]
pub enum SimulationState {
    Pause,
    Play,
    Exit,
}

#[derive(Clone, Debug)]
pub enum SimulationMessage {
    Pause,
    Play,
    Abort,
    Update((Option<u8>, u8, u64)),
    Finish,
}

#[derive(Debug, Default)]
pub struct FrontendSimulation {
    pub opinion_distribution: OpinionDistribution,
    pub interaction_count: u64,
    pub finished: bool,
}

impl Simulation {
    pub fn new(config: Config, sender: SyncSender<SimulationMessage>) -> Self {
        let mut rng = rand::thread_rng();
        let mut agents = vec![];
        let mut opinion_distribution = OpinionDistribution::default();
        let weighted_index = WeightedIndex::new(config.weights.into_values().collect::<Vec<_>>())
            .unwrap_or(WeightedIndex::new(vec![1; config.opinion_count as usize]).unwrap());
        let choices = (0..config.opinion_count).collect::<Vec<_>>();

        // Create agents with random opinions and generate the opinion
        // distribution.
        for _ in 0..config.agent_count {
            let new_opinion = choices[weighted_index.sample(&mut rng)];

            opinion_distribution.update(None, new_opinion);
            if sender
                .send(SimulationMessage::Update((None, new_opinion, 0)))
                .is_err()
            {
                error!("Error sending initial simulation updates to egui!");
            }
            agents.push(Agent::new(new_opinion));
        }

        info!("Agent initilization finished!");

        Simulation {
            agents,
            j: config.sample_size,
            k: config.opinion_count,
            opinion_distribution,
            interaction_count: 0,
            sender,
            state: Arc::new((Mutex::new(SimulationState::Play), Condvar::new())),
        }
    }

    /// Starts the simulation loop and exits if all agents agree on the
    /// same opinion.
    pub fn execute(&mut self, receiver: Receiver<SimulationMessage>) -> Result<(), AppError> {
        // Return on a single opinion, as consensus is already reached.
        if self.k.eq(&1) {
            return Ok(());
        }
        let mut receiver = receiver.resubscribe();

        let state = Arc::clone(&self.state);

        // Message handler which communicates with the GUI.
        thread::spawn(move || loop {
            if let Ok(msg) = block_on(receiver.recv()) {
                let (lock, cvar) = &*state;
                let mut state = lock.lock().unwrap();
                match msg {
                    SimulationMessage::Pause => {
                        *state = SimulationState::Pause;
                    }
                    SimulationMessage::Play => {
                        *state = SimulationState::Play;
                    }
                    SimulationMessage::Abort => {
                        *state = SimulationState::Exit;
                    }
                    _ => {}
                }
                cvar.notify_one();
            }
        });

        let mut rng = rand::thread_rng();
        let (lock, cvar) = &*self.state;

        loop {
            let mut state = lock.lock().unwrap();
            if state.eq(&SimulationState::Exit) {
                if self.sender.send(SimulationMessage::Finish).is_err() {
                    error!("Error sending simulation the finish message.");
                }
                break;
            }
            // Do nothing on pause
            while state.eq(&SimulationState::Pause) {
                state = cvar.wait(state).unwrap();
            }
            drop(state);
            let (chosen_agent, sample) =
                Simulation::prepare_interaction(&mut self.agents, self.j, &mut rng)?;
            let old_opinion = chosen_agent.opinion;

            // Update agent opinion and set new opinion distribution.
            let new_opinion = chosen_agent.update(&sample, &mut self.interaction_count)?;
            let updated_opinion_count = self
                .opinion_distribution
                .update(Some(old_opinion), new_opinion);
            if self
                .sender
                .send(SimulationMessage::Update((
                    Some(old_opinion),
                    new_opinion,
                    self.interaction_count,
                )))
                .is_err()
            {
                error!("Error sending simulation updates to egui!");
            }

            // Exit simulation if all agents agree on one opinion.
            if updated_opinion_count.eq(&(self.agents.len() as u64)) {
                info!(
                    "Agents reached consensus with {} interactions!",
                    self.interaction_count
                );
                let mut state = lock.lock().unwrap();
                *state = SimulationState::Exit;
            }
        }
        Ok(())
    }

    /// Chooses and returns a agent uniformly at random as well as a sample of
    /// given size.
    pub fn prepare_interaction<'a>(
        agents: &'a mut Vec<Agent>,
        sample_size: u8,
        rng: &mut ThreadRng,
    ) -> Result<(&'a mut Agent, Vec<Agent>), AppError> {
        let n = agents.len();

        if n.eq(&0) {
            return Err(AppError::EmptyAgents);
        }

        // Swap a random agent to the first position. This way we can always
        // split the vector via `.split_first_mut()` to work via references.
        agents.swap(0, rng.gen_range(0..n));
        let (chosen_agent, remaining) = agents.split_first_mut().unwrap();
        let sample = remaining
            .choose_multiple(rng, sample_size as usize)
            .cloned()
            .collect::<Vec<_>>();

        Ok((chosen_agent, sample))
    }
}

#[cfg(test)]
mod simulation {
    use super::*;
    use rstest::{fixture, rstest};
    use std::sync::mpsc::sync_channel;
    use tokio::sync::broadcast::{channel, Receiver};

    #[fixture]
    fn sender() -> SyncSender<SimulationMessage> {
        let (tx, _) = sync_channel(1000);
        tx
    }

    #[fixture]
    fn receiver() -> Receiver<SimulationMessage> {
        let (tx, _) = channel(1000);
        tx.subscribe()
    }

    #[rstest]
    fn can_create(sender: SyncSender<SimulationMessage>) {
        let config = Config::default();
        let simulation = Simulation::new(config.clone(), sender);

        assert_eq!(simulation.agents.len() as u64, config.agent_count);
        assert_eq!(simulation.j, config.sample_size);
        assert_eq!(simulation.k, config.opinion_count);
    }

    #[rstest]
    fn single_opinion_leads_to_exit() -> Result<(), AppError> {
        let config = Config {
            agent_count: 10,
            sample_size: 5,
            opinion_count: 1,
            weights: HashMap::new(),
        };
        let mut simulation = Simulation::new(config, sender());
        simulation.execute(receiver())?;
        assert_eq!(simulation.interaction_count, 0);
        Ok(())
    }

    #[rstest]
    #[case(32)]
    #[case(64)]
    #[case(128)]
    #[case(255)]
    fn two_agents_only_need_one_interaction(#[case] opinion_count: u8) -> Result<(), AppError> {
        let config = Config {
            agent_count: 2,
            sample_size: 2,
            opinion_count,
            weights: HashMap::new(),
        };
        let mut simulation = Simulation::new(config, sender());
        simulation.execute(receiver())?;
        assert_eq!(simulation.interaction_count, 1);
        Ok(())
    }

    #[rstest]
    fn prepare_with_empty_agents_leads_to_err() -> Result<(), AppError> {
        let mut agents = vec![];
        let mut rng = rand::thread_rng();
        let result = Simulation::prepare_interaction(&mut agents, 10, &mut rng);
        assert!(result.is_err());
        Ok(())
    }
}
