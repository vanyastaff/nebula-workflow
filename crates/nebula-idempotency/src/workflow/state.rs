use serde::{Serialize, Deserialize};

/// State of a workflow execution.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum WorkflowState {
    Running,
    Completed,
    Failed,
}
