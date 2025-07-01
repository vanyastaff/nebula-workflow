use std::collections::HashMap;
use std::sync::Mutex;
use std::future::Future;
use std::sync::Arc;
use crate::core::{key::IdempotencyKey, error::IdempotencyError};

/// In-memory request idempotency manager (MVP).
pub struct RequestIdempotencyManager<V: Send + Sync + Clone + 'static> {
    cache: Arc<Mutex<HashMap<IdempotencyKey, V>>>,
}

impl<V: Send + Sync + Clone + 'static> RequestIdempotencyManager<V> {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn execute_request<F, Fut>(&self, key: IdempotencyKey, fut: F) -> Result<V, IdempotencyError>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<V, IdempotencyError>> + Send,
    {
        // Check cache
        if let Some(val) = self.cache.lock().unwrap().get(&key).cloned() {
            return Ok(val);
        }
        // Execute and cache
        let result = fut().await?;
        self.cache.lock().unwrap().insert(key, result.clone());
        Ok(result)
    }
}

impl<V: Send + Sync + Clone + 'static> Default for RequestIdempotencyManager<V> {
    fn default() -> Self {
        Self::new()
    }
}
