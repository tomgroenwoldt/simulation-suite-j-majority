extern crate human_format;

use config::Config;
use eframe::NativeOptions;
use error::AppError;
use human_format::Formatter;
use simulation::{FrontendSimulation, SimulationMessage};
use std::{
    sync::{
        mpsc::{self, SyncSender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use tokio::sync::broadcast::{self, Sender};

pub mod agent;
pub mod config;
pub mod error;
pub mod simulation;
pub mod ui;

pub struct App {
    state: State,
    config: Config,
    simulations: Vec<Arc<Mutex<FrontendSimulation>>>,
    senders: Vec<SyncSender<SimulationMessage>>,
    broadcast: Sender<SimulationMessage>,
    paused: bool,
    formatter: Formatter,
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
        let mut simulations = vec![];
        let mut senders = vec![];
        let update_interval = Duration::from_millis(10);

        let ctx_clone = cc.egui_ctx.clone();
        // Helper thread which periodically requests a repaint. This is more
        // efficient than repainting on every simulation update.
        thread::spawn(move || loop {
            ctx_clone.request_repaint_after(update_interval);
            thread::sleep(update_interval / 5);
        });

        // Create ten worker threads which are listening to possible simulations.
        for _ in 0..10 {
            let (sender, receiver) = mpsc::sync_channel::<SimulationMessage>(1000);

            let frontend_simulation = Arc::new(Mutex::new(FrontendSimulation::default()));
            let frontend_simulation_clone = Arc::clone(&frontend_simulation);
            simulations.push(frontend_simulation);
            senders.push(sender);

            // Message handler which communicates with the simulation thread.
            thread::spawn(move || loop {
                if let Ok(msg) = receiver.recv() {
                    let mut simulation = frontend_simulation_clone.lock().unwrap();
                    match msg {
                        SimulationMessage::Update((old, new, new_interaction_count)) => {
                            simulation.opinion_distribution.update(old, new);
                            simulation.interaction_count = new_interaction_count;
                        }
                        SimulationMessage::Finish => simulation.finished = true,
                        _ => {}
                    }
                }
            });
        }

        let (broadcast, _) = broadcast::channel(1000);
        let formatter = Formatter::new();

        Self {
            state: State::Config,
            config: Config::default(),
            simulations,
            senders,
            broadcast,
            paused: false,
            formatter,
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
