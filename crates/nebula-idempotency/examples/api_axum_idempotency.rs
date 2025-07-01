use axum::{Router, routing::post, extract::Json, middleware, http::StatusCode};
use nebula_idempotency::request::middleware::{idempotency_middleware, IdempotencyCache};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let cache: IdempotencyCache = Arc::new(Mutex::new(std::collections::HashMap::new()));
    let app = Router::new()
        .route("/echo", post(echo))
        .layer(middleware::from_fn_with_state(cache.clone(), idempotency_middleware))
        .with_state(cache);
    println!("Try: curl -X POST http://localhost:3000/echo -H 'Idempotency-Key: test' -d '{{\"msg\":\"hi\"}}'");
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}

async fn echo(Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(payload))
} 