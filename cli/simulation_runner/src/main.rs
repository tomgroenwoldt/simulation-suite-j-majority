use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;
use clap::Parser;

use args::Args;
use simulation::Simulation;

mod args;

fn main() -> Result<()> {
    // Parse CLI arguments and initialize logger
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let simulations = Arc::new(Mutex::new(vec![]));
    let mut handlers = vec![];

    // Run simulations in parallel
    let simulation_config = args.to_simulation_config()?;
    let simulation = Simulation::new(simulation_config)?;
    for _ in 0..args.simulations {
        let mut simulation = simulation.clone();
        let simulations = Arc::clone(&simulations);
        let handler = thread::spawn(move || {
            simulation.execute();
            simulations.lock().unwrap().push(simulation);
        });
        handlers.push(handler);
    }

    // Wait for all simulations to finish
    handlers
        .into_iter()
        .for_each(|handler| handler.join().expect("Simulation thread panicked"));

    let mut simulations = simulations.lock().unwrap();

    let path = format!("output/{}/simulation.json", args.output);
    // If a file already exists, read previous simulations
    if Path::new(&path).exists() {
        let mut previous_simulations: Vec<Simulation> =
            serde_json::from_str(&read_to_string(&path)?)?;
        simulations.append(&mut previous_simulations);
    } else {
        let dir_path = format!("output/{}", args.output);
        create_dir_all(dir_path)?;
    };

    // Create a fresh file and store previous and new simulations in JSON format
    let mut file = File::create(&path)?;
    let export = serde_json::to_string_pretty(&simulations.clone())?;
    file.write_all(export.as_bytes())?;

    Ok(())
}
