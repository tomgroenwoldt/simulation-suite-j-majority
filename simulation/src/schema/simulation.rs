use std::sync::{mpsc::SyncSender, Arc, Condvar, Mutex};

use super::{agent::Agent, config::Config, opinion_distribution::OpinionDistribution};
use crate::export::OpinionPlot;

#[derive(Debug)]
pub struct Simulation {
    /// Collection of agents
    pub agents: Vec<Agent>,
    pub sample_size: u8,
    /// Current opinion
    pub k: u16,
    pub upper_bound_k: u16,
    /// Counts the interactions of all agents
    pub interaction_count: u64,
    /// Stores number of occurences for each opinion
    pub opinion_distribution: OpinionDistribution,
    /// Communication channel to UI
    pub sender: SyncSender<SimulationMessage>,
    /// State and condition variable, which can block the executor thread
    /// of the simulation
    pub state: Arc<(Mutex<SimulationState>, Condvar)>,
    /// Stores the initial config for resetting purposes
    pub config: Config,
    pub plot: OpinionPlot,
}

#[derive(Debug, Default)]
pub struct FrontendSimulation {
    pub opinion_distribution: OpinionDistribution,
    pub current_opinion: u16,
    pub finished: bool,
    pub plot: OpinionPlot,
}

#[derive(Debug, PartialEq)]
pub enum SimulationState {
    Pause,
    Play,
    ReadyForNext,
    Exit,
}

#[derive(Clone, Debug)]
pub enum SimulationMessage {
    Pause,
    Play,
    Abort,
    Update(OpinionDistribution),
    Next,
    Finish(OpinionPlot),
}
