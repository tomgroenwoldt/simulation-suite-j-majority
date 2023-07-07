use egui::{
    plot::{Bar, BarChart, Legend, Plot},
    Context, ProgressBar,
};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use tracing::error;

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
                                                simulation.opinion_distribution.map.len() as u16;
                                            let interaction_count = simulation.interaction_count;
                                            plot.points.push((opinion_count, interaction_count));
                                            simulation.opinion_distribution =
                                                OpinionDistribution::default();
                                            simulation.interaction_count = 0;
                                            simulation.current_opinion += 1;
                                        }
                                        SimulationMessage::Finish => {
                                            simulation.plot = plot;
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
                if ui.button("Clear export").clicked() {
                    app.export = SimulationExport::default();
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
                    if ui.button("Run new simulation").clicked() {
                        app.broadcast.send(SimulationMessage::Finish)?;
                        app.simulations.clear();
                        app.state = State::Config;
                    }
                    if ui.button("Export").clicked() {
                        let mut exports = vec![];
                        for simulation in app.simulations.iter() {
                            let mut simulation = simulation.lock().unwrap();
                            let sample_size = app.config.sample_size;
                            simulation.plot.j = sample_size;
                            exports.push(simulation.plot.clone());
                        }
                        // Save all single plots before averaging them.
                        App::save(&exports)?;
                        // Calculate average of all plots and conclude into one.
                        app.export.plots.push(SimulationExport::average(exports));
                        app.export.generate_pdf();
                    }
                    ui.add(ProgressBar::new(app.progress).show_percentage());
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
        let mut current_opinions = vec![];
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
            current_opinions.push(simulation.current_opinion + 1);
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
        let progress = current_opinions
            .iter()
            .sum::<u16>()
            .checked_div(current_opinions.len() as u16);
        if let Some(progress) = progress {
            app.progress = progress as f32 / app.config.opinion_count as f32;
        }
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
