use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The simulation does not hold any agents.")]
    EmptyAgents,
    #[error("The sample for an agent update is not allowed to be empty.")]
    EmptySample,
}
