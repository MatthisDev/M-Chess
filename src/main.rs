use futures::{SinkExt, StreamExt};
use game_lib::automation::ai::{Difficulty, AI};
use game_lib::game::Game;
use game_lib::messages::{ClientMessage, ServerMessage};
use game_lib::piece::Color;
use game_lib::sharedenums::{GameMode, PlayerRole, RoomStatus};
use room::RoomCommand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::sync::{mpsc, mpsc::UnboundedSender};
use tokio::time::{interval, Duration, Instant};
use tokio_tungstenite::tungstenite::Bytes;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Message, Utf8Bytes},
};
use utils::{Client, Player};
use uuid::Uuid;
mod handler;
use handler::*;
mod room;
mod serverstate;
mod utils;
use serverstate::{ServerState, SharedServerState};
use std::time::{SystemTime, UNIX_EPOCH};

fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(ServerState {
        clients: HashMap::new(),
        room_senders: HashMap::new(),
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
                let client = Client {
                    id: client_id,
                    room_id: None,
                    sender: tx.clone(),
                    hb: Arc::new(AtomicU64::new(now_timestamp())),
                };
                send_to_client(&client, &ServerMessage::QuitGame);
                let mut state_lock = state.lock().unwrap();
                state_lock.clients.insert(client_id, client);
                println!("Client with id {} connected!!", client_id);
                println!("Current clients: {:?}", state_lock.clients);
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
            while let Some(msg) = ws_rx.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let parsed: Result<ClientMessage, _> = serde_json::from_str(&text);
                        match parsed {
                            Ok(ClientMessage::CreateRoom { mode, difficulty }) => {
                                println!(
                                    "Client {} wants to create room in {:?} mode",
                                    client_id, mode
                                );
                                let mut server_state = state.lock().unwrap();

                                let msg = server_state.create_room(client_id, mode, difficulty);
                                // Handle room creation logic here.
                                if let Some(msg) = msg {
                                    println!("Sending after join");
                                    if let Err(e) =
                                        send_to_client(&server_state.clients[&client_id], &msg)
                                    {
                                        eprintln!(
                                            "Failed to send message to client {}: {}",
                                            client_id, e
                                        );
                                    }
                                    println!("Sent message to client {}: {:?}", client_id, msg);
                                }
                                println!("Room created successfully");
                            }
                            Ok(ClientMessage::JoinRoom { room_id }) => {
                                println!("Client {} wants to join room {:?}", client_id, room_id);
                                let (room, client) = {
                                    let server_state = state.lock().unwrap();
                                    let client = server_state
                                        .clients
                                        .get(&client_id)
                                        .map(|client| client.sender.clone());
                                    let room = server_state.room_senders.get(&room_id).cloned();
                                    (room, client)
                                };
                                if let (Some(room), Some(client)) = (room, client) {
                                    let msg = join_room(room, client, client_id, room_id).await;

                                    if let Some(msg) = msg {
                                        if let Err(e) = send_to_client(
                                            &state.lock().unwrap().clients[&client_id],
                                            &msg,
                                        ) {
                                            eprintln!(
                                                "Failed to send message to client {}: {}",
                                                client_id, e
                                            );
                                        }
                                        if let Some(client) =
                                            state.lock().unwrap().clients.get_mut(&client_id)
                                        {
                                            client.room_id = Some(room_id);
                                        }

                                        println!("Successfully joined room");
                                    } else {
                                        println!("Failed to join room");
                                    }
                                }
                            }

                            Ok(ClientMessage::Ready {
                                state: client_state,
                            }) => {
                                let mut state = state.lock().unwrap();
                                state.set_player_ready(client_id, client_state);
                            }
                            Ok(ClientMessage::GetLegalMoves { mv }) => {
                                println!("Ask from client moves: {}", mv);
                                let mut server_state = state.lock().unwrap();
                                server_state.get_moves(client_id, mv);
                            }
                            Ok(ClientMessage::Move { mv }) => {
                                println!("Move from client: {}", mv);
                                let mut server_state = state.lock().unwrap();
                                server_state.make_move(client_id, mv);
                            }
                            Ok(ClientMessage::StartGame) => {
                                let mut state = state.lock().unwrap();
                                state.start_game(client_id);
                            }
                            Ok(ClientMessage::StartSandboxGame) => {
                                println!("Client {} started sandbox game", client_id);
                                let mut state = state.lock().unwrap();
                                if let Some(room_id) =
                                    state.clients.get(&client_id).and_then(|c| c.room_id)
                                {
                                    state.start_sandbox_game(room_id);
                                }
                            }
                            Ok(ClientMessage::AddPiece { piece, pos }) => {
                                let mut state = state.lock().unwrap();
                                if let Some(room_id) =
                                    state.clients.get(&client_id).and_then(|c| c.room_id)
                                {
                                    state.add_piece(room_id, pos.clone(), piece.clone(), client_id);
                                }
                                println!("Client {} adds piece {} to {}", client_id, piece, pos);
                            }
                            Ok(ClientMessage::Quit) => {
                                println!("Client {} wants to quit", client_id);

                                let (room, client) = {
                                    let mut server_state = state.lock().unwrap();
                                    let client_sender = server_state
                                        .clients
                                        .get(&client_id)
                                        .map(|c| c.sender.clone());
                                    let room_id = server_state
                                        .clients
                                        .get(&client_id)
                                        .and_then(|c| c.room_id);
                                    let room = room_id
                                        .and_then(|id| server_state.room_senders.get(&id).cloned());
                                    // On enlève la room_id du client ici pour éviter conflit
                                    if let Some(client) = server_state.clients.get_mut(&client_id) {
                                        client.room_id = None;
                                    }
                                    (room, client_sender)
                                };

                                if let Some(room) = room {
                                    let msg = quit_room(room, client_id).await;

                                    if let Some(msg) = msg {
                                        if let ServerMessage::CloseRoom { id } = &msg {
                                            let mut server_state = state.lock().unwrap();
                                            server_state.room_senders.remove(id);
                                            println!("Room {} deleted !!", id);
                                        }
                                        if let Err(e) = send_to_client(
                                            &state.lock().unwrap().clients[&client_id],
                                            &msg,
                                        ) {
                                            eprintln!(
                                                "Failed to send message to client {}: {}",
                                                client_id, e
                                            );
                                        }
                                    }
                                }
                            }

                            Ok(ClientMessage::PauseRequest) => {
                                println!("Client {} sent PauseRequest", client_id);
                                let mut state_guard = state.lock().unwrap();
                                if let Some(room_id) =
                                    state_guard.clients.get(&client_id).and_then(|c| c.room_id)
                                {
                                    state_guard.toggle_pause_game(room_id, client_id);
                                }
                            }
                            Ok(ClientMessage::Pong) => {
                                println!("Client {} sent Pong", client_id);
                                let mut state = state.lock().unwrap();
                                if let Some(client) = state.clients.get_mut(&client_id) {
                                    client.hb.store(now_timestamp(), Ordering::SeqCst);
                                }
                            }
                            Err(e) => {
                                eprintln!("Invalid message from client {}: {}", client_id, e);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("Client {} requested close", client_id);
                        let (room, client) = {
                            let mut server_state = state.lock().unwrap();
                            let client_sender = server_state
                                .clients
                                .get(&client_id)
                                .map(|c| c.sender.clone());
                            let room_id =
                                server_state.clients.get(&client_id).and_then(|c| c.room_id);
                            let room =
                                room_id.and_then(|id| server_state.room_senders.get(&id).cloned());
                            // On enlève la room_id du client ici pour éviter conflit
                            if let Some(client) = server_state.clients.get_mut(&client_id) {
                                client.room_id = None;
                            }
                            (room, client_sender)
                        };

                        if let Some(room) = room {
                            let msg = quit_room(room, client_id).await;

                            if let Some(msg) = msg {
                                if let ServerMessage::CloseRoom { id } = &msg {
                                    let mut server_state = state.lock().unwrap();
                                    server_state.room_senders.remove(id);
                                    println!("Room {} deleted !!", id);
                                }
                                if let Err(e) =
                                    send_to_client(&state.lock().unwrap().clients[&client_id], &msg)
                                {
                                    eprintln!(
                                        "Failed to send message to client {}: {}",
                                        client_id, e
                                    );
                                }
                            }
                        }
                        break;
                    }
                    Ok(_) => { /* autres types, ignorer ou gérer */ }
                    Err(e) => {
                        eprintln!("WebSocket error from client {}: {}", client_id, e);
                        break;
                    }
                }
            }

            {
                // Extraire client et room_sender hors du lock
                let (maybe_client, maybe_room_sender) = {
                    let mut state_guard = state.lock().unwrap();

                    let client = state_guard.clients.remove(&client_id);

                    let room_sender = client
                        .as_ref()
                        .and_then(|c| c.room_id)
                        .and_then(|room_id| state_guard.room_senders.get(&room_id).cloned());

                    // On enlève la room_id du client ici pour éviter conflit (optionnel, selon logique)
                    // Mais comme on a retiré client, plus nécessaire

                    (client, room_sender)
                };

                if let Some(client) = maybe_client {
                    if let Some(room_sender) = maybe_room_sender {
                        // Appeler async quit_room
                        let quit_msg = quit_room(room_sender, client_id).await;

                        if let Some(ServerMessage::CloseRoom { id }) = &quit_msg {
                            // Nettoyer la room_senders si la room est fermée
                            let mut state_guard = state.lock().unwrap();
                            state_guard.room_senders.remove(id);
                            println!("Room {} deleted !!", id);
                        }

                        if let Some(msg) = quit_msg {
                            // Envoyer message au client (ou log erreur)
                            if let Err(e) = send_to_client(&client, &msg) {
                                eprintln!("Failed to send message to client {}: {}", client_id, e);
                            }
                        }
                    }

                    // Toujours envoyer l'info de déconnexion au client
                    let _ = send_to_client(
                        &client,
                        &ServerMessage::Info {
                            msg: "Disconnected".to_string(),
                        },
                    );

                    println!("Client {} disconnected", client_id);
                    println!(
                        "Current clients: {:?}",
                        state.lock().unwrap().clients.keys().collect::<Vec<_>>()
                    );
                }
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
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        println!("Cleaning up inactive rooms...");

        let to_remove = {
            let mut state_guard = match state.lock() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to acquire state lock: {}", e);
                    continue;
                }
            };
            state_guard.collect_inactive_rooms()
        };

        for (room_id, room_sender) in to_remove {
            let (response_tx, response_rx) = tokio::sync::oneshot::channel();
            let _ = room_sender.send(RoomCommand::Shutdown { response_tx });
            println!("Asking inactivity to room {}", room_id);

            match response_rx.await {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<ServerMessage>(&text) {
                        Ok(ServerMessage::InternalClose { id, clients_id }) => {
                            let mut state_guard = match state.lock() {
                                Ok(s) => s,
                                Err(e) => {
                                    eprintln!("Failed to acquire state lock during cleanup: {}", e);
                                    continue;
                                }
                            };

                            // Update clients
                            for client_id in clients_id {
                                if let Some(client) = state_guard.clients.get_mut(&client_id) {
                                    client.room_id = None;
                                }
                            }

                            // Remove the room
                            state_guard.remove_room(id);
                            println!("Room {} cleaned up successfully", id);
                        }
                        Ok(ServerMessage::Error { msg }) => {
                            println!("Not removing Room");
                        }
                        Ok(_) => {
                            println!("Unexpected message...")
                        }
                        Err(e) => eprintln!("Failed to parse room closure message: {}", e),
                    }
                }
                Ok(_) => eprintln!("Received unexpected message type from room"),
                Err(e) => eprintln!("Failed to receive room shutdown response: {}", e),
            }
        }
    }
}

