use egui::{
    plot::{Bar, BarChart, Plot},
    Context, ProgressBar,
};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use tracing::error;

use crate::{
    error::AppError,
    export::SimulationExport,
    schema::{
        opinion_distribution::OpinionDistribution,
        simulation::{FrontendSimulation, Simulation, SimulationMessage},
    },
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
                        thread::spawn(move || loop {
                            if let Ok(msg) = receiver.recv() {
                                let mut simulation = frontend_simulation_clone.lock().unwrap();
                                match msg {
                                    SimulationMessage::Update(opinion_distribution) => {
                                        simulation.opinion_distribution = opinion_distribution;
                                    }
                                    SimulationMessage::Next => {
                                        simulation.opinion_distribution =
                                            OpinionDistribution::default();
                                        simulation.current_opinion += 1;
                                    }
                                    SimulationMessage::Finish(plot, entropy) => {
                                        simulation.plot = plot;
                                        simulation.entropy = entropy;
                                        simulation.finished = true;
                                        return;
                                    }
                                    _ => {}
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
                            if let Ok(mut simulation) = Simulation::new(config, sender) {
                                if let Err(e) = simulation.execute(receiver) {
                                    error!("Error while executing simulation: {e}");
                                }
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
                    if ui.button("Run new simulation").clicked() {
                        app.simulations.clear();
                        app.entropies = vec![];
                        app.state = State::Config;
                    }
                    if ui.button("Export").clicked() {
                        let mut plots = vec![];
                        let mut entropies = vec![];
                        for simulation in app.simulations.iter() {
                            let simulation = simulation.lock().unwrap();
                            plots.push(simulation.plot.clone());
                            entropies.push(simulation.entropy.clone());
                        }
                        // Save all single plots before averaging them.
                        App::save(&plots, &entropies)?;
                        // Calculate average of all plots and conclude into one.
                        let (average_plot, average_entropy) =
                            SimulationExport::average(plots, entropies);
                        app.export.plots = app
                            .export
                            .plots
                            .iter()
                            .cloned()
                            .filter(|plot| plot.j != average_plot.j)
                            .collect::<Vec<_>>();
                        app.export.plots.push(average_plot);
                        app.export.entropies = app
                            .export
                            .entropies
                            .iter()
                            .cloned()
                            .filter(|entropy| entropy.sample_size != average_entropy.sample_size)
                            .collect::<Vec<_>>();
                        app.export.entropies.push(average_entropy);
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

pub fn render_simulation_data(ctx: &Context, app: &mut App) {
    let mut charts = vec![];
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
                        *x as f64 + ((app.config.upper_bound_k * 2) as f64 * index as f64),
                        *y as f64,
                    )
                    .width(0.8_f64)
                })
                .collect(),
        );

        charts.push(chart);
        current_opinions.push(simulation.opinion_distribution.progress);
    }
    app.progress = current_opinions.iter().sum::<f32>() / current_opinions.len() as f32;

    render_simulation_charts(ctx, app, charts);
}

pub fn render_simulation_charts(ctx: &Context, _app: &mut App, charts: Vec<BarChart>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        Plot::new("charts").show(ui, |plot_ui| {
            for chart in charts {
                plot_ui.bar_chart(chart);
            }
        });
    });
}
