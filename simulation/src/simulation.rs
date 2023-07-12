use std::{
    sync::{mpsc::SyncSender, Arc, Condvar, Mutex},
    thread,
};

use futures::executor::block_on;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use tokio::sync::broadcast::Receiver;

use crate::{
    entropy::Entropy,
    error::AppError,
    export::OpinionPlot,
    schema::{
        agent::Agent,
        opinion_distribution::OpinionDistribution,
        simulation::{Simulation, SimulationMessage, SimulationState},
    },
    Config,
};

impl Simulation {
    pub fn new(config: Config, sender: SyncSender<SimulationMessage>) -> Result<Self, AppError> {
        let mut rng = rand::thread_rng();

        // Create agents with random opinions and generate the opinion
        // distribution.
        let mut agents = vec![];
        let mut opinion_distribution = OpinionDistribution::default();
        let entropy = Entropy::new(config.sample_size);
        let choices = (0..opinion_distribution.opinion).collect::<Vec<u16>>();
        for _ in 0..config.agent_count {
            let new_opinion = choices.choose(&mut rng).unwrap();
            opinion_distribution.update(None, *new_opinion);
            sender.send(SimulationMessage::Update(opinion_distribution.clone()))?;
            agents.push(Agent::new(*new_opinion));
        }

        let simulation = Simulation {
            agents,
            sample_size: config.sample_size,
            upper_bound_k: config.upper_bound_k + 1,
            opinion_distribution,
            entropy,
            sender,
            state: Arc::new((Mutex::new(SimulationState::Play), Condvar::new())),
            config,
            plot: OpinionPlot::default(),
        };
        Ok(simulation)
    }

    pub fn prepare_next_simulation(&mut self) -> Result<(), AppError> {
        let mut rng = rand::thread_rng();
        self.agents = vec![];
        self.opinion_distribution = OpinionDistribution::next(self.opinion_distribution.opinion);
        // TODO: remove duplicate
        self.opinion_distribution.progress =
            self.opinion_distribution.opinion as f32 / self.upper_bound_k as f32;
        let choices = (0..self.opinion_distribution.opinion).collect::<Vec<_>>();

        // Create agents with random opinions and generate the opinion
        // distribution.
        for _ in 0..self.config.agent_count {
            let new_opinion = choices.choose(&mut rng).unwrap();

            self.opinion_distribution.update(None, *new_opinion);
            self.sender
                .send(SimulationMessage::Update(self.opinion_distribution.clone()))?;
            self.agents.push(Agent::new(*new_opinion));
        }
        Ok(())
    }

    /// Starts the simulation loop and exits if all agents agree on the
    /// same opinion.
    pub fn execute(&mut self, receiver: Receiver<SimulationMessage>) -> Result<(), AppError> {
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

        // TODO: Get rid of this...
        let (lock, _cvar) = &*self.state.clone();

        while !self.quit()? {
            // Calculate the entropy every 'agent_count'th interaction
            if self.opinion_distribution.interaction_count % self.config.agent_count == 0 {
                self.entropy
                    .map
                    .entry(self.opinion_distribution.interaction_count)
                    .and_modify(|v| {
                        *v += self
                            .opinion_distribution
                            .calculate_entropy(self.config.agent_count)
                    })
                    .or_insert_with(|| {
                        self.opinion_distribution
                            .calculate_entropy(self.config.agent_count)
                    });
            }

            self.interact(&mut rng)?;

            // Send update to frontend
            self.sender
                .send(SimulationMessage::Update(self.opinion_distribution.clone()))?;

            if self.reached_consensus()? {
                // Exit simulation if we simulated all k, otherwise ready up for next run.
                let mut state = lock.lock().unwrap();
                if self.opinion_distribution.opinion.eq(&self.upper_bound_k) {
                    self.entropy.average(self.config.upper_bound_k as f32);
                    *state = SimulationState::Exit;
                } else {
                    *state = SimulationState::ReadyForNext;
                    self.sender.send(SimulationMessage::Next)?;
                    self.prepare_next_simulation()?;
                }
            }
        }
        Ok(())
    }

    fn interact(&mut self, rng: &mut ThreadRng) -> Result<(), AppError> {
        // Swap a random agent to the first position. This way we can always
        // split the vector via `.split_first_mut()` to work via references.
        self.agents
            .swap(0, rng.gen_range(0..self.config.agent_count as usize));
        if let Some((chosen_agent, remaining)) = self.agents.split_first_mut() {
            let sample = remaining
                .choose_multiple(rng, self.sample_size as usize)
                .cloned()
                .collect::<Vec<_>>();

            chosen_agent.update(&sample, &mut self.opinion_distribution)?;
        }

        Ok(())
    }

    fn quit(&self) -> Result<bool, AppError> {
        let mut state = self.state.0.lock().unwrap();
        if state.eq(&SimulationState::Exit) {
            self.sender.send(SimulationMessage::Finish(
                self.plot.clone(),
                self.entropy.clone(),
            ))?;
            return Ok(true);
        }
        if state.eq(&SimulationState::ReadyForNext) {
            *state = SimulationState::Play;
            return Ok(false);
        }
        // Do nothing on pause
        while state.eq(&SimulationState::Pause) {
            state = self.state.1.wait(state).unwrap();
        }
        drop(state);
        Ok(false)
    }

    fn reached_consensus(&mut self) -> Result<bool, AppError> {
        if self
            .opinion_distribution
            .check_occurence_with(self.config.agent_count)
        {
            self.plot.j = self.config.sample_size;
            self.plot.points.push((&self.opinion_distribution).into());
            return Ok(true);
        }
        Ok(false)
    }
}
