use axum::{
    routing::{get, get_service},
    Json, Router,
};
use game_lib::game::Game;
use serde_json::json;
use std::process::Command;
use tower_http::services::fs::ServeDir;

#[tokio::main]
async fn main() {
    let mut game1 = Game::init(false);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(get_service(ServeDir::new("web/dist")));

    // run our app with hyper, listening globally on port 3000
    println!("Listening 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// WebSocket connection handler
async fn ws_handler(ws: axum::extract::ws::WebSocketUpgrade) -> impl axum::response::IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: axum::extract::ws::WebSocket) {
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
                println!("Received text: {}", text);
            }
            _ => {}
        }
    }
}
