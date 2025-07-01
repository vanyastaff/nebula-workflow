use std::collections::HashSet;
use std::time::Duration;
use crate::workflow::NodeId;

/// Strategy for workflow checkpointing.
#[derive(Clone, Debug)]
pub enum CheckpointStrategy {
    /// Checkpoint after every node
    AfterEachNode,
    /// Only at critical points
    CriticalPointsOnly {
        critical_nodes: HashSet<NodeId>,
    },
    /// Time-based checkpoints
    TimeBased {
        interval: Duration,
    },
    /// Adaptive ML-based
    Adaptive {
        base_strategy: Box<CheckpointStrategy>,
        ml_optimization: bool,
    },
}
