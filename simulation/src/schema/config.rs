#[derive(Clone, Debug)]
pub struct Config {
    pub agent_count: u64,
    pub sample_size: u8,
    pub upper_bound_k: u16,
    pub simulation_count: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent_count: 100000,
            sample_size: 3,
            upper_bound_k: 5,
            simulation_count: 5,
        }
    }
}
