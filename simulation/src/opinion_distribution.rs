use crate::schema::opinion_distribution::OpinionDistribution;

impl OpinionDistribution {
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

    pub fn calculate_entropy(&self, n: f32) -> f32 {
        let opinion_percentages = self
            .map
            .values()
            .map(|agents_with_opinion| *agents_with_opinion as f32 / n)
            .collect::<Vec<f32>>();
        let mut entropy = 0.0;
        for percentage in opinion_percentages.into_iter() {
            entropy -= percentage * f32::log2(percentage);
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
