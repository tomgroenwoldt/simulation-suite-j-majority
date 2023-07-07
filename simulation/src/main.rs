extern crate human_format;

use config::Config;
use eframe::NativeOptions;
use error::AppError;
use export::{OpinionPlot, SimulationExport};
use human_format::Formatter;
use simulation::{FrontendSimulation, SimulationMessage};
use std::{
    sync::{mpsc::SyncSender, Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::sync::broadcast::{self, Sender};
use umya_spreadsheet::{new_file, writer};

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
    progress: f32,
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
            progress: 0.0,
        }
    }

    pub fn save(plots: &Vec<OpinionPlot>) -> Result<(), AppError> {
        let mut book = new_file();
        let _ = book.new_sheet("Sheet1");
        let file_name = if let Some(first_plot) = plots.first() {
            format!(
                "{}-simulations-with-{}-j-{}-k.csv",
                plots.len(),
                first_plot.j,
                first_plot.points.len() + 1
            )
        } else {
            panic!("Not a single plot was found, this is impossible!");
        };
        let path = std::path::Path::new(&file_name);

        // Write header.
        if let Some(plot) = plots.first() {
            for i in 0..plot.points.len() {
                book.get_sheet_by_name_mut("Sheet1")
                    .unwrap()
                    .get_cell_mut(((i + 1) as u32, 1))
                    .set_value((i + 2).to_string());
            }
        }

        for (i, plot) in plots.iter().enumerate() {
            for j in 0..plot.points.len() {
                book.get_sheet_by_name_mut("Sheet1")
                    .unwrap()
                    .get_cell_mut(((j + 1) as u32, (i + 3) as u32))
                    .set_value(&plot.points[j].1.to_string());
            }
        }
        writer::csv::write(&book, path, None).map_err(|_| AppError::ExportError)?;
        Ok(())
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
