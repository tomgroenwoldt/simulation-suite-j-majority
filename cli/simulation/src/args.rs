use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
pub struct Args {
    /// Number of agents
    #[arg(short, long, default_value_t = 100000)]
    pub n: u64,
    /// Sample size
    #[arg(short, long, default_value_t = 5)]
    pub j: u8,
    /// Number of opinions
    #[arg(short, long, default_value_t = 10)]
    pub k: u16,
    /// Initial consensus configuration
    ///
    /// Either adds up to provided n with k provided values or takes k-1 values
    /// and shifts remaining n to last element
    #[arg(short, long, default_values_t = [5, 5, 5].to_vec())]
    pub config: Vec<u64>,
    /// Enables or disables verbose output
    #[command(flatten)]
    pub verbose: Verbosity,
    /// Folder to store files
    #[arg(short, long)]
    pub output: String,
}
