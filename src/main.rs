use futures::{SinkExt, StreamExt};
use game_lib::game::Game;
use game_lib::piece::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

mod messages;
use messages::{ClientMessage, ServerMessage};
mod sharedenums;
use sharedenums::GameMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomStatus {
    WaitingPlayers,
    WaitingReady,
    ReadyToStart,
    Running,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerRole {
    White,
    Black,
    Spectator,
    Solo,
}

pub struct Client {
    pub id: Uuid,
    pub room_id: Option<Uuid>,
    pub sender: UnboundedSender<Message>,
    pub ready: bool,
}

pub struct Player {
    pub id: Uuid,
    pub role: PlayerRole,
    pub ready: bool,
    pub sender: UnboundedSender<Message>,
}

pub struct Room {
    pub id: Uuid,
    pub mode: GameMode,
    pub status: RoomStatus,
    pub players: HashMap<Uuid, Player>,
    pub game: Game,
}

pub struct ServerState {
    pub clients: HashMap<Uuid, Client>,
    pub rooms: HashMap<Uuid, Room>,
}

pub type SharedServerState = Arc<Mutex<ServerState>>;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(ServerState {
        clients: HashMap::new(),
        rooms: HashMap::new(),
    }));

    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("Server started on ws://127.0.0.1:9001");

    while let Ok((stream, _)) = listener.accept().await {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("WebSocket handshake failed: {}", e);
                    return;
                }
            };
            let (mut ws_tx, mut ws_rx) = ws_stream.split();
            let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

            let client_id = Uuid::new_v4();

            {
                let mut state_lock = state.lock().unwrap();
                state_lock.clients.insert(
                    client_id,
                    Client {
                        id: client_id,
                        room_id: None,
                        sender: tx.clone(),
                        ready: false,
                    },
                );
            }

            // WebSocket sender task
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if let Err(e) = ws_tx.send(msg).await {
                        eprintln!("WebSocket send error: {}", e);
                        break;
                    }
                }
            });

            // Message handler loop
            while let Some(Ok(Message::Text(text))) = ws_rx.next().await {
                println!("Received message from client {}: {}", client_id, text);
                let parsed: Result<ClientMessage, _> = serde_json::from_str(&text);
                match parsed {
                    Ok(ClientMessage::Connect) => {
                        println!("Client {} sent Connect", client_id);
                    }
                    Ok(ClientMessage::Quit) => {
                        println!("Client {} is disconnecting", client_id);
                        break;
                    }
                    Ok(ClientMessage::JoinRoom { room_id }) => {
                        println!("Client {} wants to join room {:?}", client_id, room_id);
                        let msg = handle_join_room(client_id, room_id, &mut state.lock().unwrap());
                        if let Some(msg) = msg {
                            if let Err(e) =
                                send_to_client(&state.lock().unwrap().clients[&client_id], &msg)
                            {
                                eprintln!("Failed to send message to client {}: {}", client_id, e);
                            }
                            println!("Successfully joined room");
                            println!("Sent message to client {}: {:?}", client_id, msg);
                        } else {
                            println!("Failed to join room");
                        }
                    }
                    Ok(ClientMessage::CreateRoom { mode, difficulty }) => {
                        println!(
                            "Client {} wants to create room in {:?} mode",
                            client_id, mode
                        );

                        let msg = handle_create_room(
                            client_id,
                            mode,
                            difficulty,
                            &mut state.lock().unwrap(),
                        );
                        // Handle room creation logic here.
                        if let Some(msg) = msg {
                            if let Err(e) =
                                send_to_client(&state.lock().unwrap().clients[&client_id], &msg)
                            {
                                eprintln!("Failed to send message to client {}: {}", client_id, e);
                            }
                            println!("Sent message to client {}: {:?}", client_id, msg);
                        }
                        println!("Room created successfully");
                    }
                    Ok(ClientMessage::Ready) => {
                        println!("Client {} is ready", client_id);
                        handle_set_ready(client_id, &state);
                    }
                    Err(e) => {
                        eprintln!("Invalid message from client {}: {}", client_id, e);
                    }
                    _ => {
                        println!("Unhandled message from client {}", client_id);
                    }
                }
            }

            {
                let mut state = state.lock().unwrap();
                state.clients.remove(&client_id);
                println!("Client {} disconnected", client_id);
                println!(
                    "Current clients: {:?}",
                    state.clients.keys().collect::<Vec<_>>()
                );
            }
        });
    }
}

pub fn send_to_client(client: &Client, msg: &ServerMessage) -> Result<(), String> {
    let serialized = serde_json::to_string(msg)
        .map_err(|e| format!("Failed to serialize ServerMessage: {}", e))?;
    client
        .sender
        .send(Message::Text(serialized.into()))
        .map_err(|e| format!("Failed to send to client: {}", e))
}

pub fn send_to_player(player: &Player, msg: &ServerMessage) -> Result<(), String> {
    let serialized = serde_json::to_string(msg)
        .map_err(|e| format!("Failed to serialize ServerMessage: {}", e))?;
    player
        .sender
        .send(Message::Text(serialized.into()))
        .map_err(|e| format!("Failed to send to player: {}", e))
}

