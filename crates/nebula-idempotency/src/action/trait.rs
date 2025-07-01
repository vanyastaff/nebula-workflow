use async_trait::async_trait;
use crate::core::{config::IdempotencyConfig, error::IdempotencyError};

/// Trait for idempotent actions.
#[async_trait]
pub trait IdempotentAction: Send + Sync {
    type Input: Send + Sync;
    type Output: Send + Sync + Clone;

    /// Returns the idempotency configuration for this action.
    fn idempotency_config(&self) -> IdempotencyConfig {
        IdempotencyConfig::default()
    }

    /// Executes the action.
    async fn execute(&self, input: Self::Input) -> Result<Self::Output, IdempotencyError>;
}
