use futures::{SinkExt, StreamExt};
use game_lib::automation::ai::{Difficulty, AI};
use game_lib::game::Game;
use game_lib::piece::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
use sharedenums::{GameMode, PlayerRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomStatus {
    WaitingPlayers,
    WaitingReady,
    ReadyToStart,
    Running,
    Finished,
    Paused,
}

pub struct Client {
    pub id: Uuid,
    pub room_id: Option<Uuid>,
    pub sender: UnboundedSender<Message>,
    pub ready: bool,
    pub last_active: Instant,
}

#[derive(Debug, Clone)]
pub enum PlayerType {
    Human,
    Ai { ai: AI }, // tu peux même ajouter un champ `name`, `strategy`, etc.
}

pub struct Player {
    pub id: Uuid,
    pub role: PlayerRole,
    pub ready: bool,
    pub sender: Option<UnboundedSender<Message>>,
    pub kind: PlayerType,
}

pub struct Room {
    pub id: Uuid,
    pub mode: GameMode,
    pub status: RoomStatus,
    pub players: HashMap<Uuid, Player>,
    pub game: Game,
    pub created_at: Instant,
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
                        last_active: Instant::now(),
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
    if let Some(sender) = &player.sender {
        sender
            .send(Message::Text(serialized.into()))
            .map_err(|e| format!("Failed to send to player: {}", e))
    } else {
        Err("Player is an Ai".to_string())
    }
}

pub fn to_player_role(color: Color) -> PlayerRole {
    match color {
        Color::White => PlayerRole::White,
        Color::Black => PlayerRole::Black,
    }
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
        GameMode::PlayerVsAI => {
            players.insert(
                client_id,
                Player {
                    id: Uuid::new_v4(),
                    role: PlayerRole::Black,
                    ready: true,
                    sender: None,
                    kind: PlayerType::Ai {
                        ai: AI {
                            difficulty: difficulty.unwrap_or(Difficulty::Easy),
                            color: Color::Black,
                        },
                    },
                },
            );
            PlayerRole::White
        }
        GameMode::AIvsAI => {
            players.insert(
                client_id,
                Player {
                    id: Uuid::new_v4(),
                    role: PlayerRole::Black,
                    ready: true,
                    sender: None,
                    kind: PlayerType::Ai {
                        ai: AI {
                            difficulty: difficulty.clone().unwrap_or(Difficulty::Easy),
                            color: Color::Black,
                        },
                    },
                },
            );

            players.insert(
                client_id,
                Player {
                    id: Uuid::new_v4(),
                    role: PlayerRole::White,
                    ready: true,
                    sender: None,
                    kind: PlayerType::Ai {
                        ai: AI {
                            difficulty: difficulty.unwrap_or(Difficulty::Easy),
                            color: Color::White,
                        },
                    },
                },
            );

            PlayerRole::Spectator
        }
        GameMode::Sandbox => PlayerRole::Solo,
    };

    players.insert(
        client_id,
        Player {
            id: client_id,
            role,
            ready: false,
            sender: Some(client.sender.clone()),
            kind: PlayerType::Human,
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
            created_at: Instant::now(),
        },
    );

    if let Some(c) = server_state.clients.get_mut(&client_id) {
        c.room_id = Some(room_id);
    }
    println!("returning from craeting room");

    Some(ServerMessage::Joined {
        role: PlayerRole::White,
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
            role: role.clone(),
            ready: false,
            sender: Some(client.sender.clone()),
            kind: PlayerType::Human,
        },
    );

    if let Some(c) = server_state.clients.get_mut(&client_id) {
        c.room_id = Some(room_id);
    }

    Some(ServerMessage::Joined { role, room_id })
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

pub fn handle_move(
    client_id: Uuid,
    mv: String,
    server_state: &Arc<Mutex<ServerState>>,
) -> Option<ServerMessage> {
    let room_id;
    {
        let state = server_state.lock().unwrap();
        room_id = state.clients.get(&client_id)?.room_id?;
    }

    let mut state = server_state.lock().unwrap();
    let room = state.rooms.get_mut(&room_id)?;
    if room.status != RoomStatus::Running {
        return Some(ServerMessage::Error {
            msg: "The game hasn't started yet.".into(),
        });
    }

    let player = room.players.get(&client_id)?;
    let expected_color = room.game.board.turn;
    let player_color = match player.role {
        PlayerRole::White => Color::White,
        PlayerRole::Black => Color::Black,
        _ => {
            return Some(ServerMessage::Error {
                msg: "You are not allowed to make a move.".into(),
            });
        }
    };

    if player_color != expected_color {
        return Some(ServerMessage::Error {
            msg: "It's not your turn.".into(),
        });
    }

    match room.game.make_move_algebraic(&mv) {
        Ok(_) => {
            send_game_state_to_clients(room);

            if room.game.board.is_game_over() {
                let result = if room.game.board.is_checkmate(expected_color) {
                    format!(
                        "Checkmate! {:?} wins.",
                        match expected_color {
                            Color::White => Color::Black,
                            Color::Black => Color::White,
                        }
                    )
                } else {
                    "Draw!".to_string()
                };
                handle_game_over(room, &result);
                return None;
            }

            handle_ai_turn(room);
            None
        }
        Err(e) => Some(ServerMessage::Error {
            msg: format!("Invalid move: {}", e),
        }),
    }
}

pub fn handle_ai_turn(room: &mut Room) {
    let next_color = room.game.board.turn;
    let ai_player = room.players.values().find(
        |p| matches!((&p.kind, p.role.clone()), (PlayerType::Ai { ai }, _) if ai.color == next_color),
    );

    if let Some(Player {
        kind: PlayerType::Ai { ai },
        ..
    }) = ai_player
    {
        if let Some((from, to)) = ai.get_best_move(&room.game.board) {
            let ai_mv = format!("{}->{}", from.to_algebraic(), to.to_algebraic());
            if room.game.make_move_algebraic(&ai_mv).is_ok() {
                send_game_state_to_clients(room);

                if room.game.board.is_game_over() {
                    let result = if room.game.board.is_checkmate(next_color) {
                        format!(
                            "Checkmate! {:?} wins.",
                            match next_color {
                                Color::White => Color::Black,
                                Color::Black => Color::White,
                            }
                        )
                    } else {
                        "Draw!".to_string()
                    };
                    handle_game_over(room, &result);
                }
            }
        }
    }
}

pub fn send_game_state_to_clients(room: &Room) {
    let board = room.game.board.export_display_board();
    let turn = if room.game.board.turn == Color::White {
        "White".to_string()
    } else {
        "Black".to_string()
    };

    for player in room.players.values() {
        if let Some(sender) = &player.sender {
            let _ = sender.send(Message::Text(
                serde_json::to_string(&ServerMessage::State {
                    board: board.clone(),
                    turn: turn.clone(),
                })
                .unwrap()
                .into(),
            ));
        }
    }
}

pub fn handle_game_over(room: &mut Room, reason: &str) {
    room.status = RoomStatus::Finished;
    for player in room.players.values() {
        if let Some(sender) = &player.sender {
            let _ = sender.send(Message::Text(
                serde_json::to_string(&ServerMessage::GameOver {
                    result: reason.to_string(),
                })
                .unwrap()
                .into(),
            ));
        }
    }
}

pub async fn run_ai_vs_ai_game(room_id: Uuid, server_state: SharedServerState) {
    loop {
        {
            let state = server_state.lock().unwrap();
            if let Some(room) = state.rooms.get(&room_id) {
                if room.status != RoomStatus::Running {
                    break;
                }
            } else {
                break;
            }
        }

        let (ai_move, board_snapshot, player_ids) = {
            let mut state = server_state.lock().unwrap();
            let room = match state.rooms.get_mut(&room_id) {
                Some(r) => r,
                None => return,
            };

            if room.game.board.is_game_over() {
                handle_game_over(room, "Game over");
                return;
            }

            let turn_color = room.game.board.turn;
            let role = to_player_role(turn_color);
            let ai_player = room.players.values().find(|p| p.role == role).unwrap();

            let PlayerType::Ai { ai } = &ai_player.kind else {
                return;
            };

            let m = ai.get_best_move(&room.game.board).unwrap();
            let algebraic = format!("{}->{}", m.0.to_algebraic(), m.1.to_algebraic());

            if room.game.make_move_algebraic(&algebraic).is_err() {
                return;
            }

            (
                Some(algebraic),
                room.game.board.export_display_board(),
                room.players
                    .values()
                    .filter_map(|p| p.sender.as_ref().map(|_| p.id))
                    .collect::<Vec<Uuid>>(),
            )
        };

        for player_id in player_ids {
            if let Some(client) = server_state.lock().unwrap().clients.get(&player_id) {
                let _ = send_to_client(
                    client,
                    &ServerMessage::State {
                        board: board_snapshot.clone(),
                        turn: "White".to_string(),
                    },
                );
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

pub fn toggle_pause_game(room_id: Uuid, server_state: &SharedServerState) -> Option<ServerMessage> {
    let mut state = server_state.lock().unwrap();
    let room = state.rooms.get_mut(&room_id)?;
    if room.mode != GameMode::AIvsAI {
        return Some(ServerMessage::Error {
            msg: "Pause only available in AI vs AI mode".to_string(),
        });
    }

    match room.status {
        RoomStatus::Running => {
            room.status = RoomStatus::Paused;
            Some(ServerMessage::Info {
                msg: "Game paused".to_string(),
            })
        }
        RoomStatus::Paused => {
            room.status = RoomStatus::Running;
            Some(ServerMessage::Info {
                msg: "Game resumed".to_string(),
            })
        }
        _ => None,
    }
}

pub async fn cleanup_inactive_rooms(server_state: SharedServerState) {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        let mut state = server_state.lock().unwrap();
        let now = Instant::now();

        let inactive_rooms: Vec<_> = state
            .rooms
            .iter()
            .filter(|(_, room)| {
                matches!(
                    room.status,
                    RoomStatus::WaitingPlayers | RoomStatus::WaitingReady
                ) && now.duration_since(room.created_at) > Duration::from_secs(300)
            })
            .map(|(id, _)| *id)
            .collect();

        for room_id in inactive_rooms {
            println!("Cleaning up inactive room: {}", room_id);
            state.rooms.remove(&room_id);
        }
    }
}
