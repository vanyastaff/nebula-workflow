use async_trait::async_trait;
use crate::core::{key::IdempotencyKey, error::IdempotencyError};

#[async_trait]
pub trait TransactionalAction: Send + Sync {
    type Input: Send + Sync;
    type Output: Send + Sync;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, IdempotencyError>;
}

/// Transaction manager with idempotency support (MVP).
pub struct TransactionManager;

impl TransactionManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute_idempotent<A, F, Fut, T>(&self, _action: &A, _input: A::Input, _tx_id: IdempotencyKey, f: F) -> Result<T, IdempotencyError>
    where
        A: TransactionalAction,
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = Result<T, IdempotencyError>> + Send,
        T: Send + Sync,
    {
        // TODO: check idempotency by tx_id, run f(), store result
        f().await
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}
