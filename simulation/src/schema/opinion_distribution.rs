use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct OpinionDistribution {
    pub map: HashMap<u16, u64>,
    pub interaction_count: u64,
}
