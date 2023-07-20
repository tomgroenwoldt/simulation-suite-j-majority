use args::Args;
use simulation::Simulation;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use clap::Parser;

mod args;

fn main() -> Result<()> {
    // Parse CLI arguments and initialize logger
    let args = Args::parse();
    let output_folder = args.output.clone();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let mut simulations = vec![];

    // Run single simulation
    let simulation_config = args.info_simulation_config()?;
    let mut simulation = Simulation::new(simulation_config)?;
    simulation.execute();
    simulations.push(simulation);

    // Create the file if it does not exist yet
    let path = format!("output/{}/simulation.json", output_folder);
    if Path::new(&path).exists() {
        let mut previous_simulations: Vec<Simulation> =
            serde_json::from_str(&read_to_string(&path)?)?;
        simulations.append(&mut previous_simulations);
    } else {
        let dir_path = format!("output/{}", output_folder);
        create_dir_all(dir_path)?;
    };
    let mut file = File::create(&path)?;
    let export = serde_json::to_string_pretty(&simulations)?;

    file.write_all(export.as_bytes())?;

    Ok(())
}
