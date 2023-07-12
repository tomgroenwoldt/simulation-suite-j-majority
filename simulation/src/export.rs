use std::collections::HashMap;

use pgfplots::{
    axis::{
        plot::{coordinate::Coordinate2D, MarkShape, Marker, Plot2D, PlotKey},
        Axis, AxisKey,
    },
    Engine, Picture,
};

use crate::entropy::Entropy;

#[derive(Default, Debug, Clone)]
pub struct SimulationExport {
    pub plots: Vec<OpinionPlot>,
    pub entropies: Vec<Entropy>,
}

#[derive(Default, Debug, Clone)]
pub struct OpinionPlot {
    pub points: Vec<(u16, u64)>,
    pub j: u8,
}

impl From<&OpinionPlot> for MarkShape {
    fn from(val: &OpinionPlot) -> Self {
        match val.j {
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

impl From<&OpinionPlot> for PlotKey {
    fn from(val: &OpinionPlot) -> Self {
        match val.j {
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
    pub fn generate_pdf(&mut self) {
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
        axis.add_key(AxisKey::Custom(format!("legend entries={{{}}}", entries)));
        axis.add_key(AxisKey::Custom(String::from(
            "legend style={
        at={(0.5,1.05)}, % adjust the values to center the legend
        anchor=south,
        align=center}",
        )));
        axis.add_key(AxisKey::Custom(String::from("legend columns=-1")));
        axis.add_key(AxisKey::Custom(String::from("nodes={inner sep=5pt}")));
        axis.add_key(AxisKey::Custom(String::from("xmode=log")));
        axis.add_key(AxisKey::Custom(String::from("log ticks with fixed point")));
        axis.plots = plots;
        Picture::from(axis).show_pdf(Engine::PdfLatex).unwrap();

        let mut plots = vec![];
        self.entropies.sort_by(|entropy_one, entropy_two| {
            entropy_one.sample_size.cmp(&entropy_two.sample_size)
        });
        self.entropies.dedup_by_key(|entropy| entropy.sample_size);
        self.entropies.iter().for_each(|entropy| {
            let mut pgf_plot = Plot2D::new();
            let mut points = entropy
                .map
                .iter()
                .map(|(x, y)| {
                    let pgf_point = (*x as f64, *y as f64);
                    pgf_point.into()
                })
                .collect::<Vec<Coordinate2D>>();
            points.sort_by(|coord_one, coord_two| coord_one.x.total_cmp(&coord_two.x));
            pgf_plot.coordinates = points;
            pgf_plot.add_key(PlotKey::Custom(String::from("mark size=1pt")));
            plots.push(pgf_plot);
        });
        let mut axis = Axis::new();
        let mut entries = self
            .entropies
            .iter()
            .map(|entropy| format!("{}-Maj.", entropy.sample_size))
            .collect::<Vec<_>>();
        entries.dedup();
        let entries = entries.join(",");
        axis.add_key(AxisKey::Custom(format!("legend entries={{{}}}", entries)));
        axis.add_key(AxisKey::Custom(String::from(
            "legend style={
        at={(0.5,1.05)}, % adjust the values to center the legend
        anchor=south,
        align=center}",
        )));
        axis.add_key(AxisKey::Custom(String::from("legend columns=-1")));
        axis.add_key(AxisKey::Custom(String::from("nodes={inner sep=5pt}")));
        axis.set_x_label("Interactions");
        axis.set_y_label("Entropy");
        axis.plots = plots;
        Picture::from(axis).show_pdf(Engine::PdfLatex).unwrap();
    }

    pub fn average(plots: Vec<OpinionPlot>, entropies: Vec<Entropy>) -> (OpinionPlot, Entropy) {
        let mut point_map: HashMap<u16, u64> = HashMap::new();
        let mut j = 0;
        plots.iter().for_each(|plot| {
            plot.points.iter().for_each(|(x, y)| {
                point_map
                    .entry(*x)
                    .and_modify(|v| *v += y)
                    .or_insert_with(|| *y);
            });
            j = plot.j;
        });
        let points = point_map
            .iter()
            .map(|(x, y)| (*x, y / plots.len() as u64))
            .collect::<Vec<_>>();
        point_map.clear();

        let mut map: HashMap<u64, f32> = HashMap::new();
        let mut sample_size = 0;
        entropies.iter().for_each(|entropy| {
            entropy.map.iter().for_each(|(x, y)| {
                map.entry(*x).and_modify(|v| *v += y).or_insert_with(|| *y);
            });
            sample_size = entropy.sample_size;
        });
        map.values_mut().for_each(|v| *v /= entropies.len() as f32);
        (OpinionPlot { points, j }, Entropy { map, sample_size })
    }
}
