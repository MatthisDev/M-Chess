mod game_lib;

use axum::{
    extract::State,
    routing::{get, get_service},
    Json, Router,
};
use game_lib::board::Board;
use serde_json::json;
use std::sync::Arc;
use tower_http::services::fs::ServeDir;

#[tokio::main]
async fn main() {
    // Create the board at the start of the application
    let board = Arc::new(Board::full_init());

    // Build our application with the shared board state
    let app = Router::new()
        .route("/api", get(api_handler))
        .fallback_service(get_service(ServeDir::new("web/dist")))
        .with_state(board);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Modified to extract the board from the application state
async fn api_handler(State(board): State<Arc<Board>>) -> Json<serde_json::Value> {
    // Use the board from state
    let board_data: [[String; 8]; 8] = board.get();

    Json(json!({
        "message": "Hello, World!",
        "board": board_data
    }))
}
