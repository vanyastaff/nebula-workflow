use crate::workflow::strategies::CheckpointStrategy;
use crate::storage::workflow_memory::InMemoryCheckpointStorage;
use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};
use chrono::{DateTime, Utc};
use crate::workflow::state::WorkflowState;

/// Unique identifier for a workflow.
pub type WorkflowId = String;
/// Unique identifier for a checkpoint.
pub type CheckpointId = String;
/// Unique identifier for a node.
pub type NodeId = String;

/// Data for a workflow checkpoint.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkflowCheckpoint {
    pub workflow_id: WorkflowId,
    pub checkpoint_id: CheckpointId,
    pub created_at: DateTime<Utc>,
    pub completed_nodes: HashSet<NodeId>,
    pub workflow_state: WorkflowState,
    pub node_outputs: HashMap<NodeId, serde_json::Value>,
    pub variables: HashMap<String, serde_json::Value>,
}

/// Manages workflow checkpoints.
pub struct CheckpointManager {
    pub storage: InMemoryCheckpointStorage,
    pub strategy: CheckpointStrategy,
}

impl CheckpointManager {
    pub fn new(storage: InMemoryCheckpointStorage, strategy: CheckpointStrategy) -> Self {
        Self { storage, strategy }
    }

    pub async fn create_checkpoint(&self, checkpoint: WorkflowCheckpoint) -> Result<(), String> {
        self.storage.save(checkpoint);
        Ok(())
    }

    pub async fn find_latest_checkpoint(&self, workflow_id: &WorkflowId) -> Result<Option<WorkflowCheckpoint>, String> {
        Ok(self.storage.latest(workflow_id))
    }

    pub async fn resume_from_checkpoint(&self, _checkpoint: WorkflowCheckpoint) -> Result<(), String> {
        // TODO: logic to resume workflow from checkpoint
        Ok(())
    }
}
