use egui::Ui;

use crate::{App, State};

pub fn render_config(ui: &mut Ui, app: &mut App) {
    ui.set_enabled(app.state.eq(&State::Config));
    ui.heading("Configuration");
    ui.add(
        egui::Slider::new(&mut app.config.agent_count, 2..=100000000)
            .text("Number of Agents")
            .logarithmic(true)
            .trailing_fill(true),
    );
    ui.add(
        egui::Slider::new(&mut app.config.sample_size, 2..=255)
            .text("Sample Size")
            .trailing_fill(true),
    );
    ui.add(
        egui::Slider::new(&mut app.config.opinion_count, 2..=10)
            .text("Number of Opinions")
            .trailing_fill(true),
    );
}

pub fn render_opinion_distribution_config(ui: &mut Ui, app: &mut App) {
    ui.set_enabled(app.state.eq(&State::Config));
    ui.heading("Opinion distribution");
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        for opinion in 0..app.config.opinion_count {
            ui.add(
                egui::Slider::new(app.config.weights.entry(opinion).or_insert(1), 0..=5)
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
