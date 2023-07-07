use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The simulation does not hold any agents.")]
    EmptyAgents,
    #[error("The sample for an agent update is not allowed to be empty.")]
    EmptySample,
    #[error("Error while starting GUI.")]
    Egui(#[from] eframe::Error),
    #[error("Error sending via channel.")]
    SendError(#[from] std::sync::mpsc::SendError<crate::simulation::SimulationMessage>),
    #[error("Error sending via broadcast channel.")]
    BroadcastSendError(
        #[from] tokio::sync::broadcast::error::SendError<crate::simulation::SimulationMessage>,
    ),
    #[error("Error exporting plots to CSV.")]
    ExportError,
}
