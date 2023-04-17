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
    // TODO: Add constructor methods supporting a bias.
}
