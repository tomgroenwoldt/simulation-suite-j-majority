use clap::ValueEnum;
use pgfplots::Picture;
use simulation::Simulation;

use self::triangle::generate_triangle;

mod triangle;

pub struct Plot {
    pub plot_type: PlotType,
    pub simulations: Vec<Simulation>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum PlotType {
    K,
    Triangle,
}

pub trait PictureGeneration {
    fn generate_picture(self) -> Picture;
}

impl PictureGeneration for Plot {
    fn generate_picture(self) -> Picture {
        match self.plot_type {
            PlotType::K => todo!(),
            PlotType::Triangle => generate_triangle(self.simulations),
        }
    }
}
