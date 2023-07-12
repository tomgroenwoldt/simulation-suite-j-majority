use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OpinionDistribution {
    pub map: HashMap<u16, u64>,
    pub interaction_count: u64,
    pub progress: f32,
    pub opinion: u16,
}
