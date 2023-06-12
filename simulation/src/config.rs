use std::collections::HashMap;

use crate::simulation::SimulationModel;

#[derive(Clone, Debug)]
pub struct Config {
    pub agent_count: u64,
    pub sample_size: u8,
    pub opinion_count: u8,
    pub weights: HashMap<u8, u8>,
    pub simulation_count: u8,
    pub model: SimulationModel,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent_count: 100000,
            sample_size: 2,
            opinion_count: 2,
            weights: HashMap::new(),
            simulation_count: 1,
            model: SimulationModel::Population,
        }
    }
}
