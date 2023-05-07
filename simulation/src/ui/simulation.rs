use egui::{
    plot::{Bar, BarChart, Legend, Plot},
    Context,
};
use std::thread;

use crate::{
    simulation::{Simulation, SimulationMessage},
    App, State,
};

pub fn render_simulation_header(ctx: &Context, app: &mut App) {
    egui::TopBottomPanel::top("simulation_header").show(ctx, |ui| {
        ui.heading("Simulation");
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_enabled_ui(app.state.eq(&State::Config), |ui| {
                if ui.button("Start").clicked() {
                    app.state = State::Simulation;
                    // Execute the simulation on another thread.
                    for sender in &app.senders {
                        let sender = sender.clone();
                        let config = app.config.clone();
                        let receiver = app.broadcast.subscribe();
                        thread::spawn(move || {
                            let mut simulation = Simulation::new(config, sender);
                            simulation.execute(receiver).unwrap();
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
                    if ui.toggle_value(&mut app.paused, "Pause").clicked() {
                        match app.paused {
                            true => app.broadcast.send(SimulationMessage::Pause).unwrap(),
                            false => app.broadcast.send(SimulationMessage::Play).unwrap(),
                        };
                    }
                    if ui.button("Abort").clicked() {
                        app.broadcast.send(SimulationMessage::Abort).unwrap();
                        app.paused = false;
                    }
                });
                ui.add_enabled_ui(finished, |ui| {
                    if ui.button("Repeat").clicked() {
                        // Reset the simulations. This way we keep the communication with the
                        // simulation thread open.
                        for simulation in app.simulations.iter() {
                            app.broadcast.send(SimulationMessage::Finish).unwrap();
                            let mut simulation = simulation.lock().unwrap();
                            simulation.opinion_distribution.clear();
                            simulation.finished = false;
                        }
                        app.state = State::Config;
                    }
                });
            });
        });
    });
}

pub fn render_simulation_charts(ctx: &Context, app: &mut App) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let mut charts = vec![];
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
        }

        Plot::new("chart")
            .legend(Legend::default().background_alpha(1.0))
            .show(ui, |plot_ui| {
                for chart in charts {
                    plot_ui.bar_chart(chart);
                }
            });
    });
}
