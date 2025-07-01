use async_trait::async_trait;
use crate::core::{key::IdempotencyKey, error::IdempotencyError};

/// Trait for idempotency storage backends.
#[async_trait]
pub trait IdempotencyStorage: Send + Sync {
    type Value: Send + Sync + Clone;

    async fn get(&self, key: &IdempotencyKey) -> Result<Option<Self::Value>, IdempotencyError>;
    async fn set(&self, key: IdempotencyKey, value: Self::Value) -> Result<(), IdempotencyError>;
    async fn remove(&self, key: &IdempotencyKey) -> Result<(), IdempotencyError>;
}
