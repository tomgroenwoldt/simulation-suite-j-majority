use anyhow::Result;
use plot::OpinionPlotWithIncreasingSampleSize;
use simulation::Simulation;
use std::fs::read_to_string;

use args::Args;
use clap::Parser;

mod args;
mod plot;

fn main() -> Result<()> {
    // Parse CLI arguments and initialize logger
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let simulations: Vec<Simulation> = serde_json::from_str(&read_to_string(&args.input)?)?;

    let opinion_plot = OpinionPlotWithIncreasingSampleSize { simulations };
    opinion_plot.generate_pdf();

    Ok(())
}