pub async fn inactivity_check(state: SharedServerState) {
    let mut interval = interval(Duration::from_secs(60)); // Vérifie toutes les 60s
    loop {
        interval.tick().await;
        println!("Inactivity check...");

        let inactives_clients = {
            let state_guard = match state.lock() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!(
                        "Failed to acquire state lock during inactivity check: {}",
                        e
                    );
                    continue;
                }
            };
            state_guard.get_inactives()
        };

        for client_id in inactives_clients {
            println!("Disconnecting inactive client: {}", client_id);
            let mut state_guard = match state.lock() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to acquire state lock during client removal: {}", e);
                    continue;
                }
            };

            state_guard.remove_client(&client_id);
        }
        /*let mut to_remove: Vec<Uuid> = Vec::new();
        for client in state_guard.clients.values_mut() {
            let last_hb = client.hb.load(Ordering::SeqCst);
            if now_timestamp() - last_hb > 300 {
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
        );*/
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

pub async fn join_room(
    room: mpsc::UnboundedSender<RoomCommand>,
    client: UnboundedSender<Message>, // Le type de ton sender
    client_id: Uuid,
    room_id: Uuid,
) -> Option<ServerMessage> {
    // Channel pour la réponse
    let (response_tx, response_rx) = oneshot::channel();
    // Construire la commande à envoyer au RoomActor
    let cmd = RoomCommand::JoinRoom {
        client_id,
        sender: client,
        response_tx,
    };
    // Envoyer la commande, gérer le cas d’erreur
    if room.send(cmd).is_err() {
        return Some(ServerMessage::Error {
            msg: "Room is not available.".into(),
        });
    }

    // Attendre la réponse
    match response_rx.await {
        Ok(message) => {
            if let Message::Text(text) = message {
                match serde_json::from_str(&text) {
                    Ok(ServerMessage::Joined {
                        role,
                        room_id,
                        room_status,
                        host,
                        gamemod,
                    }) => Some(ServerMessage::Joined {
                        role,
                        room_id,
                        room_status,
                        host,
                        gamemod,
                    }),
                    _ => Some(ServerMessage::Error {
                        msg: "Invalid join response".into(),
                    }),
                }
            } else {
                Some(ServerMessage::Error {
                    msg: "Expected text message".into(),
                })
            }
        }
        Err(_) => Some(ServerMessage::Error {
            msg: "No response from room".into(),
        }),
    }
}

pub async fn quit_room(
    room: mpsc::UnboundedSender<RoomCommand>,
    client_id: Uuid,
) -> Option<ServerMessage> {
    let (response_tx, response_rx) = oneshot::channel();
    let cmd = RoomCommand::PlayerQuit {
        client_id,
        response_tx,
    };

    if room.send(cmd).is_err() {
        return Some(ServerMessage::Error {
            msg: "Room is not available.".into(),
        });
    }

    println!("Quit request Sended to Room");
    match response_rx.await {
        Ok(message) => {
            if let Message::Text(text) = message {
                match serde_json::from_str(&text) {
                    Ok(ServerMessage::CloseRoom { id }) => Some(ServerMessage::CloseRoom { id }),
                    _ => Some(ServerMessage::Error {
                        msg: "Invalid quit response".into(),
                    }),
                }
            } else {
                Some(ServerMessage::Error {
                    msg: "Expected text message".into(),
                })
            }
        }
        Err(_) => Some(ServerMessage::Error {
            msg: "No response from room".into(),
        }),
    }
}
