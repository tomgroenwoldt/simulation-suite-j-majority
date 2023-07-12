use std::{
    sync::{mpsc::SyncSender, Arc, Mutex},
    thread,
    time::Duration,
};

use eframe::NativeOptions;
use entropy::Entropy;
use error::AppError;
use export::{OpinionPlot, SimulationExport};
use schema::{
    config::Config,
    simulation::{FrontendSimulation, SimulationMessage},
};
use tokio::sync::broadcast::{self, Sender};
use umya_spreadsheet::{new_file, writer};

mod agent;
mod entropy;
mod error;
mod export;
mod opinion_distribution;
mod schema;
mod simulation;
mod ui;

pub struct App {
    state: State,
    config: Config,
    simulations: Vec<Arc<Mutex<FrontendSimulation>>>,
    senders: Vec<SyncSender<SimulationMessage>>,
    broadcast: Sender<SimulationMessage>,
    paused: bool,
    entropies: Vec<Entropy>,
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

        Self {
            state: State::Config,
            config: Config::default(),
            simulations: vec![],
            senders: vec![],
            broadcast,
            paused: false,
            entropies: vec![],
            export: SimulationExport::default(),
            progress: 0.0,
        }
    }

    pub fn save(plots: &Vec<OpinionPlot>, entropies: &[Entropy]) -> Result<(), AppError> {
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

        let mut current_row = 1;
        // Write plot header.
        if let Some(plot) = plots.first() {
            for i in 0..plot.points.len() {
                book.get_sheet_by_name_mut("Sheet1")
                    .unwrap()
                    .get_cell_mut(((i + 1) as u32, current_row))
                    .set_value((i + 2).to_string());
            }
            current_row += 2;
        }
        for plot in plots.iter() {
            for j in 0..plot.points.len() {
                book.get_sheet_by_name_mut("Sheet1")
                    .unwrap()
                    .get_cell_mut(((j + 1) as u32, current_row))
                    .set_value(&plot.points[j].1.to_string());
            }
            current_row += 1;
        }

        current_row += 2;
        // Write entropy header.
        if let Some(entropy) = entropies.first() {
            let mut keys = entropy.map.keys().collect::<Vec<_>>();
            keys.sort();
            for (i, key) in keys.iter().enumerate() {
                book.get_sheet_by_name_mut("Sheet1")
                    .unwrap()
                    .get_cell_mut(((i + 1) as u32, current_row))
                    .set_value(key.to_string());
            }
            current_row += 2;
        }
        for entropy in entropies.iter() {
            let mut points = entropy.map.iter().collect::<Vec<_>>();
            points.sort_by(|point_one, point_two| point_one.0.cmp(point_two.0));
            for (j, point) in points.iter().enumerate() {
                book.get_sheet_by_name_mut("Sheet1")
                    .unwrap()
                    .get_cell_mut(((j + 1) as u32, current_row))
                    .set_value(point.1.to_string());
            }
            current_row += 1;
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
