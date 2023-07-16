use simulation::args::Args;
use simulation::Simulation;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    // Parse CLI arguments and initialize logger
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let mut simulations = vec![];

    // Run single simulation
    let mut simulation = Simulation::new(&args);
    simulation.execute();
    simulations.push(simulation);

    let path = format!("output/{}/simulation.json", &args.output);
    if Path::new(&path).exists() {
        let mut previous_simulations: Vec<Simulation> =
            serde_json::from_str(&read_to_string(&path)?)?;
        simulations.append(&mut previous_simulations);
    } else {
        let dir_path = format!("output/{}", &args.output);
        create_dir_all(dir_path)?;
    };
    let mut output = File::create(&path)?;
    let export = serde_json::to_string_pretty(&simulations)?;
    write!(output, "{}", export)?;

    Ok(())
}
