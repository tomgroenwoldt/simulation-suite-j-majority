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
    fn generate_picture(self) -> Picture;
}

impl PictureGeneration for Plot {
    fn generate_picture(self) -> Picture {
        match self.plot_type {
            PlotType::Entropy => generate_entropy_plot(self.simulations),
            PlotType::K => generate_k_plot(self.simulations),
            PlotType::N => generate_n_plot(self.simulations),
            PlotType::Triangle => generate_triangle_plot(self.simulations),
        }
    }
}
