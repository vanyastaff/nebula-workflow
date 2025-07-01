use crate::core::{key::IdempotencyKey, error::IdempotencyError};
use crate::action::r#trait::IdempotentAction;
use crate::storage::traits::IdempotencyStorage;

/// Executor for idempotent actions.
pub struct IdempotentExecutor<A, S>
where
    A: IdempotentAction,
    S: IdempotencyStorage<Value = A::Output>,
{
    pub storage: S,
    pub action: A,
}

impl<A, S> IdempotentExecutor<A, S>
where
    A: IdempotentAction,
    S: IdempotencyStorage<Value = A::Output>,
{
    pub fn new(action: A, storage: S) -> Self {
        Self { action, storage }
    }

    pub async fn execute(&self, key: IdempotencyKey, input: A::Input) -> Result<A::Output, IdempotencyError> {
        // Check cache
        if let Some(cached) = self.storage.get(&key).await? {
            return Ok(cached);
        }
        // Execute action
        let result = self.action.execute(input).await?;
        // Store result
        self.storage.set(key, result.clone()).await?;
        Ok(result)
    }
}
