#[derive(Debug)]
pub struct Config {
    /// Number of agents
    pub n: u64,
    /// Sample size
    pub j: u8,
    /// Number of opinions
    pub k: u16,
    /// # Initial consensus configuration
    ///
    /// Either adds up to provided n with k provided values or takes k-1 values
    /// and shifts remaining n to last element
    pub config: Vec<u64>,
}
