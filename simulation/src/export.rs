use std::collections::HashMap;

use pgfplots::{
    axis::{
        plot::{MarkShape, Marker, Plot2D, PlotKey},
        Axis,
    },
    Engine, Picture,
};
use tracing::info;

#[derive(Default, Debug, Clone)]
pub struct SimulationExport {
    pub plots: Vec<OpinionPlot>,
}

#[derive(Default, Debug, Clone)]
pub struct OpinionPlot {
    pub points: Vec<(u8, u64)>,
    pub j: u8,
}

impl Into<MarkShape> for &OpinionPlot {
    fn into(self) -> MarkShape {
        info!(self.j);
        match self.j {
            3 => MarkShape::Plus,
            4 => MarkShape::X,
            5 => MarkShape::Asterisk,
            6 => MarkShape::Square,
            7 => MarkShape::SquareFilled,
            8 => MarkShape::O,
            9 => MarkShape::OFilled,
            10 => MarkShape::Triangle,
            11 => MarkShape::TriangleFilled,
            12 => MarkShape::Diamond,
            _ => MarkShape::DiamondFilled,
        }
    }
}

impl SimulationExport {
    pub fn to_pdf(&self) {
        let mut plots = vec![];
        self.plots.iter().for_each(|plot| {
            let mut pgf_plot = Plot2D::new();
            pgf_plot.coordinates = plot
                .points
                .iter()
                .map(|point| {
                    let pgf_point = (point.0 as f64, point.1 as f64);
                    pgf_point.into()
                })
                .collect();
            // pgf_plot.add_key(PlotKey::Marker(Marker::new(MarkShape::OFilled, vec![])));
            pgf_plot.add_key(PlotKey::Marker(Marker::new(plot.into(), vec![])));
            pgf_plot.add_key(PlotKey::Type2D(pgfplots::axis::plot::Type2D::OnlyMarks));
            plots.push(pgf_plot);
        });
        let mut axis = Axis::new();
        axis.set_title("My plot title");
        axis.set_x_label("Opinion");
        axis.set_y_label("Convergence time");
        axis.plots = plots;

        Picture::from(axis).show_pdf(Engine::PdfLatex).unwrap();
    }

    pub fn average(exports: Vec<SimulationExport>) -> OpinionPlot {
        let mut point_map: HashMap<u8, u64> = HashMap::new();
        let mut j = 0;
        exports.iter().for_each(|export| {
            export.plots.iter().for_each(|plot| {
                plot.points.iter().for_each(|(x, y)| {
                    point_map
                        .entry(*x)
                        .and_modify(|v| *v += y)
                        .or_insert_with(|| *y);
                });
                j = plot.j;
            });
        });
        let points = point_map
            .iter()
            .map(|(x, y)| (x.clone(), y / exports.len() as u64))
            .collect::<Vec<_>>();
        point_map.clear();
        OpinionPlot { points, j }
    }
}
