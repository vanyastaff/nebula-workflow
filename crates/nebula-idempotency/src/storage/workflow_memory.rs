use std::collections::HashMap;
use std::sync::Mutex;
use crate::workflow::{WorkflowCheckpoint, WorkflowId};

/// In-memory storage for workflow checkpoints.
pub struct InMemoryCheckpointStorage {
    map: Mutex<HashMap<WorkflowId, Vec<WorkflowCheckpoint>>>,
}

impl InMemoryCheckpointStorage {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    /// Save a checkpoint for a workflow.
    pub fn save(&self, checkpoint: WorkflowCheckpoint) {
        let mut map = self.map.lock().unwrap();
        map.entry(checkpoint.workflow_id.clone())
            .or_default()
            .push(checkpoint);
    }

    /// Get the latest checkpoint for a workflow.
    pub fn latest(&self, workflow_id: &WorkflowId) -> Option<WorkflowCheckpoint> {
        let map = self.map.lock().unwrap();
        map.get(workflow_id)
            .and_then(|v| v.last().cloned())
    }

    /// Clear all checkpoints for a workflow.
    pub fn clear(&self, workflow_id: &WorkflowId) {
        let mut map = self.map.lock().unwrap();
        map.remove(workflow_id);
    }
}

impl Default for InMemoryCheckpointStorage {
    fn default() -> Self {
        Self::new()
    }
}
