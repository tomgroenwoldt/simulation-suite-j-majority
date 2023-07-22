use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;

use pgfplots::Engine;

use args::Args;
use plot::{PictureGeneration, Plot};
use simulation::Simulation;

mod args;
mod plot;
mod util;

fn main() -> Result<()> {
    // Parse CLI arguments and initialize logger
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    // Extract simulations from input file
    let input_file_content = &read_to_string(&args.input)?;
    let simulations: Vec<Simulation> = serde_json::from_str(input_file_content)?;

    // Generate pgfplot
    let plot = Plot {
        plot_type: args.plot_type,
        simulations,
    };
    let picture = plot.generate_picture();
    println!("{}", picture.standalone_string());
    picture.show_pdf(Engine::PdfLatex)?;

    Ok(())
}
