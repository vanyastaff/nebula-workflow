use thiserror::Error;

/// Errors related to idempotency operations.
#[derive(Debug, Error)]
pub enum IdempotencyError {
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Idempotency conflict: {0}")]
    Conflict(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
