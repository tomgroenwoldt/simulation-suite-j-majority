use config::Config;
use eframe::NativeOptions;

use error::AppError;
use simulation::{FrontendSimulation, SimulationMessage};
use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

pub mod agent;
pub mod config;
pub mod error;
pub mod simulation;
pub mod ui;

pub struct App {
    state: State,
    config: Config,
    simulations: Vec<Arc<Mutex<FrontendSimulation>>>,
    senders: Vec<Sender<SimulationMessage>>,
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

        // Create ten worker threads which are listening to possible simulations.
        for _ in 0..10 {
            let (sender, receiver) = mpsc::channel::<SimulationMessage>();

            let frontend_simulation = Arc::new(Mutex::new(FrontendSimulation::default()));
            let frontend_simulation_clone = Arc::clone(&frontend_simulation);
            let ctx_clone = cc.egui_ctx.clone();
            simulations.push(frontend_simulation);
            senders.push(sender);

            // Message handler which communicates with the simulation thread.
            thread::spawn(move || {
                for msg in &receiver {
                    let mut simulation = frontend_simulation_clone.lock().unwrap();
                    match msg {
                        SimulationMessage::Update((old, new, new_interaction_count)) => {
                            simulation.opinion_distribution.update(old, new);
                            simulation.interaction_count = new_interaction_count;
                            ctx_clone.request_repaint();
                        }
                        SimulationMessage::Finish => simulation.finished = true,
                    }
                }
            });
        }

        Self {
            state: State::Config,
            config: Config::default(),
            simulations,
            senders,
        }
    }
}

fn main() -> Result<(), AppError> {
    // Run GUI.
    eframe::run_native(
        "Simulation",
        NativeOptions::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )?;

    Ok(())
}
