use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Bad configuration")]
    BadConfig(#[from] rand::distributions::WeightedError),
}
