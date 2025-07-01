/// Idempotency levels supported by the system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdempotencyLevel {
    Action,
    Workflow,
    Request,
    Transaction,
}
