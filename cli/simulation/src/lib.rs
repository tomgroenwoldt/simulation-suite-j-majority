use agent::Agent;
use config::Config;
use error::SimulationError;
use opinion_distribution::OpinionDistribution;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

mod agent;
pub mod config;
mod error;
mod opinion_distribution;

#[derive(Debug, Deserialize, Serialize)]
pub struct Simulation {
    /// Collection of agents
    #[serde(skip_deserializing, skip_serializing)]
    pub agents: Vec<Agent>,
    /// Number of agents
    pub n: u64,
    /// Sample size
    pub j: u8,
    /// Number of opinions
    pub k: u16,
    /// Initial configuration
    pub config: Vec<u64>,
    /// Stores number of occurences for each opinion
    #[serde(skip_deserializing, skip_serializing)]
    pub opinion_distribution: OpinionDistribution,
    /// Number of interactions
    pub interaction_count: u64,
}

impl Simulation {
    pub fn new(config: Config) -> Result<Self, SimulationError> {
        let mut agents = vec![];
        let mut opinion_distribution = OpinionDistribution::default();
        let choices = (0..config.k).collect::<Vec<u16>>();
        let weighted_choices = choices
            .into_iter()
            .zip(config.config.clone())
            .collect::<Vec<_>>();
        for (opinion, weight) in weighted_choices {
            opinion_distribution.batch(opinion, weight);
            for _ in 0..weight {
                agents.push(Agent::new(opinion));
            }
        }

        Ok(Simulation {
            agents,
            n: config.n,
            j: config.j,
            k: config.k,
            config: config.config,
            opinion_distribution,
            interaction_count: 0,
        })
    }

    /// Starts the simulation loop and exits if all agents agree on the
    /// same opinion.
    pub fn execute(&mut self) {
        let mut rng = rand::thread_rng();
        while !self.reached_consensus() {
            self.interact(&mut rng);
        }
    }

    fn interact(&mut self, rng: &mut ThreadRng) {
        // Swap a random agent to the first position. This way we can always
        // split the vector via `.split_first_mut()` to work via references.
        self.agents.swap(0, rng.gen_range(0..self.n as usize));
        if let Some((chosen_agent, remaining)) = self.agents.split_first_mut() {
            let sample = remaining
                .choose_multiple(rng, self.j as usize)
                .cloned()
                .collect::<Vec<_>>();

            chosen_agent.update(&sample, &mut self.opinion_distribution);
            self.interaction_count += 1;
        }
    }

    fn reached_consensus(&mut self) -> bool {
        if self.opinion_distribution.check_occurence_with(self.n) {
            return true;
        }
        false
    }
}
