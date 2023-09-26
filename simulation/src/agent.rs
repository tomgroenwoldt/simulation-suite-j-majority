#[derive(Clone, Debug)]
pub struct Agent {
    pub opinion: u16,
}

use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::opinion_distribution::OpinionDistribution;

impl Agent {
    pub fn new(opinion: u16) -> Self {
        Agent { opinion }
    }

    /// Executes the interaction for an agent and a given sample and updates the simulations
    /// interaction count. Returns the updated opinion as an option.
    pub fn update(&mut self, sample: Vec<&Agent>, opinion_distribution: &mut OpinionDistribution) {
        // Counts the occurence of each opinion and find the major opinion.
        let mut counts = HashMap::new();
        sample.iter().for_each(|agent| {
            *counts.entry(agent.opinion).or_insert(0) += 1;
        });
        let max_count = counts.values().max().unwrap_or(&0);
        let major_opinions: Vec<u16> = counts
            .iter()
            .filter(|&(_, &count)| count == *max_count)
            .map(|(&elem, _)| elem)
            .collect();

        // On a tie, choose arbitrarily and update.
        if let Some(major_opinion) = major_opinions.choose(&mut rand::thread_rng()) {
            opinion_distribution.update(Some(self.opinion), *major_opinion);
            self.opinion = *major_opinion;
        }
    }
}
