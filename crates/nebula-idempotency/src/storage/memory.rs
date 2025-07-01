use std::collections::HashMap;
use std::sync::Mutex;
use crate::core::{key::IdempotencyKey, error::IdempotencyError};
use super::traits::IdempotencyStorage;

/// In-memory idempotency storage (no TTL for MVP).
pub struct InMemoryIdempotencyStorage<V: Send + Sync + Clone + 'static> {
    map: Mutex<HashMap<IdempotencyKey, V>>,
}

impl<V: Send + Sync + Clone + 'static> InMemoryIdempotencyStorage<V> {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }
}

impl<V: Send + Sync + Clone + 'static> Default for InMemoryIdempotencyStorage<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl<V: Send + Sync + Clone + 'static> IdempotencyStorage for InMemoryIdempotencyStorage<V> {
    type Value = V;

    async fn get(&self, key: &IdempotencyKey) -> Result<Option<Self::Value>, IdempotencyError> {
        let map = self.map.lock().map_err(|e| IdempotencyError::Storage(e.to_string()))?;
        Ok(map.get(key).cloned())
    }

    async fn set(&self, key: IdempotencyKey, value: Self::Value) -> Result<(), IdempotencyError> {
        let mut map = self.map.lock().map_err(|e| IdempotencyError::Storage(e.to_string()))?;
        map.insert(key, value);
        Ok(())
    }

    async fn remove(&self, key: &IdempotencyKey) -> Result<(), IdempotencyError> {
        let mut map = self.map.lock().map_err(|e| IdempotencyError::Storage(e.to_string()))?;
        map.remove(key);
        Ok(())
    }
}
