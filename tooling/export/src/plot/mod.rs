use clap::ValueEnum;
use pgfplots::Picture;
use simulation::Simulation;

use self::{
    entropy::generate_entropy_plot, k::generate_k_plot, n::generate_n_plot,
    triangle::generate_triangle_plot,
};

mod entropy;
mod k;
mod n;
mod triangle;

pub struct Plot {
    pub plot_type: PlotType,
    pub simulations: Vec<Simulation>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum PlotType {
    Entropy,
    K,
    N,
    Triangle,
}

pub trait PictureGeneration {
    fn generate_picture(self) -> (Option<Picture>, Option<Picture>);
}

impl PictureGeneration for Plot {
    fn generate_picture(self) -> (Option<Picture>, Option<Picture>) {
        let mut population_simulations = vec![];
        let mut gossip_simulations = vec![];
        self.simulations
            .into_iter()
            .for_each(|simulation| match simulation.model {
                simulation::Model::Gossip => gossip_simulations.push(simulation),
                simulation::Model::Population => population_simulations.push(simulation),
            });

        match self.plot_type {
            PlotType::Entropy => (
                generate_entropy_plot(gossip_simulations),
                generate_entropy_plot(population_simulations),
            ),
            PlotType::K => (
                generate_k_plot(gossip_simulations),
                generate_k_plot(population_simulations),
            ),
            PlotType::N => (
                generate_n_plot(gossip_simulations),
                generate_n_plot(population_simulations),
            ),
            PlotType::Triangle => (
                generate_triangle_plot(gossip_simulations),
                generate_triangle_plot(population_simulations),
            ),
        }
    }
}
