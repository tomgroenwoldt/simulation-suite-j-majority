use std::collections::HashMap;

use crate::schema::opinion_distribution::OpinionDistribution;

impl Default for OpinionDistribution {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            interaction_count: 0,
            progress: 0.0,
            opinion: 2,
        }
    }
}

impl From<&OpinionDistribution> for (u16, u64) {
    fn from(value: &OpinionDistribution) -> Self {
        (value.opinion, value.interaction_count)
    }
}

impl OpinionDistribution {
    pub fn next(previous_opinion: u16) -> Self {
        Self {
            map: HashMap::default(),
            interaction_count: 0,
            progress: 0.0,
            opinion: previous_opinion + 1,
        }
    }

    pub fn update(&mut self, old_opinion: Option<u16>, new_opinion: u16) -> u64 {
        if let Some(old_opinion) = old_opinion {
            self.map.entry(old_opinion).and_modify(|v| *v -= 1);
        }
        let updated_count = *self
            .map
            .entry(new_opinion)
            .and_modify(|v| *v += 1)
            .or_insert_with(|| 1);
        self.interaction_count += 1;
        updated_count
    }

    pub fn calculate_entropy(&self, n: u64) -> f32 {
        let opinion_percentages = self
            .map
            .values()
            .map(|agents_with_opinion| *agents_with_opinion as f32 / n as f32)
            .collect::<Vec<f32>>();
        let mut entropy = 0.0;
        for percentage in opinion_percentages.into_iter() {
            if percentage != 0.0 {
                entropy -= percentage * percentage.log(self.opinion as f32);
            }
        }
        entropy
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
