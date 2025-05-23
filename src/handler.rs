use futures::{SinkExt, StreamExt};
use game_lib::automation::ai::{Difficulty, AI};
use game_lib::game::Game;
use game_lib::piece::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, mpsc::UnboundedSender};
use tokio::time::{interval, Duration, Instant};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Message, Utf8Bytes},
};
use uuid::Uuid;

use crate::messages::{ClientMessage, ServerMessage};
use crate::sharedenums::{GameMode, PlayerRole, RoomStatus};
use crate::structures::{Player, PlayerType, Room, ServerState, SharedServerState};
use crate::{send_to_client, send_to_player};

pub fn to_player_role(color: Color) -> PlayerRole {
    match color {
        Color::White => PlayerRole::White,
        Color::Black => PlayerRole::Black,
    }
}

// Create a Room and add the client who request it
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
            let player_id = Uuid::new_v4();
            players.insert(
                player_id,
                Player {
                    id: player_id,
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
            println!("{:?}", players);
            PlayerRole::White
        }
        GameMode::AIvsAI => {
            let player_id = Uuid::new_v4();
            players.insert(
                player_id,
                Player {
                    id: player_id,
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
            let player_id = Uuid::new_v4();
            players.insert(
                player_id,
                Player {
                    id: player_id,
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
            role: role.clone(),
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
    println!("returning from creating room");

    Some(ServerMessage::Joined {
        role,
        room_id,
        room_status: status,
        host: true,
    })
}

//Join a room using the room id
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
        GameMode::AIvsAI | GameMode::PlayerVsPlayer => PlayerRole::Spectator,
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

    Some(ServerMessage::Joined {
        role,
        room_id,
        room_status: room.status,
        host: false,
    })
}

//Handle the client status to launch a game
// Only For PvP and PvAi
pub fn handle_set_ready(
    client_id: Uuid,
    server_state: &Arc<Mutex<ServerState>>,
    client_state: bool,
) {
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

    player.ready = !client_state;

    let _ = send_to_player(
        player,
        &ServerMessage::Status {
            ready: player.ready,
        },
    );

    let all_ready = room.players.values().all(|p| p.ready);
    match room.mode {
        GameMode::PlayerVsPlayer | GameMode::PlayerVsAI => {
            if room.players.len() == 2 && all_ready {
                room.status = RoomStatus::WaitingReady;
            }
        }
        GameMode::Sandbox => {
            if room.players.len() == 1 && all_ready {
                room.status = RoomStatus::WaitingReady;
            }
        }
        _ => {}
    }
    for (player) in room.players.values() {
        let _ = send_to_player(
            player,
            &ServerMessage::RoomStatus {
                status: room.status,
            },
        );
    }
}

//handle game starting request
pub fn handle_start_game(client_id: Uuid, server_state: &Arc<Mutex<ServerState>>) {
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

    if matches!(
        room.mode,
        GameMode::PlayerVsPlayer | GameMode::PlayerVsAI | GameMode::Sandbox
    ) && matches!(
        room.status,
        RoomStatus::WaitingReady | RoomStatus::WaitingPlayers
    ) {
        if room.status == RoomStatus::WaitingReady {
            room.status = RoomStatus::Running;
            for player in room.players.values() {
                let _ = send_to_player(
                    player,
                    &ServerMessage::GameStarted {
                        room_status: room.status,
                        board: room.game.board.export_display_board(),
                        turn: room.game.board.turn,
                    },
                );
            }
            println!("Room {:?} game started (PvP)", room.id);
        }
    } else if room.mode == GameMode::AIvsAI {
        room.status = RoomStatus::Running;
        // Release lock before async spawn
        tokio::spawn(run_ai_vs_ai_game(room_id, Arc::clone(server_state)));
    }
}

//Handle PossibleMove request
pub fn handle_get_moves(client_id: Uuid, mv: String, server_state: &Arc<Mutex<ServerState>>) {
    let room_id;
    {
        let state = server_state.lock().unwrap();
        if let Some(client) = state.clients.get(&client_id) {
            if let Some(id) = client.room_id {
                room_id = id;
            } else {
                return;
            }
        } else {
            return;
        }
    };

    let mut state = server_state.lock().unwrap();
    let room = state.rooms.get_mut(&room_id);
    if let Some(room) = room {
        if room.status != RoomStatus::Running {
            println!("room status error: {:?} instead of Running", room.status);
            return;
        }
        let player = room.players.get(&client_id);
        if let Some(player) = player {
            let mut t = mv.clone();

            let mv = mv.trim().replace('"', "");
            let movelist = room.game.get_list_moves(mv);
            room.game.board.print_board();
            match movelist {
                Ok(moves) => {
                    println!("Moves: {:?}", moves);
                    send_to_player(player, &ServerMessage::LegalMoves { moves });
                }
                Err(s) => {
                    println!("Error: {}", s);
                    send_to_player(player, &ServerMessage::LegalMoves { moves: Vec::new() });
                }
            }
        } else {
            println!("error unwraping player");
        }
    } else {
        println!("Room unwrap error");
    }
}

//Handle player move check if player is a player and if the move is valid
// If the move is valid, send the new game state to all players (Spectators included)
// If the game is over, send the game over message to all players
// If the move is invalid, send an error message to the player
// If the game is in sandbox mode, always allow the move
//--------------------
//NOT FOR AIvAI MODE
//--------------------
pub fn handle_move(
    client_id: Uuid,
    mv: String,
    server_state: &Arc<Mutex<ServerState>>,
) -> Option<ServerMessage> {
    let mv = mv.trim().replace('"', "");

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

    if room.mode == GameMode::Sandbox {
        // Toujours autoriser
        match room.game.make_move_algebraic(&mv) {
            Ok(_) => {
                send_game_state_to_clients(room);
                if room.game.board.is_game_over() {
                    handle_game_over(room, "Sandbox Game Over");
                }
            }
            Err(e) => {
                return Some(ServerMessage::Error {
                    msg: format!("Invalid move: {}", e),
                });
            }
        }
        return None;
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

//Make an AI move if it's the AI's turn
// If the AI makes a move, send the new game state to all players (Spectators included)
// If the game is over, send the game over message to all players
// If the AI cannot make a move, send an error message to the player |----Should Not Happen----|
fn handle_ai_turn(room: &mut Room) {
    let next_color = room.game.board.turn;
    let ai_player = room.players.values().find(|p| match &p.kind {
        PlayerType::Ai { ai } => ai.color == next_color,
        _ => false,
    });

    if let Some(p) = ai_player {
        if let PlayerType::Ai { ai } = &p.kind {
            println!("AI: {:?}", ai);
            if let Some((from, to)) = ai.get_best_move(&room.game.board) {
                let ai_mv = format!("{}->{}", from.to_algebraic(), to.to_algebraic());
                if room.game.make_move_algebraic(&ai_mv).is_ok() {
                    println!("moved");
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
}

//---------------------
//  ONLY AIvAI MODE
//---------------------
//AIvAI game loop
// This function runs in a loop, making moves for both AI players until the game is over
// It checks the game state and sends updates to all players in the room
// If the game is over, it breaks the loop
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

        let (ai_move, board_snapshot, turn, player_ids) = {
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
                room.game.board.turn,
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
                        turn,
                    },
                );
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

//Send the board and the turn to all players in the room
// This function is called after every move to update the game state
pub fn send_game_state_to_clients(room: &Room) {
    let board = room.game.board.export_display_board();
    let turn = room.game.board.turn;

    for player in room.players.values() {
        if let Some(sender) = &player.sender {
            let _ = sender.send(Message::Text(
                serde_json::to_string(&ServerMessage::State {
                    board: board.clone(),
                    turn,
                })
                .unwrap()
                .into(),
            ));
        }
    }
}

//Handle the game over state
// This function is called when the game is over to update the game state
// It sends the game over message to all players in the room
// It also updates the room status to Finished
fn handle_game_over(room: &mut Room, reason: &str) {
    room.status = RoomStatus::Finished;
    for player in room.players.values() {
        if let Some(sender) = &player.sender {
            let _ = sender.send(Message::Text(
                serde_json::to_string(&ServerMessage::GameOver {
                    room_status: room.status,
                    result: reason.to_string(),
                })
                .unwrap()
                .into(),
            ));
        }
    }
}

//-----------------
//  SANDBOX MODE
//  AIvAI MODE
//-----------------
// This function is called when the game is paused or resumed
// It updates the room status to Paused or Running
// It sends a message to the player in the room
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
            Some(ServerMessage::PauseGame {
                room_status: room.status,
            })
        }
        RoomStatus::Paused => {
            room.status = RoomStatus::Running;
            Some(ServerMessage::PauseGame {
                room_status: room.status,
            })
        }
        _ => None,
    }
}

//-----------------
//  QUIT
//-----------------
// This function is called when the player quits the game
// It removes the player from the room and updates the game state
// It also deletes the room if there are no players left
// It sends a message to all players remainging in the room
//Confirmation message to the player quitting
pub fn handle_quit(client_id: Uuid, server_state: &Arc<Mutex<ServerState>>) {
    let (room_id_opt, was_last_human);

    // --- Étape 1 : retirer le joueur de la room et obtenir son room_id ---
    {
        let mut state = server_state.lock().unwrap();

        // Récupération et mutation du client
        let client_opt = state.clients.get_mut(&client_id);
        if client_opt.is_none() {
            return;
        }
        let client = client_opt.unwrap();

        let Some(room_id) = client.room_id else {
            return;
        };

        // Pour éviter le conflit, on copie l'UUID de room ici
        room_id_opt = Some(room_id);

        // On va muter `client`, donc on termine ici avant de re-muter le state
        client.room_id = None;
    }

    {
        let mut state = server_state.lock().unwrap();

        let Some(room_id) = room_id_opt else { return };
        let Some(room) = state.rooms.get_mut(&room_id) else {
            return;
        };

        // On enlève le joueur
        if let Some(player) = room.players.remove(&client_id) {
            send_to_player(&player, &ServerMessage::QuitGame);
        }
        println!("Client {} removed from room {}", client_id, room_id);

        // S'il reste des humains ?
        was_last_human = !room
            .players
            .values()
            .any(|p| matches!(p.kind, PlayerType::Human));

        if was_last_human {
            state.rooms.remove(&room_id);
            println!("Room {} deleted: no human players remaining.", room_id);
            return;
        }

        // Si PvP et en cours => victoire par forfait
        if room.mode == GameMode::PlayerVsPlayer && room.status == RoomStatus::Running {
            if let Some(winner) = room
                .players
                .values()
                .find(|p| matches!(p.role, PlayerRole::White | PlayerRole::Black))
            {
                room.status = RoomStatus::Finished;
                let msg = ServerMessage::GameOver {
                    room_status: room.status,
                    result: format!(
                        "A player quit the game!!\nVictory by forfeit for {:?} !!!",
                        winner.role
                    ),
                };
                for p in room.players.values() {
                    let _ = send_to_player(p, &msg);
                }
                println!(
                    "Player {} quit the game. Victory by forfeit for {:?}.",
                    client_id, winner.role
                );
            }
        }

        // Si la room est vide
        if room.players.is_empty() {
            state.rooms.remove(&room_id);
            println!("Room {} deleted: no players remaining.", room_id);
        }
    }
}
