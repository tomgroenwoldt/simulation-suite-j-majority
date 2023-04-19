use rand::{seq::SliceRandom, Rng};
use std::collections::HashMap;

use crate::{agent::Agent, error::AppError, Config};

#[derive(Debug, Default)]
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
}

#[derive(Debug, Default)]
pub struct Simulation {
    // TODO: Investigate wheter a HashMap would be more fitting.
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
}

impl Simulation {
    // TODO: Add constructor methods supporting a bias.
    pub fn new(config: Config) -> Self {
        let mut rng = rand::thread_rng();
        let mut agents = vec![];
        let mut opinion_distribution = OpinionDistribution::default();

        // Create agents with random opinions and generate the opinion
        // distribution.
        for _ in 0..config.agent_count {
            let new_opinion = rng.gen_range(0..config.opinion_count);
            opinion_distribution.update(None, new_opinion);
            agents.push(Agent::new(new_opinion));
        }

        Simulation {
            agents,
            j: config.sample_size,
            k: config.opinion_count,
            opinion_distribution,
            ..Simulation::default()
        }
    }

    /// Starts the simulation loop and exits if all agents agree on the
    /// same opinion.
    pub fn execute(&mut self) -> Result<(), AppError> {
        // Return on a single opinion, as consensus is already reached.
        if self.k.eq(&1) {
            return Ok(());
        }
        // TODO: Add this as state.
        let mut exit = false;

        while !exit {
            let (chosen_agent, sample) = Simulation::prepare_interaction(&mut self.agents, self.j)?;
            let old_opinion = chosen_agent.opinion;

            // Update agent opinion and set new opinion distribution.
            let new_opinion = chosen_agent.update(&sample, &mut self.interaction_count)?;
            let updated_opinion_count = self
                .opinion_distribution
                .update(Some(old_opinion), new_opinion);

            // Exit simulation if all agents agree on the new opinion.
            if updated_opinion_count.eq(&(self.agents.len() as u64)) {
                exit = true;
            }
        }
        Ok(())
    }

    /// Chooses and returns a agent uniformly at random as well as a sample of
    /// given size.
    pub fn prepare_interaction(
        agents: &mut Vec<Agent>,
        sample_size: u8,
    ) -> Result<(&mut Agent, Vec<Agent>), AppError> {
        let mut rng = rand::thread_rng();
        let n = agents.len();
        if n.eq(&0) {
            return Err(AppError::EmptyAgents);
        }
        // Swap a random agent to the first position. This way we can always
        // split the vector via `.split_first_mut()` to work via references.
        agents.swap(0, rng.gen_range(0..n));

        // We can safely unwrap here, because we already checked if agents
        // is empty.
        let (chosen_agent, remaining) = agents.split_first_mut().unwrap();
        // Randomly choose j agents out of remaining vector.
        let sample = remaining
            .choose_multiple(&mut rng, sample_size as usize)
            .cloned()
            .collect::<Vec<_>>();

        Ok((chosen_agent, sample))
    }
}

#[cfg(test)]
mod simulation {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn can_create() {
        let opinion_count = 2;
        let sample_size = 5;
        let agent_count = 10;
        let config = Config {
            agent_count,
            sample_size,
            opinion_count,
        };
        config.validate();

        let simulation = Simulation::new(config);

        assert_eq!(simulation.agents.len(), agent_count as usize);
        assert_eq!(simulation.j, sample_size);
        assert_eq!(simulation.k, opinion_count);
    }

    #[rstest]
    fn single_opinion_leads_to_exit() -> Result<(), AppError> {
        let config = Config {
            agent_count: 10,
            sample_size: 5,
            opinion_count: 1,
        };
        config.validate();
        let mut simulation = Simulation::new(config);
        simulation.execute()?;
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
        };
        config.validate();
        let mut simulation = Simulation::new(config);
        simulation.execute()?;
        assert_eq!(simulation.interaction_count, 1);
        Ok(())
    }

    #[rstest]
    fn prepare_with_empty_agents_leads_to_err() -> Result<(), AppError> {
        let mut agents = vec![];
        let result = Simulation::prepare_interaction(&mut agents, 10);
        assert!(result.is_err());
        Ok(())
    }
}
