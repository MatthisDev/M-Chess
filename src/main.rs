use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use game_lib::game::{Game, GameMode, PlayerType};
use game_lib::piece::Color;

// Assuming AI lives here

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
    Join,
    Move { mv: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ServerMessage {
    AssignColor {
        color: Color,
    },
    State {
        board: Vec<Vec<Option<String>>>,
        turn: Color,
    },
    Error {
        message: String,
    },
    GameOver {
        result: String,
    },
}

struct Player {
    color: Color,
    sender: mpsc::UnboundedSender<Message>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let mode = parse_mode(args.get(1));
    let game = Arc::new(Mutex::new(Game::init(mode)));

    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Server running on {}", addr);

    let players: Arc<Mutex<HashMap<Color, Player>>> = Arc::new(Mutex::new(HashMap::new()));

    // Instant AI vs AI
    if mode == GameMode::AIvsAI {
        let mut game = game.lock().unwrap();
        game.run_ai_loop();
        return;
    }

    // Connection loop
    while players.lock().unwrap().len() < 2 && mode != GameMode::Sandbox {
        let (stream, _addr) = listener.accept().await.expect("Accept failed");
        let ws_stream = accept_async(stream)
            .await
            .expect("WebSocket handshake failed");
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let color = {
            let mut players_guard = players.lock().unwrap();
            if players_guard.contains_key(&Color::White) {
                Color::Black
            } else {
                Color::White
            }
        };

        ws_sender
            .send(Message::Text(Utf8Bytes::from(
                serde_json::to_string(&ServerMessage::AssignColor { color }).unwrap(),
            )))
            .await
            .unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
        {
            let mut players_guard = players.lock().unwrap();
            players_guard.insert(
                color,
                Player {
                    color,
                    sender: tx.clone(),
                },
            );
        }

        // Sender task
        let tx_sender = tx.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = ws_sender.send(msg).await {
                    eprintln!("Send error: {}", e);
                    break;
                }
            }
        });

        // Receiver task
        let game_arc = Arc::clone(&game);
        let players_arc = Arc::clone(&players);
        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
                if let Ok(ClientMessage::Move { mv }) = serde_json::from_str(&text) {
                    let mut game = game_arc.lock().unwrap();

                    if game.get_current_turn() != color {
                        let _ = tx_sender.send(Message::Text(Utf8Bytes::from(
                            serde_json::to_string(&ServerMessage::Error {
                                message: "Not your turn.".into(),
                            })
                            .unwrap(),
                        )));
                        continue;
                    }

                    match game.make_move_algebraic(&mv) {
                        Ok(true) => {
                            broadcast_state(&game, &players_arc);
                            if game.is_ai_turn() {
                                if let Ok(true) = game.run_ai_turn() {
                                    broadcast_state(&game, &players_arc);
                                }
                            }
                        }
                        Ok(false) => {
                            broadcast_gameover(&players_arc, "Game over.");
                        }
                        Err(e) => {
                            let _ = tx_sender.send(Message::Text(Utf8Bytes::from(
                                serde_json::to_string(&ServerMessage::Error {
                                    message: e.to_string(),
                                })
                                .unwrap(),
                            )));
                        }
                    }
                } else {
                    let _ = tx_sender.send(Message::Text(Utf8Bytes::from(
                        serde_json::to_string(&ServerMessage::Error {
                            message: "Invalid message format.".into(),
                        })
                        .unwrap(),
                    )));
                }
            }
        });
    }

    println!("Game started.");
}

fn parse_mode(arg: Option<&String>) -> GameMode {
    match arg.map(|s| s.as_str()) {
        Some("sandbox") => GameMode::Sandbox,
        Some("pvai") => GameMode::PlayerVsAI,
        Some("pvp") => GameMode::PlayerVsPlayer,
        Some("aivai") => GameMode::AIvsAI,
        _ => {
            println!("Defaulting to PlayerVsAI. Use one of: sandbox, pvai, pvp, aivai");
            GameMode::PlayerVsAI
        }
    }
}

fn broadcast_state(game: &Game, players: &Arc<Mutex<HashMap<Color, Player>>>) {
    let msg = ServerMessage::State {
        board: game.board.export_display_board(), // assume this returns Vec<Vec<Option<String>>>
        turn: game.get_current_turn(),
    };
    let msg_text = Message::Text(Utf8Bytes::from(serde_json::to_string(&msg).unwrap()));

    for player in players.lock().unwrap().values() {
        let _ = player.sender.send(msg_text.clone());
    }
}

fn broadcast_gameover(players: &Arc<Mutex<HashMap<Color, Player>>>, reason: &str) {
    let msg = ServerMessage::GameOver {
        result: reason.to_string(),
    };
    let msg_text = Message::Text(Utf8Bytes::from(serde_json::to_string(&msg).unwrap()));

    for player in players.lock().unwrap().values() {
        let _ = player.sender.send(msg_text.clone());
    }
}
