use crate::simulation::SimulationModel;

#[derive(Clone, Debug)]
pub struct Config {
    pub agent_count: u64,
    pub sample_size: u8,
    pub opinion_count: u16,
    pub simulation_count: u8,
    pub model: SimulationModel,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent_count: 100000,
            sample_size: 3,
            opinion_count: 10,
            simulation_count: 1,
            model: SimulationModel::Population,
        }
    }
}
