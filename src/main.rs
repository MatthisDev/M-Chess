use axum::{
    extract::{Path, State},
    routing::{get, get_service},
    Json, Router,
};
use game_lib::game::Game;
use serde_json::json;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::process::Command;
use tower_http::services::fs::ServeDir;

#[tokio::main]
async fn main() {
    // Wrap games in Arc<Mutex<_>> for thread-safe sharing
    let games = Arc::new(Mutex::new(Vec::<Game>::new()));

    let app = Router::new()
        .route("/create", get(create_game_handler))
        .route("/join/:uid", get(join_game_handler))
        .route("/ws/:uid", get(ws_handler))
        .fallback_service(get_service(ServeDir::new("web/dist")))
        .with_state(games);

    // run our app with hyper, listening globally on port 3000
    println!("Listening 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for creating a new game
async fn create_game_handler(
    State(games): State<Arc<Mutex<Vec<Game>>>>,
) -> Json<serde_json::Value> {
    // Create a new game instance
    let game = Game::init(false);
    let game_id = game.uid.clone();

    // Add the game to our vector
    games.lock().unwrap().push(game);

    // Return the game state as JSON
    Json(json!({
        "game_id": game_id,
        "status": "created"
    }))
}

// Handler for joining a game
async fn join_game_handler(
    Path(game_id): Path<String>,
    State(games): State<Arc<Mutex<Vec<Game>>>>,
) -> Json<serde_json::Value> {
    let games_lock = games.lock().unwrap();

    // Find the game with the given ID
    if let Some(game) = games_lock.iter().find(|g| g.uid == game_id) {
        Json(json!({
            "status": "ok",
            "game_id": game_id
        }))
    } else {
        Json(json!({
            "status": "error",
            "message": "Game not found"
        }))
    }
}

// WebSocket connection handler
async fn ws_handler(
    ws: axum::extract::ws::WebSocketUpgrade,
    Path(uid): Path<String>, // Extraire l'uid depuis le chemin
    State(games): State<Arc<Mutex<Vec<Game>>>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, uid, games))
}

async fn handle_socket(
    mut socket: axum::extract::ws::WebSocket,
    uid: String,
    games: Arc<Mutex<Vec<Game>>>,
) {
    // Handle the WebSocket connection here
    // For example, you can send a message to the client
    let message = json!({
        "type": "welcome",
        "message": "Welcome to the game!"
    });
    socket
        .send(axum::extract::ws::Message::Text(message.to_string().into()))
        .await
        .unwrap();

    // You can also receive messages from the client
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            axum::extract::ws::Message::Text(text) => {
                // Afficher l'ID de la game et le contenu du message dans le terminal
                println!("id de la game : {}", uid);
                println!("contenu du msg : {}", text);

                socket
                    .send(axum::extract::ws::Message::Text(
                        format!("You said: {}", text).as_str().into(),
                    ))
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }
}
