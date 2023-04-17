use agent::Agent;
use clap::Parser;
use simulation::Simulation;

pub mod agent;
pub mod simulation;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    agent_count: u64,

    #[arg(short, long)]
    sample_size: u8,

    #[arg(short, long)]
    opinion_count: u8,
}

impl Config {
    pub fn validate(&self) {
        if self.sample_size as u64 > self.agent_count {
            panic!(
                "It is not possible to sample a greater number of agents than
                the total number of agents currently present."
            );
        }
    }
}

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Config,
    Simulation,
    Plotting,
    Exit,
}

fn main() {
    let config = Config::parse();
    config.validate();
    let simulation = Simulation::new(config);
    let mut agent = Agent::new(0);
    agent.update(&vec![]);
}
