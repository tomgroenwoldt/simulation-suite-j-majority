use clap::ValueEnum;
use pgfplots::Picture;
use simulation::Simulation;

use self::{
    j::generate_j_plot, k::generate_k_plot, n::generate_n_plot, triangle::generate_triangle_plot,
};

mod j;
mod k;
mod n;
mod triangle;

pub struct Plot {
    pub plot_type: PlotType,
    pub simulations: Vec<Simulation>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum PlotType {
    // EntropyOverJ,
    // EntropyOverK,
    // EntropyOverN,
    J,
    K,
    N,
    Triangle,
}

pub trait PictureGeneration {
    fn generate_picture(self, error_bars: bool) -> (Option<Picture>, Option<Picture>);
}

impl PictureGeneration for Plot {
    fn generate_picture(self, error_bars: bool) -> (Option<Picture>, Option<Picture>) {
        let mut population_simulations = vec![];
        let mut gossip_simulations = vec![];
        self.simulations
            .into_iter()
            .for_each(|simulation| match simulation.model {
                simulation::Model::Gossip => gossip_simulations.push(simulation),
                simulation::Model::Population => population_simulations.push(simulation),
            });

        match self.plot_type {
            // PlotType::EntropyOverJ => (
            //     generate_entropy_j_plot(gossip_simulations, error_bars),
            //     generate_entropy_j_plot(population_simulations, error_bars),
            // ),
            // PlotType::EntropyOverK => (
            //     generate_entropy_k_plot(gossip_simulations, error_bars),
            //     generate_entropy_k_plot(population_simulations, error_bars),
            // ),
            // PlotType::EntropyOverN => (
            //     generate_entropy_n_plot(gossip_simulations, error_bars),
            //     generate_entropy_n_plot(population_simulations, error_bars),
            // ),
            PlotType::J => (
                generate_j_plot(gossip_simulations, error_bars),
                generate_j_plot(population_simulations, error_bars),
            ),
            PlotType::K => (
                generate_k_plot(gossip_simulations, error_bars),
                generate_k_plot(population_simulations, error_bars),
            ),
            PlotType::N => (
                generate_n_plot(gossip_simulations, error_bars),
                generate_n_plot(population_simulations, error_bars),
            ),
            PlotType::Triangle => (
                generate_triangle_plot(gossip_simulations, error_bars),
                generate_triangle_plot(population_simulations, error_bars),
            ),
        }
    }
}
