use futures::{SinkExt, StreamExt};
use game_lib::automation::ai::{Difficulty, AI};
use game_lib::game::Game;
use game_lib::piece::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, mpsc::UnboundedSender};
use tokio::time::{interval, Duration, Instant};
use tokio_tungstenite::tungstenite::Bytes;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Message, Utf8Bytes},
};
use uuid::Uuid;

mod messages;
use messages::{ClientMessage, ServerMessage};
mod sharedenums;
use sharedenums::{GameMode, PlayerRole};
mod handler;
use handler::*;
mod structures;
use structures::{Client, Player, PlayerType, Room, RoomStatus, ServerState, SharedServerState};

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(ServerState {
        clients: HashMap::new(),
        rooms: HashMap::new(),
    }));

    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("Server started on ws://127.0.0.1:9001");

    tokio::spawn(cleanup_inactive_rooms(state.clone()));
    tokio::spawn(inactivity_check(state.clone()));
    tokio::spawn(server_ping_loop(state.clone()));

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
                        last_active: Instant::now(),
                        hb: Instant::now(),
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
                    Ok(ClientMessage::Disconnect) => {
                        println!("Client {} is disconnecting", client_id);
                        let mut state_guard = state.lock().unwrap();
                        let client = state_guard.clients.remove(&client_id);
                        if let Some(client) = client {
                            if client.room_id.is_some() {
                                handle_quit(client_id, &state);
                            }
                            let _ = send_to_client(
                                &client,
                                &ServerMessage::Info {
                                    msg: "Disconnected".to_string(),
                                },
                            );
                        }
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
                    Ok(ClientMessage::Move { mv }) => {
                        if let Some(reply) = handle_move(client_id, mv, &state) {
                            let state_guard = state.lock().unwrap();
                            if let Some(client) = state_guard.clients.get(&client_id) {
                                let _ = send_to_client(client, &reply);
                            }
                        }
                    }
                    Ok(ClientMessage::StartGame) => {
                        let mut state_guard = state.lock().unwrap();
                        if let Some(room_id) =
                            state_guard.clients.get(&client_id).and_then(|c| c.room_id)
                        {
                            if let Some(room) = state_guard.rooms.get_mut(&room_id) {
                                if room.mode == GameMode::AIvsAI {
                                    room.status = RoomStatus::Running;
                                    drop(state_guard); // Release lock before async spawn
                                    tokio::spawn(run_ai_vs_ai_game(room_id, Arc::clone(&state)));
                                }
                            }
                        }
                    }
                    Ok(ClientMessage::StartSandboxGame) => {
                        println!("Client {} started sandbox game", client_id);
                        let mut state = state.lock().unwrap();
                        if let Some(room_id) = state.clients.get(&client_id).and_then(|c| c.room_id)
                        {
                            if let Some(room) = state.rooms.get_mut(&room_id) {
                                if room.mode == GameMode::Sandbox {
                                    room.status = RoomStatus::Running;
                                    send_game_state_to_clients(room);
                                    for p in room.players.values() {
                                        let _ = send_to_player(p, &ServerMessage::GameStarted);
                                    }
                                }
                            }
                        }
                    }
                    Ok(ClientMessage::AddPiece { piece, pos }) => {
                        println!("Client {} adds piece {} to {}", client_id, piece, pos);
                        let mut state = state.lock().unwrap();
                        if let Some(room_id) = state.clients.get(&client_id).and_then(|c| c.room_id)
                        {
                            if let Some(room) = state.rooms.get_mut(&room_id) {
                                if room.mode == GameMode::Sandbox {
                                    room.game.board.add_piece(&format!("{}{}", piece, pos));
                                    send_game_state_to_clients(room);
                                    if let Some(player) = room.players.get(&client_id) {
                                        let _ = send_to_player(
                                            player,
                                            &ServerMessage::SandboxPieceAdded { piece, pos },
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Ok(ClientMessage::Quit) => {
                        println!("Client {} sent Quit", client_id);
                        handle_quit(client_id, &state);
                    }
                    Ok(ClientMessage::PauseRequest) => {
                        println!("Client {} sent PauseRequest", client_id);
                        let mut state_guard = state.lock().unwrap();
                        if let Some(room_id) =
                            state_guard.clients.get(&client_id).and_then(|c| c.room_id)
                        {
                            toggle_pause_game(room_id, &state);
                        }
                    }
                    Ok(ClientMessage::Pong) => {
                        println!("Client {} sent Pong", client_id);
                        let mut state = state.lock().unwrap();
                        if let Some(client) = state.clients.get_mut(&client_id) {
                            client.hb = Instant::now();
                        }
                    }
                    Err(e) => {
                        eprintln!("Invalid message from client {}: {}", client_id, e);
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
    if let Some(sender) = &player.sender {
        sender
            .send(Message::Text(serialized.into()))
            .map_err(|e| format!("Failed to send to player: {}", e))
    } else {
        Err("Player is an Ai".to_string())
    }
}

pub async fn cleanup_inactive_rooms(state: SharedServerState) {
    let mut interval = interval(Duration::from_secs(60)); // Vérifie toutes les 60s
    loop {
        interval.tick().await;

        println!("Cleaning up inactive rooms...");

        let mut state = match state.lock() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to acquire state lock during cleanup: {}", e);
                continue;
            }
        };

        let now = Instant::now();
        let timeout = Duration::from_secs(300); // 5 min

        let mut to_remove = vec![];

        // 1. Identifier les rooms à supprimer
        for (room_id, room) in state.rooms.iter() {
            if room.status == RoomStatus::Running {
                continue;
            }

            let inactive_too_long = now.duration_since(room.created_at) > timeout;
            let empty = room.players.is_empty();

            if empty || inactive_too_long {
                println!(
                    "Room {:?} will be removed. Status: {:?}, Players: {}, Inactive: {}s",
                    room_id,
                    room.status,
                    room.players.len(),
                    now.duration_since(room.created_at).as_secs()
                );
                to_remove.push(*room_id);
            }
        }

        // 2. Supprimer les rooms et réinitialiser les clients associés
        for room_id in to_remove {
            if let Some(room) = state.rooms.remove(&room_id) {
                for player in room.players.values() {
                    if let Some(client) = state.clients.get_mut(&player.id) {
                        if client.room_id == Some(room_id) {
                            client.room_id = None;
                        }
                    }
                }
            }
            println!("Room {:?} removed", room_id);
            println!(
                "Current rooms: {:?}",
                state.rooms.keys().collect::<Vec<_>>()
            );
        }
    }
}

pub async fn inactivity_check(state: SharedServerState) {
    let mut interval = interval(Duration::from_secs(60)); // Vérifie toutes les 60s
    loop {
        interval.tick().await;
        println!("Inactivity check...");

        let mut state_guard = match state.lock() {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "Failed to acquire state lock during inactivity check: {}",
                    e
                );
                continue;
            }
        };

        let mut to_remove: Vec<Uuid> = Vec::new();
        let now = Instant::now();
        for client in state_guard.clients.values_mut() {
            if now.duration_since(client.hb) > Duration::from_secs(300) {
                println!("Client {:?} is inactive for too long", client.id);
                handle_quit(client.id, &state);
                to_remove.push(client.id);
            }
        }

        for id in to_remove {
            state_guard.clients.remove(&id);
            println!("Removed inactive client {:?}", id);
        }
        println!(
            "Current clients: {:?}",
            state_guard.clients.keys().collect::<Vec<_>>()
        );
    }
}

pub async fn server_ping_loop(state: SharedServerState) {
    let mut interval = interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        println!("Sending ping to clients...");

        let state_guard = match state.lock() {
            Ok(s) => s,
            Err(_) => continue,
        };

        for client in state_guard.clients.values() {
            let _ = send_to_client(client, &ServerMessage::Ping);
        }
    }
}
