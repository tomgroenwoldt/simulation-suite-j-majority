use self::{
    config::render_config,
    simulation::{render_simulation_charts, render_simulation_header},
};
use crate::{error::AppError, App};

pub mod config;
pub mod simulation;

/// GUI implementation via egui and eframe.
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("main").show(ctx, |_| {
            egui::TopBottomPanel::top("general_configuration").show(ctx, |ui| {
                render_config(ui, self);
            });
            // egui::TopBottomPanel::top("opinion_distribution_configuration").show(ctx, |ui| {
            //     render_opinion_distribution_config(ui, self);
            // });
            egui::CentralPanel::default().show(ctx, |_ui| {
                render_simulation_header(ctx, self)?;
                render_simulation_charts(ctx, self);
                Ok::<(), AppError>(())
            });
        });
    }
}
