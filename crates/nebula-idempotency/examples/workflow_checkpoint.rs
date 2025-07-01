use nebula_idempotency::workflow::checkpoint::WorkflowCheckpoint;
use nebula_idempotency::workflow::state::WorkflowState;
use nebula_idempotency::workflow::strategies::CheckpointStrategy;
use nebula_idempotency::workflow::checkpoint::CheckpointManager;
use nebula_idempotency::storage::workflow_memory::InMemoryCheckpointStorage;
use chrono::Utc;
use std::collections::{HashSet, HashMap};

#[tokio::main]
async fn main() {
    let storage = InMemoryCheckpointStorage::new();
    let strategy = CheckpointStrategy::AfterEachNode;
    let manager = CheckpointManager::new(storage, strategy);
    let workflow_id = "wf-1".to_string();
    let checkpoint = WorkflowCheckpoint {
        workflow_id: workflow_id.clone(),
        checkpoint_id: "cp-1".to_string(),
        created_at: Utc::now(),
        completed_nodes: HashSet::new(),
        workflow_state: WorkflowState::Running,
        node_outputs: HashMap::new(),
        variables: HashMap::new(),
    };
    manager.create_checkpoint(checkpoint).await.unwrap();
    let latest = manager.find_latest_checkpoint(&workflow_id).await.unwrap();
    println!("Latest checkpoint: {latest:?}");
} 