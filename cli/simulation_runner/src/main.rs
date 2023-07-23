use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;
use clap::Parser;

use args::{get_simulation_config, Args};
use simulation::config::Config;
use simulation::Simulation;

mod args;

fn main() -> Result<()> {
    let args = Args::parse();

    // Store finished simulations inside this vector
    let simulations = Arc::new(Mutex::new(vec![]));

    // Run all possible combinations for supplied n, k and j
    let mut n = args.n;
    while n <= args.total_n {
        let mut k = args.k;
        while k <= args.total_k {
            let mut j = args.j;
            while j <= args.total_j {
                let config = get_simulation_config(n, j, k, &args.initial_config)?;
                run_simulations(config, &simulations, args.batch_size)?;
                j += args.j_step_size;
            }
            k += args.k_step_size;
        }
        n += args.n_step_size;
    }

    export_simulations(&args, &simulations)?;

    Ok(())
}

/// # Run a batch of simulations in parallel
///
/// Creates a simulation config and executes a number of simulations with it
/// in parallel. Returns after all simulations finished and stores them in the
/// supplied vector.
fn run_simulations(
    config: Config,
    simulations: &Arc<Mutex<Vec<Simulation>>>,
    batch_size: usize,
) -> Result<()> {
    let mut handlers = vec![];
    let simulation = Simulation::new(config)?;

    // Run simulations in multiple threads
    for _ in 0..batch_size {
        let mut simulation = simulation.clone();
        let simulations = Arc::clone(simulations);
        let handler = thread::spawn(move || {
            simulation.execute();
            simulations.lock().unwrap().push(simulation);
        });
        handlers.push(handler);
    }

    // Wait for all simulations to finish
    handlers.into_iter().for_each(|handler| {
        handler.join().expect("Simulation thread panicked");
    });

    Ok(())
}

/// # Export simulations
///
/// Serializes finished simulations and stores them inside a JSON file. If the
/// file already exists, read previously ran simulations first.
fn export_simulations(args: &Args, simulations: &Arc<Mutex<Vec<Simulation>>>) -> Result<()> {
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
