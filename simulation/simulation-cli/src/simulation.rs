use rand::Rng;

use crate::{agent::Agent, Config};

#[derive(Debug, Default)]
pub struct Simulation {
    pub agents: Vec<Agent>,
    pub j: u8,
    pub k: u8,
    pub interaction_count: u64,
    // TODO: add opinion_distribution
}

impl Simulation {
    pub fn new(config: Config) -> Self {
        let mut agents = vec![];
        let mut rng = rand::thread_rng();

        // Create config.agent_count agents with a random opinion between
        // 0 and config.opinion_count.
        for _ in 0..config.agent_count {
            let opinion = rng.gen_range(0..config.opinion_count);
            agents.push(Agent::new(opinion));
        }

        Simulation {
            agents,
            j: config.sample_size,
            k: config.opinion_count,
            ..Simulation::default()
        }
    }
    /// Checks whether all agents hold the same opinion.
    pub fn reached_consensus(&self) -> bool {
        if let Some(first_agent) = self.agents.first() {
            let first_opinion = first_agent.opinion;
            self.agents
                .iter()
                .all(|agent| first_opinion.eq(&agent.opinion))
        } else {
            // If the simulation has no agents, we instantly reach consensus.
            true
        }
    }
    // TODO: Add constructor methods supporting a bias.
}

#[cfg(test)]
mod simulation {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn achieve_consensus_with_at_most_one_agent(#[case] agent_count: u64) {
        let config = Config {
            agent_count,
            sample_size: 5,
            opinion_count: 5,
        };
        let simulation_without_agents = Simulation::new(config);
        assert!(simulation_without_agents.reached_consensus());
    }
}
