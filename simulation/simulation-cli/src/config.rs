use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..))]
    pub agent_count: u64,

    #[arg(short, long)]
    pub sample_size: u8,

    #[arg(short, long)]
    pub opinion_count: u8,
}

impl Config {
    pub fn validate(&self) {
        if self.sample_size as u64 > self.agent_count {
            panic!("It is not possible to sample a greater number of agents than the total number of agents currently present.");
        }
    }
}
