use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OpinionDistribution {
    pub map: HashMap<u16, u64>,
}

impl OpinionDistribution {
    pub fn batch(&mut self, opinion: u16, amount: u64) {
        self.map.insert(opinion, amount);
    }

    pub fn update(&mut self, old_opinion: Option<u16>, new_opinion: u16) {
        if let Some(old_opinion) = old_opinion {
            self.map.entry(old_opinion).and_modify(|v| *v -= 1);
        }
        self.map
            .entry(new_opinion)
            .and_modify(|v| *v += 1)
            .or_insert_with(|| 1);
    }

    pub fn check_occurence_with(&self, occurence_count: u64) -> bool {
        for value in self.map.values() {
            if value.eq(&occurence_count) {
                return true;
            }
        }
        false
    }
}
