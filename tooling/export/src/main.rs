use std::{
    fs::{read_to_string, File},
    io::Write,
    time::Instant,
};

use anyhow::{anyhow, Result};
use clap::Parser;

use common::{CHECKMARK, FACTORY, FOLDER, GRAPH, TOOLS};
use console::style;
use indicatif::HumanDuration;
use pgfplots::Engine;

use args::Args;
use plot::{PictureGeneration, Plot};
use simulation::Simulation;

mod args;
mod plot;
mod util;

fn main() -> Result<()> {
    let started = Instant::now();
    println!(
        "{} {} Parse arguments...",
        console::style("[1/5]").bold().dim(),
        TOOLS
    );
    let args = Args::parse();

    // Extract simulations from input file
    println!(
        "{} {} Extract simulation data...",
        console::style("[2/5]").bold().dim(),
        FACTORY
    );
    let input_file_content = &read_to_string(&args.input)?;
    let simulations: Vec<Simulation> = serde_json::from_str(input_file_content)?;

    // Generate plot
    println!(
        "{} {} Generate plot...",
        console::style("[3/5]").bold().dim(),
        GRAPH
    );
    let plot = Plot {
        plot_type: args.plot_type,
        simulations,
    };
    let (gossip_picture, population_picture) = plot.generate_picture();

    if args.generate_latex {
        if let Some(picture) = &population_picture {
            let mut plot = String::new();
            let mut file = File::create("population.tex")?;
            plot.push_str(&picture.standalone_string());
            file.write_all(plot.as_bytes())?;
        }
        if let Some(picture) = &gossip_picture {
            let mut plot = String::new();
            let mut file = File::create("gossip.tex")?;
            plot.push_str(&picture.standalone_string());
            file.write_all(plot.as_bytes())?;
        }
    }

    // Open plot in default PDF viewer
    println!(
        "{} {} Open plot in default PDF viewer...",
        console::style("[4/5]").bold().dim(),
        FOLDER
    );
    if let Some(picture) = population_picture {
        picture
            .show_pdf(Engine::Tectonic)
            .map_err(|e| anyhow!(e.to_string()))?;
    }
    if let Some(picture) = gossip_picture {
        picture
            .show_pdf(Engine::Tectonic)
            .map_err(|e| anyhow!(e.to_string()))?;
    }

    println!(
        "{} {} Generated and opened plot in {}",
        style("[5/5]").bold().dim(),
        CHECKMARK,
        HumanDuration(started.elapsed())
    );
    Ok(())
}
