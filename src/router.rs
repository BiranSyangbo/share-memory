use axum::{
    extract::Multipart, http::StatusCode, routing::{get, post}, Json, Router
};
use serde_json::Value;

pub fn router() -> Router {
    return Router::new()
        .route("/", get(root))
        .route("/api/v1/upload", post(upload))
        .fallback(fallback);
}

async fn root() {}
async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}

async fn fallback() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"status": "Not Found"})),
    )
}
