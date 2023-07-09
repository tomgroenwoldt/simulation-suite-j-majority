use egui::{global_dark_light_mode_buttons, Layout, Ui};

use crate::{App, State};

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
                egui::Slider::new(&mut app.config.upper_bound_k, 2..=65535)
                    .text("Number of Opinions")
                    .logarithmic(true)
                    .trailing_fill(true),
            );
            ui.add(
                egui::Slider::new(&mut app.config.simulation_count, 1..=100)
                    .text("Number of Simulations")
                    .trailing_fill(true),
            );
        });
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            global_dark_light_mode_buttons(ui);
        });
    });
}
