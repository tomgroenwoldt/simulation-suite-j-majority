use std::collections::HashMap;

use itertools::Itertools;
use pgfplots::{
    axis::{
        plot::{color::PredefinedColor, Color, MarkOption, Marker, Plot3D, PlotKey},
        Axis, AxisKey,
    },
    Picture,
};
use simulation::Simulation;

use crate::util::map_value_to_color;

pub fn generate_triangle_plot(simulations: Vec<Simulation>) -> Option<Picture> {
    if simulations.len().eq(&0) {
        return None;
    }

    let mut point_map = HashMap::new();
    let mut simulation_counts = HashMap::new();

    simulations.into_iter().for_each(|simulation| {
        point_map
            .entry(simulation.config.clone())
            .and_modify(|v| *v += simulation.interaction_count)
            .or_insert(simulation.interaction_count);
        simulation_counts
            .entry(simulation.config)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    });

    point_map.iter_mut().for_each(|(k, v)| {
        if let Some(simulation_count) = simulation_counts.get(k) {
            *v /= simulation_count;
        }
    });

    let lowest_value = *point_map.values().min().unwrap();
    let highest_value = *point_map.values().max().unwrap();

    let triangle_points = point_map
        .into_iter()
        .map(|point| {
            (
                point.0,
                map_value_to_color(point.1, lowest_value, highest_value),
            )
        })
        .collect::<Vec<_>>();

    let grouped_triangle_points = triangle_points
        .into_iter()
        .sorted_by(|point_one, point_two| point_one.1.cmp(&point_two.1))
        .group_by(|point| point.1)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<_>>())
        .collect_vec();

    let mut plots: Vec<pgfplots::axis::plot::Plot> = vec![];

    for points in grouped_triangle_points {
        let mut pgf_plot = Plot3D::new();
        pgf_plot.coordinates = points
            .iter()
            .map(|point| (point.0[0] as f64, point.0[1] as f64, point.0[2] as f64).into())
            .collect_vec();
        pgf_plot.add_key(PlotKey::Type2D(pgfplots::axis::plot::Type2D::OnlyMarks));
        if let Some(point) = points.first() {
            let color_mix = vec![
                (PredefinedColor::Red, point.1 .0),
                (PredefinedColor::Green, point.1 .1),
                (PredefinedColor::Blue, 0),
            ];
            pgf_plot.add_key(PlotKey::Marker(Marker::new(
                pgfplots::axis::plot::MarkShape::OFilled,
                vec![
                    MarkOption::Scale(0.2),
                    MarkOption::Fill(Color::from_mix(color_mix.clone())),
                    MarkOption::Draw(Color::from_mix(color_mix)),
                ],
            )));
        }
        plots.push(pgf_plot.into());
    }

    let mut axis = Axis::new();
    axis.add_key(AxisKey::Custom(String::from("view={40}{60}")));
    axis.plots = plots;

    Some(Picture::from(axis))
}