pub fn handle_create_room(
    client_id: Uuid,
    mode: GameMode,
    difficulty: Option<game_lib::automation::ai::Difficulty>,
    server_state: &mut ServerState,
) -> Option<ServerMessage> {
    let client = server_state.clients.get(&client_id)?;
    let room_id = Uuid::new_v4();
    let game = Game::init(matches!(mode, GameMode::Sandbox));
    let mut players = HashMap::new();
    let status = match mode {
        GameMode::PlayerVsPlayer => RoomStatus::WaitingPlayers,
        _ => RoomStatus::WaitingReady,
    };
    println!("Creating room with ID: {:?}", room_id);

    let role = match mode {
        GameMode::PlayerVsPlayer => PlayerRole::White,
        GameMode::PlayerVsAI => PlayerRole::White,
        GameMode::AIvsAI => PlayerRole::Spectator,
        GameMode::Sandbox => PlayerRole::Solo,
    };

    players.insert(
        client_id,
        Player {
            id: client_id,
            role,
            ready: false,
            sender: client.sender.clone(),
        },
    );

    server_state.rooms.insert(
        room_id,
        Room {
            id: room_id,
            mode,
            status,
            players,
            game,
        },
    );

    if let Some(c) = server_state.clients.get_mut(&client_id) {
        c.room_id = Some(room_id);
    }
    println!("returning from craeting room");

    Some(ServerMessage::Joined {
        color: Some(Color::White),
        room_id,
    })
}

pub fn handle_join_room(
    client_id: Uuid,
    room_id: Uuid,
    server_state: &mut ServerState,
) -> Option<ServerMessage> {
    let client = server_state.clients.get(&client_id)?;
    let room = server_state.rooms.get_mut(&room_id)?;

    if matches!(room.status, RoomStatus::Running | RoomStatus::Finished) {
        return Some(ServerMessage::Error {
            msg: "Cannot join this room.".into(),
        });
    }

    if room.players.contains_key(&client_id) {
        return Some(ServerMessage::Error {
            msg: "You already joined this room.".into(),
        });
    }

    let role = match room.mode {
        GameMode::PlayerVsPlayer if room.players.len() == 1 => PlayerRole::Black,
        GameMode::AIvsAI => PlayerRole::Spectator,
        _ => {
            return Some(ServerMessage::Error {
                msg: "Unsupported or full room.".into(),
            })
        }
    };

    room.players.insert(
        client_id,
        Player {
            id: client_id,
            role,
            ready: false,
            sender: client.sender.clone(),
        },
    );

    if let Some(c) = server_state.clients.get_mut(&client_id) {
        c.room_id = Some(room_id);
    }

    Some(ServerMessage::Joined {
        color: None,
        room_id,
    })
}

pub fn handle_game_over(room: &mut Room, reason: &str, state: &ServerState) {
    room.status = RoomStatus::Finished;
    for player in room.players.values() {
        let _ = send_to_player(
            player,
            &ServerMessage::GameOver {
                result: reason.to_string(),
            },
        );
    }
}

pub fn send_game_state_to_clients(room: &Room, state: &ServerState) {
    let board = room.game.board.export_display_board();
    let turn = if room.game.board.turn == Color::White {
        "White".to_string()
    } else {
        "Black".to_string()
    };

    for player in room.players.values() {
        let _ = send_to_player(
            player,
            &ServerMessage::State {
                board: board.clone(),
                turn: turn.clone(),
            },
        );
    }
}

pub fn handle_set_ready(client_id: Uuid, server_state: &Arc<Mutex<ServerState>>) {
    let mut state = server_state.lock().unwrap();
    let client = match state.clients.get(&client_id) {
        Some(c) => c,
        None => return,
    };
    let room_id = match client.room_id {
        Some(id) => id,
        None => return,
    };
    let room = match state.rooms.get_mut(&room_id) {
        Some(r) => r,
        None => return,
    };
    let player = match room.players.get_mut(&client_id) {
        Some(p) => p,
        None => return,
    };

    player.ready = true;
    let _ = send_to_player(
        player,
        &ServerMessage::Info {
            msg: "You are ready.".into(),
        },
    );

    if room.mode == GameMode::PlayerVsPlayer
        && matches!(
            room.status,
            RoomStatus::WaitingReady | RoomStatus::WaitingPlayers
        )
    {
        let all_ready = room
            .players
            .values()
            .filter(|p| matches!(p.role, PlayerRole::White | PlayerRole::Black))
            .all(|p| p.ready);
        let player_count = room
            .players
            .values()
            .filter(|p| matches!(p.role, PlayerRole::White | PlayerRole::Black))
            .count();

        if all_ready && player_count == 2 {
            room.status = RoomStatus::Running;
            for player in room.players.values() {
                let _ = send_to_player(player, &ServerMessage::GameStarted);
            }
            println!("Room {:?} game started (PvP)", room.id);
        }
    }
}
