use egui::{
    plot::{Bar, BarChart, Plot},
    Color32,
};
use std::thread;

use crate::{simulation::Simulation, App, State};

/// GUI implementation via egui and eframe.
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("main").show(ctx, |_| {
            egui::TopBottomPanel::top("general_configuration").show(ctx, |ui| {
                ui.set_enabled(self.state.eq(&State::Config));
                ui.heading("Configuration");
                ui.add(
                    egui::Slider::new(&mut self.config.agent_count, 2..=100000000)
                        .text("Number of Agents")
                        .logarithmic(true),
                );
                ui.add(
                    egui::Slider::new(&mut self.config.sample_size, 2..=255).text("Sample Size"),
                );
                ui.add(
                    egui::Slider::new(&mut self.config.opinion_count, 2..=10)
                        .text("Number of Opinions"),
                );
                if ui.button("Start simulation").clicked() {
                    self.state = State::Simulation;

                    // Execute the simulation on another thread.
                    for sender in &self.senders {
                        let sender = sender.clone();
                        let config = self.config.clone();
                        thread::spawn(move || {
                            let mut simulation = Simulation::new(config, sender);
                            simulation.execute().unwrap();
                        });
                    }
                }
            });
            egui::TopBottomPanel::bottom("opinion_distribution_configuration").show(ctx, |ui| {
                ui.set_enabled(self.state.eq(&State::Config));
                ui.heading("Opinion distribution");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    for opinion in 0..self.config.opinion_count {
                        ui.add(
                            egui::Slider::new(
                                self.config.weights.entry(opinion).or_insert(1),
                                0..=5,
                            )
                            .vertical(),
                        );
                    }
                    // Remove old opinion distribution entries. Otherwise a decrease in opinion
                    // count via the GUI would result in a program crash.
                    for old in self.config.opinion_count..10 {
                        self.config.weights.remove_entry(&old);
                    }
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_enabled(self.state.eq(&State::Simulation));
                egui::TopBottomPanel::top("simulation_header").show(ctx, |ui| {
                    ui.heading("Simulation");
                    let finished = self
                        .simulations
                        .iter()
                        .all(|sim| sim.lock().unwrap().finished);
                    ui.set_enabled(finished);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        // ui.label(format!(
                        //     "Number of interactions: {}",
                        //     simulation.interaction_count
                        // ));
                        if ui
                            .button("Simulations finished. Click to simulate again!")
                            .clicked()
                        {
                            // Reset the simulations. This way we keep the communication with the
                            // simulation thread open.
                            for simulation in self.simulations.iter() {
                                let mut simulation = simulation.lock().unwrap();
                                simulation.opinion_distribution.clear();
                                simulation.finished = false;
                            }
                            self.state = State::Config;
                        }
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut charts = vec![];
                    for (index, simulation) in self.simulations.iter().enumerate() {
                        let simulation = simulation.lock().unwrap();
                        let chart = BarChart::new(
                            simulation
                                .opinion_distribution
                                .map
                                .iter()
                                .map(|(x, y)| {
                                    Bar::new(
                                        *x as f64
                                            + ((self.config.opinion_count * 2) as f64
                                                * index as f64),
                                        *y as f64,
                                    )
                                    .width(0.8_f64)
                                })
                                .collect(),
                        )
                        .color(Color32::WHITE);
                        charts.push(chart);
                    }

                    Plot::new("chart").show(ui, |plot_ui| {
                        for chart in charts {
                            plot_ui.bar_chart(chart);
                        }
                    });
                });
            });
        });
    }
}
