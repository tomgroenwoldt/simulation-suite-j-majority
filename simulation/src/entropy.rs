use std::collections::HashMap;

use egui::plot::{PlotPoint, PlotPoints, Points};

#[derive(Debug, Default, Clone)]
pub struct Entropy {
    pub map: HashMap<u64, f32>,
    pub sample_size: u8,
}

impl Entropy {
    pub fn new(sample_size: u8) -> Self {
        Self {
            map: HashMap::default(),
            sample_size,
        }
    }

    pub fn average(&mut self, opinion_count: f32) {
        self.map.values_mut().for_each(|v| *v /= opinion_count);
    }
}

impl From<Entropy> for Points {
    fn from(val: Entropy) -> Points {
        let entropy_plot: Vec<PlotPoint> = val
            .map
            .into_iter()
            .map(|(interaction_count, entropy_value)| {
                PlotPoint::new(interaction_count as f32, entropy_value)
            })
            .collect();
        Points::new(PlotPoints::Owned(entropy_plot))
    }
}
