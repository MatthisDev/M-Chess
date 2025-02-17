mod game_lib;

use axum::{
    routing::{get, get_service},
    Json, Router,
};
use serde_json::json;
use tower_http::services::fs::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api", get(api_handler))
        .fallback_service(get_service(ServeDir::new("web/dist")));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Basic API handler that responds with a JSON object
async fn api_handler() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, API!" }))
}
