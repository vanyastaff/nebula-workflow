use nebula_idempotency::core::key::IdempotencyKey;
use nebula_idempotency::request::manager::RequestIdempotencyManager;

#[tokio::main]
async fn main() {
    let manager = RequestIdempotencyManager::<String>::new();
    let key = IdempotencyKey("req-123".to_string());
    let result1 = manager.execute_request(key.clone(), || async {
        // Имитация обработки запроса
        Ok("response-1".to_string())
    }).await.unwrap();
    let result2 = manager.execute_request(key.clone(), || async {
        Ok("response-2".to_string())
    }).await.unwrap();
    println!("First: {result1} | Second (should be cached): {result2}");
} 