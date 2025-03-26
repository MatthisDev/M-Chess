mod game_lib;

use axum::{
    extract::{Path, State},
    routing::{get, get_service},
    Json, Router,
};
use game_lib::board::Board;
use game_lib::position::Position;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tower_http::services::fs::ServeDir;

#[tokio::main]
async fn main() {
    // Create the board at the start of the application
    let board = Arc::new(Mutex::new(Board::full_init()));

    // Build our application with the shared board state
    let app = Router::new()
        .route("/api", get(api_handler))
        .route("/move/{from}/{to}", get(move_piece_handler))
        .fallback_service(get_service(ServeDir::new("web/dist")))
        .with_state(board);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn api_handler(State(board): State<Arc<Mutex<Board>>>) -> Json<serde_json::Value> {
    // Use the board from state
    let board_data = board.lock().unwrap().get();

    Json(json!({
        "board": board_data
    }))
}

// Handler pour déplacer une pièce
// Format des paramètres:
// - from: position de départ au format algébrique (ex: "e2")
// - to: position d'arrivée au format algébrique (ex: "e4")
async fn move_piece_handler(
    State(board): State<Arc<Mutex<Board>>>,
    Path((from_str, to_str)): Path<(String, String)>,
) -> Json<serde_json::Value> {
    // Convertir les chaînes en positions
    let from_position = match Position::from_algebraic(&from_str) {
        Ok(pos) => pos,
        Err(_) => {
            return Json(json!({
                "success": false,
                "message": format!("Position invalide: {}", from_str)
            }));
        }
    };

    let to_position = match Position::from_algebraic(&to_str) {
        Ok(pos) => pos,
        Err(_) => {
            return Json(json!({
                "success": false,
                "message": format!("Position invalide: {}", to_str)
            }));
        }
    };

    // Effectuer le déplacement
    let success = {
        let mut board_guard = board.lock().unwrap();
        board_guard.move_piece(&from_position, &to_position)
    };

    if success {
        Json(json!({
            "success": true,
            "message": format!("Pièce déplacée de {} à {}", from_str, to_str)
        }))
    } else {
        Json(json!({
            "success": false,
            "message": "Déplacement invalide"
        }))
    }
}
