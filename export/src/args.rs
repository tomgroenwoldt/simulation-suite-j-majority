use std::fs::File;

use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::Verbosity;

use crate::plot::PlotType;

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the input file
    #[arg(short, long, value_parser = file_exists)]
    pub input: String,
    /// Enables or disables verbose output
    #[command(flatten)]
    pub verbose: Verbosity,
    #[arg(value_enum)]
    pub plot_type: PlotType,
    #[arg(long, default_value_t = false)]
    pub generate_latex: bool,
    #[arg(long, default_value_t = false)]
    pub error_bars: bool,
}

fn file_exists(s: &str) -> Result<String> {
    File::open(s)?;
    Ok(s.to_string())
}
