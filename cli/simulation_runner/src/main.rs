use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use console::style;
use indicatif::HumanDuration;

use args::{get_simulation_config, Args};
use indicator::{create_progress_bar, CHECKMARK, FLOPPY_DISK, ROCKET, TOOLS};
use simulation::config::Config;
use simulation::Simulation;

mod args;
mod indicator;

fn main() -> Result<()> {
    let started = Instant::now();
    println!(
        "{} {} Parse arguments...",
        style("[1/4]").bold().dim(),
        TOOLS
    );
    let args = Args::parse();

    // Store finished simulations inside this vector
    let simulations = Arc::new(Mutex::new(vec![]));

    let total_n = args.total_n.unwrap_or(args.n);
    let total_k = args.total_k.unwrap_or(args.k);
    let total_j = args.total_j.unwrap_or(args.j);
    let simulation_batch_count = (((total_n - args.n) / args.n_step_size) + 1)
        * (((total_k - args.k) / args.k_step_size) + 1) as u64
        * (((total_j - args.j) / args.j_step_size) + 1) as u64;
    let progress_bar = create_progress_bar(simulation_batch_count)?;

    // Run all possible combinations for supplied n, k and j
    println!(
        "{} {} Run simulations...",
        style("[2/4]").bold().dim(),
        ROCKET
    );
    let mut n = args.n;
    while n <= total_n {
        let mut k = args.k;
        while k <= total_k {
            let mut j = args.j;
            while j <= total_j {
                let config = get_simulation_config(n, j, k, &args.initial_config)?;
                progress_bar.set_message(format!("n={n}, k={k}, j={j}"));
                run_simulations(config, &simulations, args.batch_size)?;
                progress_bar.inc(1);
                j += args.j_step_size;
            }
            k += args.k_step_size;
        }
        n += args.n_step_size;
    }

    println!(
        "{} {} Export data to {}...",
        style("[3/4]").bold().dim(),
        FLOPPY_DISK,
        style(format!("output/{}/simulation.json", args.output)).bold()
    );
    export_simulations(&args, &simulations)?;

    println!(
        "{} {} Ran {} simulations in {}",
        style("[4/4]").bold().dim(),
        CHECKMARK,
        simulation_batch_count * 20,
        HumanDuration(started.elapsed())
    );
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
