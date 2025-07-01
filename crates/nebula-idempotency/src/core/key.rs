/// Represents a unique idempotency key for deduplication purposes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub String);
