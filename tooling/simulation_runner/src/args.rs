use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_verbosity_flag::Verbosity;
use simulation::{config::Config, Model};

#[derive(Clone, Parser)]
pub struct Args {
    /// Number of agents
    #[arg(short, long, default_value_t = 100000)]
    pub n: u64,
    /// Upper treshold for n
    ///
    /// Set to simulate for all [n, total_n]
    #[arg(long)]
    pub total_n: Option<u64>,
    #[arg(long, default_value_t = 1000)]
    pub n_step_size: u64,
    /// Sample size
    #[arg(short, long, default_value_t = 3)]
    pub j: u8,
    /// Upper treshold for j
    ///
    /// Set to simulate for all [j, total_j]
    #[arg(long)]
    pub total_j: Option<u8>,
    #[arg(long, default_value_t = 1)]
    pub j_step_size: u8,
    /// Number of opinions
    #[arg(short, long, default_value_t = 2)]
    pub k: u16,
    /// Upper treshold for k
    ///
    /// Set to simulate for all [k, total_k]
    #[arg(long)]
    pub total_k: Option<u16>,
    #[arg(long, default_value_t = 1)]
    pub k_step_size: u16,
    /// Initial consensus configuration
    #[arg(long, use_value_delimiter = true)]
    pub initial_config: Option<Vec<u64>>,
    /// Number of simulations to run
    #[arg(long, default_value_t = 10)]
    pub batch_size: usize,
    #[arg(long)]
    pub model: Model,
    /// Folder to store files
    #[arg(short, long)]
    pub output: String,
    /// Enables or disables verbose output
    #[command(flatten)]
    pub verbose: Verbosity,
}

/// # Get simulation config
///
/// Converts parameters into a valid config for the simulation.
pub fn get_simulation_config(
    n: u64,
    j: u8,
    k: u16,
    initial_config: &Option<Vec<u64>>,
    model: Model,
) -> Result<Config> {
    let config = validate_initial_config(initial_config, n, k)?;
    Ok(Config {
        n,
        j,
        k,
        config,
        model,
    })
}

/// # Validate initial config
///
/// Validates the user supplied initial config and responds with fitting error
/// messages. If there is no initial config this function provides one.
fn validate_initial_config(initial_config: &Option<Vec<u64>>, n: u64, k: u16) -> Result<Vec<u64>> {
    if let Some(initial_config) = initial_config {
        let mut cmd = Args::command();
        if !initial_config.len().eq(&(k as usize)) {
            cmd.error(
                clap::error::ErrorKind::TooFewValues,
                "Initial initial_configuration should have k elements",
            )
            .exit();
        }
        if initial_config.iter().sum::<u64>().eq(&n) {
            return Ok(initial_config.clone());
        } else {
            cmd.error(
                clap::error::ErrorKind::ValueValidation,
                "Initial configuration should sum up to n",
            )
            .exit();
        }
    }

    // If no initial config was supplied, create an evenly spreaded one
    let initial_value = n / k as u64;
    let remainder = n % k as u64;

    let mut initial_config = vec![initial_value; k as usize];
    for item in initial_config.iter_mut().take(remainder as usize) {
        *item += 1;
    }

    Ok(initial_config)
}
