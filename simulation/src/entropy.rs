use std::collections::HashMap;

use egui::plot::{PlotPoint, PlotPoints};

#[derive(Debug, Default, Clone)]
pub struct Entropy {
    pub map: HashMap<u64, f32>,
}

impl From<Entropy> for PlotPoints {
    fn from(val: Entropy) -> PlotPoints {
        let entropy_plot: Vec<PlotPoint> = (0..val.map.len())
            .map(|i| {
                if let Some(value) = val.map.get(&(i as u64)) {
                    PlotPoint::new(i as f32, *value)
                } else {
                    PlotPoint::new(0.0, 0.0)
                }
            })
            .collect();
        PlotPoints::Owned(entropy_plot)
    }
}
