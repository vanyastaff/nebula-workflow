use nebula_idempotency::prelude::*;
use async_trait::async_trait;

struct AddAction;

#[async_trait]
impl IdempotentAction for AddAction {
    type Input = (u32, u32);
    type Output = u32;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, IdempotencyError> {
        Ok(input.0 + input.1)
    }
}

#[tokio::main]
async fn main() {
    let storage = InMemoryIdempotencyStorage::new();
    let action = AddAction;
    let executor = IdempotentExecutor::new(action, storage);
    let key = IdempotencyKey("add-1-2".to_string());
    let result = executor.execute(key, (1, 2)).await.unwrap();
    println!("Result: {result}");
} 