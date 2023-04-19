use clap::Parser;
use config::Config;
use error::AppError;
use simulation::Simulation;

pub mod agent;
pub mod config;
pub mod error;
pub mod simulation;

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Config,
    Simulation,
    Plotting,
    Exit,
}

fn main() -> Result<(), AppError> {
    let config = Config::parse();
    config.validate();

    let mut simulation = Simulation::new(config);
    simulation.execute()?;

    Ok(())
}

#[cfg(test)]
mod main {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[should_panic]
    fn panic_on_sample_size_greater_than_agent_count() {
        let config = Config {
            agent_count: 10,
            sample_size: 11,
            opinion_count: 5,
        };
        config.validate();
    }
}
