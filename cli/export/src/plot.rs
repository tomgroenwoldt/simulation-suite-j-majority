use std::collections::HashMap;

use clap::ValueEnum;
use pgfplots::{
    axis::{
        plot::{Plot2D, PlotKey},
        Axis, AxisKey,
    },
    Engine, Picture,
};
use simulation::Simulation;

#[derive(Clone, Debug, ValueEnum)]
pub enum PlotType {
    K,
}

pub struct OpinionPlotWithIncreasingSampleSize {
    pub simulations: Vec<Simulation>,
}

impl OpinionPlotWithIncreasingSampleSize {
    pub fn generate_pdf(&self) {
        let mut plots = HashMap::new();
        let mut points = HashMap::new();
        self.simulations.iter().for_each(|simulation| {
            let mut first_point = HashMap::new();
            first_point.insert(simulation.k, simulation.interaction_count);
            plots
                .entry(simulation.j)
                .and_modify(|points: &mut HashMap<u16, u64>| {
                    points
                        .entry(simulation.k)
                        .and_modify(|v| *v += simulation.interaction_count)
                        .or_insert(simulation.interaction_count);
                })
                .or_insert(first_point);
            points
                .entry(simulation.k)
                .and_modify(|v| *v += simulation.interaction_count)
                .or_insert(simulation.interaction_count);
        });
        dbg!(&plots);

        plots.values_mut().for_each(|points| {
            points.iter_mut().for_each(|(k, interaction_count)| {
                let length = self
                    .simulations
                    .iter()
                    .filter(|simulation| simulation.k.eq(k))
                    .count();
                dbg!(&length);

                *interaction_count /= length as u64;
            });
        });

        dbg!(&plots);

        let plots = plots
            .iter()
            .map(|plot| {
                let mut pgf_plot = Plot2D::new();
                pgf_plot.coordinates = plot
                    .1
                    .iter()
                    .map(|(k, interaction_count)| (*k as f64, *interaction_count as f64).into())
                    .collect::<Vec<_>>();
                pgf_plot.add_key(PlotKey::Type2D(pgfplots::axis::plot::Type2D::OnlyMarks));
                pgf_plot
            })
            .collect::<Vec<_>>();

        let mut axis = Axis::new();
        axis.set_x_label("Opinions");
        axis.set_y_label("Interactions");
        axis.add_key(AxisKey::Custom(String::from("xmode=log")));
        axis.add_key(AxisKey::Custom(String::from("log ticks with fixed point")));
        axis.plots = plots;
        Picture::from(axis).show_pdf(Engine::PdfLatex).unwrap();
    }
}
