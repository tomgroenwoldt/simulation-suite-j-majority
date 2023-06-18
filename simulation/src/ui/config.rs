use egui::{global_dark_light_mode_buttons, Layout, Ui};

use crate::{simulation::SimulationModel, App, State};

pub fn render_config(ui: &mut Ui, app: &mut App) {
    ui.heading("Configuration");
    ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
        ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
            ui.set_enabled(app.state.eq(&State::Config));
            ui.add_enabled(
                false,
                egui::Slider::new(&mut app.config.agent_count, 2..=100000000)
                    .text("Number of Agents")
                    .logarithmic(true)
                    .trailing_fill(true),
            );
            ui.add(
                egui::Slider::new(&mut app.config.sample_size, 3..=12)
                    .text("Sample Size")
                    .trailing_fill(true),
            );
            ui.add(
                egui::Slider::new(&mut app.config.opinion_count, 2..=50)
                    .text("Number of Opinions")
                    .trailing_fill(true),
            );
            ui.add(
                egui::Slider::new(&mut app.config.simulation_count, 1..=100)
                    .text("Number of Simulations")
                    .trailing_fill(true),
            );
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                ui.selectable_value(
                    &mut app.config.model,
                    SimulationModel::Population,
                    "Population model",
                );
                ui.selectable_value(
                    &mut app.config.model,
                    SimulationModel::Gossip,
                    "Gossip model",
                );
            });
        });
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            global_dark_light_mode_buttons(ui);
        });
    });
}

pub fn render_opinion_distribution_config(ui: &mut Ui, app: &mut App) {
    // ui.set_enabled(app.state.eq(&State::Config));
    ui.set_enabled(false);
    ui.heading("Opinion distribution");
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        for opinion in 0..app.config.opinion_count {
            ui.add(
                egui::Slider::new(app.config.weights.entry(opinion).or_insert(1), 0..=50)
                    .vertical()
                    .trailing_fill(true),
            );
        }
        // Remove old opinion distribution entries. Otherwise a decrease in opinion
        // count via the GUI would result in a program crash.
        for old in app.config.opinion_count..10 {
            app.config.weights.remove_entry(&old);
        }
    });
}
