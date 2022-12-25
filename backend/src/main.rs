use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
    extract::Path
};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use serde_json::json;

struct CreateUser {
    username: String,
}

struct User {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(json_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}

async fn root() -> String {
    "Hello, world!".to_owned()
}

async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
    let greeting = name.as_str();
    let hello = String::from("Hello ");
    (StatusCode::OK, Json(json!({"message": hello + greeting})))
}