use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Agent {
    opinion: u8,
}

impl Agent {
    pub fn new(opinion: u8) -> Self {
        Agent { opinion }
    }

    pub fn update(&mut self, agents: &Vec<Agent>) {
        // If the sample is empty, do not update the
        // agent opinion and return early.
        if agents.is_empty() {
            return;
        }

        // Counts the occurence of each opinion.
        let mut counts = HashMap::new();

        // Find the major opinions in the given sample.
        agents.iter().for_each(|agent| {
            *counts.entry(agent.opinion).or_insert(0) += 1;
        });
        let max_count = counts.values().max().unwrap_or(&0);
        let major_opinions: Vec<u8> = counts
            .iter()
            .filter(|&(_, &count)| count == *max_count)
            .map(|(&elem, _)| elem)
            .collect();

        // On a tie, choose arbitrarily and update.
        if let Some(major_opinion) = major_opinions.choose(&mut rand::thread_rng()) {
            self.opinion = *major_opinion;
        };
    }
}

#[cfg(test)]
mod agent {
    use super::*;
    use rand::Rng;
    use rstest::rstest;

    #[rstest]
    #[case(10, 5)]
    #[case(100, 10)]
    #[case(1000, 100)]
    #[case(10000, 200)]
    fn updates_to_major_opinion(#[case] sample_size: u64, #[case] opinion_count: u8) {
        let mut rng = rand::thread_rng();
        let mut agent = Agent::new(rng.gen());

        // Create n (sample_size) agents with random opinions.
        let random_agents = (0..sample_size)
            .map(|_| {
                let random_opinion = rng.gen_range(0..opinion_count);
                Agent::new(random_opinion)
            })
            .collect::<Vec<_>>();
        // Create n + 1 agents with opinion 0.
        let fixed_agents = (0..sample_size + 1)
            .map(|_| Agent::new(0))
            .collect::<Vec<_>>();

        agent.update(&[random_agents, fixed_agents].concat());

        // The major opinion should always be 0.
        assert_eq!(agent.opinion, 0);
    }

    #[rstest]
    #[case(1)]
    #[case(5)]
    #[case(10)]
    #[case(100)]
    fn updates_to_major_opinion_on_a_tie(#[case] major_opinion_count: u8) {
        let mut rng = rand::thread_rng();
        let mut agent = Agent::new(rng.gen());

        // Create 100 agents with random opinions.
        let random_agents = (0..100)
            .map(|_| {
                let random_opinion = rng.gen_range(0..20);
                Agent::new(random_opinion)
            })
            .collect::<Vec<_>>();
        // Create n + 1 agents with opinion between 0 and major_opinion_count.
        let fixed_agents = (0..major_opinion_count)
            .flat_map(|opinion| (0..101).map(|_| Agent::new(opinion)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        agent.update(&[random_agents, fixed_agents].concat());

        // The updated opinion should be equal to one of
        // the major ones.
        let major_opinions = (0..major_opinion_count).collect::<Vec<_>>();
        assert!(major_opinions.contains(&agent.opinion));
    }

    #[rstest]
    fn does_not_update_on_empty_sample() {
        let mut agent = Agent::new(0);
        let empty_sample = vec![];

        agent.update(&empty_sample);
        assert_eq!(agent.opinion, 0);
    }
}
