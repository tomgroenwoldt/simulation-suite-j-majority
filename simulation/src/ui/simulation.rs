use egui::{
    plot::{Bar, BarChart, Legend, Plot},
    Context,
};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use tracing::{error, info};

use crate::{
    error::AppError,
    export::{OpinionPlot, SimulationExport},
    simulation::{FrontendSimulation, OpinionDistribution, Simulation, SimulationMessage},
    App, State,
};

pub fn render_simulation_header(ctx: &Context, app: &mut App) -> Result<(), AppError> {
    egui::TopBottomPanel::top("simulation_header").show(ctx, |ui| {
        ui.heading("Simulation");
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_enabled_ui(app.state.eq(&State::Config), |ui| {
                if ui.button("Start").clicked() {
                    app.state = State::Simulation;
                    let mut simulations = vec![];
                    let mut senders = vec![];
                    // Create worker threads which are listening to simulation messages.
                    // Each simulation has one assigned worker thread.
                    for _ in 0..app.config.simulation_count {
                        let (sender, receiver) = mpsc::sync_channel::<SimulationMessage>(1000);

                        let frontend_simulation =
                            Arc::new(Mutex::new(FrontendSimulation::default()));
                        info!(app.config.sample_size);
                        let frontend_simulation_clone = Arc::clone(&frontend_simulation);
                        simulations.push(frontend_simulation);
                        senders.push(sender);

                        // Message handler which communicates with the simulation thread.
                        thread::spawn(move || {
                            let mut plot = OpinionPlot::default();
                            loop {
                                if let Ok(msg) = receiver.recv() {
                                    let mut simulation = frontend_simulation_clone.lock().unwrap();
                                    match msg {
                                        SimulationMessage::Update((
                                            old,
                                            new,
                                            new_interaction_count,
                                        )) => {
                                            simulation.opinion_distribution.update(old, new);
                                            simulation.interaction_count = new_interaction_count;
                                        }
                                        SimulationMessage::Next => {
                                            let opinion_count =
                                                simulation.opinion_distribution.map.len() as u8;
                                            let interaction_count = simulation.interaction_count;
                                            plot.points.push((opinion_count, interaction_count));
                                            simulation.opinion_distribution =
                                                OpinionDistribution::default();
                                            simulation.interaction_count = 0;
                                        }
                                        SimulationMessage::Finish => {
                                            simulation.export.plots.push(plot);
                                            simulation.finished = true;
                                            return;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        });
                    }
                    app.senders = senders;
                    app.simulations = simulations;

                    // Execute simulations on assigned threads.
                    for sender in &app.senders {
                        let sender = sender.clone();
                        let config = app.config.clone();
                        let receiver = app.broadcast.subscribe();

                        thread::spawn(move || {
                            let mut simulation = Simulation::new(config, sender);
                            match simulation.execute(receiver) {
                                Ok(_) => {}
                                Err(e) => error!("Error while executing simulation: {e}"),
                            }
                        });
                    }
                }
            });

            ui.add_enabled_ui(app.state.eq(&State::Simulation), |ui| {
                let finished = app
                    .simulations
                    .iter()
                    .all(|sim| sim.lock().unwrap().finished);
                ui.add_enabled_ui(!finished, |ui| {
                    if ui
                        .toggle_value(&mut app.paused, "Pause (experimental)")
                        .clicked()
                    {
                        match app.paused {
                            true => app.broadcast.send(SimulationMessage::Pause)?,
                            false => app.broadcast.send(SimulationMessage::Play)?,
                        };
                    }
                    if ui.button("Abort").clicked() {
                        app.broadcast.send(SimulationMessage::Abort)?;
                        app.paused = false;
                    }
                    Ok::<_, AppError>(())
                });
                ui.add_enabled_ui(finished, |ui| {
                    if ui.button("Reset").clicked() {
                        app.broadcast.send(SimulationMessage::Finish)?;
                        app.simulations.clear();
                        app.state = State::Config;
                    }
                    // if ui.button("Add to export").clicked() {
                    //     let mut interaction_counts = vec![];
                    //     for simulation in app.simulations.iter() {
                    //         let simulation = simulation.lock().unwrap();
                    //         interaction_counts.push(simulation.interaction_count);
                    //     }
                    //     let average_interaction_count = interaction_counts.iter().sum::<u64>()
                    //         / interaction_counts.len() as u64;
                    //     let plot = OpinionPlot::new(vec![(
                    //         app.config.opinion_count,
                    //         average_interaction_count,
                    //     )]);
                    //     app.export.plots.push(plot);
                    // }

                    if ui.button("Export").clicked() {
                        // Calculate average of all plots.
                        let mut exports = vec![];
                        app.simulations.iter().for_each(|simulation| {
                            let mut simulation = simulation.lock().unwrap();
                            let sample_size = app.config.sample_size;
                            simulation
                                .export
                                .plots
                                .iter_mut()
                                .for_each(|plot| plot.j = sample_size);
                            exports.push(simulation.export.clone());
                        });
                        app.export.plots.push(SimulationExport::average(exports));
                        app.export.to_pdf();
                    }
                    Ok::<_, AppError>(())
                });
            });
        });
    });
    Ok(())
}

pub fn render_simulation_charts(ctx: &Context, app: &mut App) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let mut charts = vec![];
        let mut interaction_counts = vec![];
        for (index, simulation) in app.simulations.iter().enumerate() {
            let simulation = simulation.lock().unwrap();
            let chart = BarChart::new(
                simulation
                    .opinion_distribution
                    .map
                    .iter()
                    .map(|(x, y)| {
                        Bar::new(
                            *x as f64 + ((app.config.opinion_count * 2) as f64 * index as f64),
                            *y as f64,
                        )
                        .width(0.8_f64)
                    })
                    .collect(),
            )
            .name(app.formatter.format(simulation.interaction_count as f64));

            charts.push(chart);
            interaction_counts.push(simulation.interaction_count);
        }

        let average_interaction_count = if let Some(average_interaction_count) = interaction_counts
            .iter()
            .sum::<u64>()
            .checked_div(interaction_counts.len() as u64)
        {
            average_interaction_count
        } else {
            0
        };
        // Format the number into a human readable string.
        let human_number = app.formatter.format(average_interaction_count as f64);

        Plot::new("chart")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                for chart in charts {
                    plot_ui
                        .bar_chart(chart.name(format!("Average interactions: {}", human_number)));
                }
            });
    });
}
