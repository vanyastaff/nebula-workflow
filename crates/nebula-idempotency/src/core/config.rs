use super::level::IdempotencyLevel;
use std::time::Duration;

/// Strategy for generating idempotency keys.
#[derive(Clone, Debug)]
pub enum IdempotencyKeyStrategy {
    ContentBased,
    UserProvided,
    Hybrid {
        user_key_prefix: bool,
        content_suffix: bool,
    },
}

/// Behavior on idempotency conflict.
#[derive(Clone, Debug)]
pub enum ConflictBehavior {
    ReturnPrevious,
    WaitForCompletion { timeout: Duration },
    Merge,
}

/// Storage backend selection.
#[derive(Clone, Debug)]
pub enum IdempotencyStorageBackend {
    TierSpecific,
    InMemory,
    Postgres,
    Redis,
    Distributed,
}

/// Result caching configuration.
#[derive(Clone, Debug)]
pub struct ResultCachingConfig {
    pub enabled: bool,
}

impl Default for ResultCachingConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Configuration for idempotency behavior.
#[derive(Clone, Debug)]
pub struct IdempotencyConfig {
    pub enabled: bool,
    pub level: IdempotencyLevel,
    pub key_strategy: IdempotencyKeyStrategy,
    pub deduplication_window: Duration,
    pub conflict_behavior: ConflictBehavior,
    pub storage_backend: IdempotencyStorageBackend,
    pub result_caching: ResultCachingConfig,
}

impl Default for IdempotencyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: IdempotencyLevel::Action,
            key_strategy: IdempotencyKeyStrategy::ContentBased,
            deduplication_window: Duration::from_secs(3600),
            conflict_behavior: ConflictBehavior::ReturnPrevious,
            storage_backend: IdempotencyStorageBackend::InMemory,
            result_caching: ResultCachingConfig::default(),
        }
    }
}
