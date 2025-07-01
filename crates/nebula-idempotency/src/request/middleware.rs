use axum::{
    body::{self, Body},
    extract::State,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type IdempotencyCache = Arc<Mutex<HashMap<String, Vec<u8>>>>;

pub async fn idempotency_middleware(
    State(cache): State<IdempotencyCache>,
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    if req.method() != axum::http::Method::POST {
        return next.run(req).await;
    }
    let key = req
        .headers()
        .get("Idempotency-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    if let Some(key) = key {
        if let Some(cached) = cache.lock().unwrap().get(&key).cloned() {
            return Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(cached))
                .unwrap();
        }
        let response = next.run(req).await;
        let (parts, body) = response.into_parts();
        let body_bytes = body::to_bytes(body, 2 * 1024 * 1024).await.unwrap();
        cache.lock().unwrap().insert(key, body_bytes.clone().to_vec());
        Response::from_parts(parts, Body::from(body_bytes))
    } else {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Missing Idempotency-Key header"))
            .unwrap()
    }
}
