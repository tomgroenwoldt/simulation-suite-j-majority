use crate::Model;

#[derive(Debug)]
pub struct Config {
    /// Number of agents
    pub n: u64,
    /// Sample size
    pub j: u8,
    /// Number of opinions
    pub k: u16,
    /// Initial consensus configuration
    pub config: Vec<u64>,
    /// Process model defining the interaction behavior
    pub model: Model,
}
