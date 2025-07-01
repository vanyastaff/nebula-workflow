use nebula_idempotency::integration::transactional::{TransactionManager, TransactionalAction};
use nebula_idempotency::core::{key::IdempotencyKey, error::IdempotencyError};
use async_trait::async_trait;

struct TransferMoney;

#[async_trait]
impl TransactionalAction for TransferMoney {
    type Input = (u32, u32, u32); // from, to, amount
    type Output = String;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, IdempotencyError> {
        Ok(format!("Transferred {} from {} to {}", input.2, input.0, input.1))
    }
}

#[tokio::main]
async fn main() {
    let manager = TransactionManager::new();
    let action = TransferMoney;
    let tx_id = IdempotencyKey("tx-abc".to_string());
    let result = manager
        .execute_idempotent(&action, (1, 2, 100), tx_id, || async {
            action.execute((1, 2, 100)).await
        })
        .await
        .unwrap();
    println!("{result}");
} 