use std::collections::HashMap;

use pgfplots::{
    axis::{
        plot::{MarkShape, Marker, Plot2D, PlotKey},
        Axis, AxisKey,
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

impl Into<PlotKey> for &OpinionPlot {
    fn into(self) -> PlotKey {
        match self.j {
            3 => PlotKey::Custom(String::from("color=red")),
            4 => PlotKey::Custom(String::from("color=green")),
            5 => PlotKey::Custom(String::from("color=blue")),
            6 => PlotKey::Custom(String::from("color=cyan")),
            7 => PlotKey::Custom(String::from("color=magenta")),
            8 => PlotKey::Custom(String::from("color=brown")),
            9 => PlotKey::Custom(String::from("color=violet")),
            10 => PlotKey::Custom(String::from("color=orange")),
            11 => PlotKey::Custom(String::from("color=darkgray")),
            12 => PlotKey::Custom(String::from("color=teal")),
            _ => PlotKey::Custom(String::from("color=black")),
        }
    }
}

impl SimulationExport {
    pub fn to_pdf(&mut self) {
        let mut plots = vec![];
        self.plots
            .sort_by(|plot_one, plot_two| plot_one.j.cmp(&plot_two.j));
        self.plots.dedup_by_key(|plot| plot.j);
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
            pgf_plot.add_key(plot.into());
            pgf_plot.add_key(PlotKey::Type2D(pgfplots::axis::plot::Type2D::OnlyMarks));
            plots.push(pgf_plot);
        });
        let mut axis = Axis::new();
        axis.set_x_label("Opinions");
        axis.set_y_label("Interactions");
        let mut entries = self
            .plots
            .iter()
            .map(|plot| format!("{}-Maj.", plot.j))
            .collect::<Vec<_>>();
        entries.dedup();
        let entries = entries.join(",");
        axis.add_key(AxisKey::Custom(String::from(format!(
            "legend entries={{{}}}",
            entries
        ))));
        axis.add_key(AxisKey::Custom(String::from(
            "legend style={
        at={(0.5,1.05)}, % adjust the values to center the legend
        anchor=south,
        align=center}",
        )));
        axis.add_key(AxisKey::Custom(String::from("legend columns=-1")));
        axis.add_key(AxisKey::Custom(String::from("nodes={inner sep=5pt}")));
        axis.plots = plots;
        info!("{}", axis.to_string());
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
