extern crate human_format;

use config::Config;
use eframe::NativeOptions;
use error::AppError;
use export::SimulationExport;
use human_format::Formatter;
use simulation::{FrontendSimulation, SimulationMessage};
use std::{
    sync::{mpsc::SyncSender, Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::sync::broadcast::{self, Sender};

mod agent;
mod config;
mod error;
mod export;
mod simulation;
mod ui;

pub struct App {
    state: State,
    config: Config,
    simulations: Vec<Arc<Mutex<FrontendSimulation>>>,
    senders: Vec<SyncSender<SimulationMessage>>,
    broadcast: Sender<SimulationMessage>,
    paused: bool,
    formatter: Formatter,
    export: SimulationExport,
}

#[derive(Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Config,
    Simulation,
    Plotting,
    Exit,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let update_interval = Duration::from_millis(10);

        let ctx_clone = cc.egui_ctx.clone();
        // Helper thread which periodically requests a repaint. This is more
        // efficient than repainting on every simulation update.
        thread::spawn(move || loop {
            ctx_clone.request_repaint();
            thread::sleep(update_interval);
        });

        let (broadcast, _) = broadcast::channel(1000);
        let formatter = Formatter::new();

        Self {
            state: State::Config,
            config: Config::default(),
            simulations: vec![],
            senders: vec![],
            broadcast,
            paused: false,
            formatter,
            export: SimulationExport::default(),
        }
    }
}

fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();
    // Run GUI.
    eframe::run_native(
        "Simulation",
        NativeOptions::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )?;

    Ok(())
}
