use std::collections::HashMap;

use itertools::Itertools;
use pgfplots::{
    axis::{plot::Plot2D, Axis, AxisKey},
    Picture,
};

use simulation::Simulation;

pub fn generate_entropy_n_plot(simulations: Vec<Simulation>, _error_bars: bool) -> Option<Picture> {
    if simulations.len().eq(&0) {
        return None;
    }

    let mut point_map = HashMap::new();
    let mut simulation_counts = HashMap::new();

    simulations.into_iter().for_each(|simulation| {
        simulation
            .entropy
            .into_iter()
            .for_each(|(interaction_count, entropy)| {
                point_map
                    .entry((interaction_count, simulation.n))
                    .and_modify(|v| *v += entropy)
                    .or_insert(entropy);
                simulation_counts
                    .entry((interaction_count, simulation.n))
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            });
    });

    point_map.iter_mut().for_each(|(k, v)| {
        if let Some(simulation_count) = simulation_counts.get(k) {
            *v /= *simulation_count as f64;
        }
    });

    let grouped_points = point_map
        .into_iter()
        .sorted_by(|((_, first_n), _), ((_, second_n), _)| first_n.cmp(second_n))
        .group_by(|((_, n), _)| *n)
        .into_iter()
        .map(|(n, group)| {
            (
                n,
                group
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|((interaction_count, _), entropy)| (interaction_count, entropy))
                    .collect_vec(),
            )
        })
        .collect_vec();

    let mut plots: Vec<pgfplots::axis::plot::Plot> = vec![];
    let mut entries = vec![];
    grouped_points
        .into_iter()
        .sorted_by(|(first_n, _), (second_n, _)| first_n.cmp(second_n))
        .for_each(|(n, points)| {
            let mut pgf_plot = Plot2D::new();
            pgf_plot.coordinates = points
                .into_iter()
                .sorted_by(|point_one, point_two| point_one.0.cmp(&point_two.0))
                .map(|(interaction_count, entropy)| (interaction_count as f64, entropy).into())
                .collect_vec();
            // pgf_plot.add_key(PlotKey::Marker(Marker::new(
            //     map_sample_size_to_markshape(n),
            //     vec![],
            // )));
            // pgf_plot.add_key(map_sample_size_to_color(j));
            plots.push(pgf_plot.into());
            entries.push(n);
        });

    entries.dedup();

    let mut axis = Axis::new();
    axis.set_x_label("Interactions");
    axis.set_y_label("Entropy");
    let entries = entries
        .into_iter()
        .map(|n| format!("$n={}$", n))
        .collect_vec()
        .join(",");
    axis.add_key(AxisKey::Custom(format!("legend entries={{{}}}", entries)));
    axis.add_key(AxisKey::Custom(String::from(
        "legend style={
        at={(0.5,1.1)}, % adjust the values to center the legend
        anchor=south,
        align=center}",
    )));
    axis.add_key(AxisKey::Custom(String::from("legend columns=-1")));
    axis.add_key(AxisKey::Custom(String::from("nodes={inner sep=5pt}")));
    axis.plots = plots;
    Some(Picture::from(axis))
}