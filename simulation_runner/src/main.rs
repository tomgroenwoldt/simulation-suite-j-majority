use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use common::{create_progress_bar, CHECKMARK, FLOPPY_DISK, ROCKET, TOOLS};
use console::style;
use indicatif::HumanDuration;

use args::{get_simulation_config, Args};
use itertools::Itertools;
use simulation::config::Config;
use simulation::Simulation;

mod args;

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
                let config =
                    get_simulation_config(n, j, k, &args.initial_config, args.model.clone())?;
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
        simulation_batch_count * args.batch_size as u64,
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
    let mut all_simulations = vec![];
    let path = format!("output/{}/simulation.json", args.output);

    // If a file already exists, read previous simulations
    if Path::new(&path).exists() {
        all_simulations = serde_json::from_str(&read_to_string(&path)?)?;
    } else {
        let dir_path = format!("output/{}", args.output);
        create_dir_all(dir_path)?;
    };
    all_simulations.append(&mut simulations);

    let averaged_simulations = average_simulations(all_simulations);

    // Create a fresh file and store previous and new simulations in JSON format
    let mut file = File::create(&path)?;
    let export = serde_json::to_string_pretty(&averaged_simulations)?;
    file.write_all(export.as_bytes())?;

    Ok(())
}

/// # Average simulations
///
/// Groups all simulations by configuration and averages the interaction count as well
/// as the entropy. Simulations with the same configuration are summarized into one
/// Simulation struct.
fn average_simulations(simulations: Vec<Simulation>) -> Vec<Simulation> {
    // Group simulations by PartialEq of Simulation
    let mut grouped_simulations = simulations
        .into_iter()
        .group_by(|simulation| simulation.clone())
        .into_iter()
        .map(|(simulation, group)| {
            (
                simulation,
                group.collect::<Vec<_>>().into_iter().collect_vec(),
            )
        })
        .collect_vec();

    grouped_simulations
        .iter_mut()
        .for_each(|(simulation, group)| {
            // Calculate the average interaction count of the group
            simulation.interaction_count = group
                .iter_mut()
                .map(|simulation| simulation.interaction_count)
                .sum::<u64>()
                / group.len() as u64;

            // Calculate the average entropy of the group
            let mut averaged_entropy = HashMap::new();
            simulation
                .entropy
                .iter()
                .for_each(|(interaction, entropy)| {
                    averaged_entropy
                        .entry(*interaction)
                        .and_modify(|v| *v += entropy)
                        .or_insert(*entropy);
                });

            averaged_entropy.iter_mut().for_each(|(_, v)| {
                *v /= group.len() as f64;
            });

            simulation.entropy = averaged_entropy
                .into_iter()
                .map(|(key, value)| (key, value))
                .collect_vec();
        });

    grouped_simulations
        .into_iter()
        .map(|(simulation, _)| simulation)
        .collect_vec()
}
