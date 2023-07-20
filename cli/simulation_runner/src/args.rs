use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_verbosity_flag::Verbosity;
use simulation::config::Config;

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
    #[arg(short, long, use_value_delimiter = true)]
    pub config: Option<Vec<u64>>,
    /// Enables or disables verbose output
    #[command(flatten)]
    pub verbose: Verbosity,
    /// Folder to store files
    #[arg(short, long)]
    pub output: String,
}

impl Args {
    pub fn info_simulation_config(self) -> Result<Config> {
        let config = validate_config(self.config, self.n, self.k)?;
        Ok(Config {
            n: self.n,
            j: self.j,
            k: self.k,
            config,
        })
    }
}

fn validate_config(config: Option<Vec<u64>>, n: u64, k: u16) -> Result<Vec<u64>> {
    if let Some(config) = config {
        let mut cmd = Args::command();
        if !config.len().eq(&(k as usize)) {
            cmd.error(
                clap::error::ErrorKind::TooFewValues,
                "Initial configuration should have k elements",
            )
            .exit();
        }
        if config.iter().sum::<u64>().eq(&n) {
            return Ok(config);
        } else {
            cmd.error(
                clap::error::ErrorKind::ValueValidation,
                "Initial configuration should sum up to n",
            )
            .exit();
        }
    }
    let initial_value = n / k as u64;
    let remainder = n % k as u64;

    let mut config = vec![initial_value; k as usize];
    for item in config.iter_mut().take(remainder as usize) {
        *item += 1;
    }

    Ok(config)
}
