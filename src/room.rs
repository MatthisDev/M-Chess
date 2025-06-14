use crate::{
    handler::send_game_state_to_clients,
    now_timestamp, send_to_player,
    utils::{Player, PlayerType},
};

use engine::{
    automation::ai::{Difficulty, AI},
    board,
    game::Game,
    color::Color,
};
use engine::{
    messages::ServerMessage,
    sharedenums::{GameMode, PlayerRole, RoomStatus},
};
use std::{collections::HashMap, time::Duration};
use tokio::{
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot::{self, Sender},
    },
    time::Instant,
};
use tokio_tungstenite::tungstenite::{http::response, Message};
use uuid::Uuid;

#[derive(Debug)]
pub struct Room {
    pub id: Uuid,
    pub mode: GameMode,
    pub status: RoomStatus,
    pub players: HashMap<Uuid, Player>,
    pub game: Game,
    pub created_at: Instant,
    paused: bool,
    rx: UnboundedReceiver<RoomCommand>,
    tx: UnboundedSender<RoomCommand>,
}

impl Room {
    pub fn new(
        id: Uuid,
        mode: GameMode,
        players: HashMap<Uuid, Player>,
        game: Game,
        rx: UnboundedReceiver<RoomCommand>,
        tx: UnboundedSender<RoomCommand>,
    ) -> Self {
        Self {
            id,
            mode,
            status: RoomStatus::WaitingPlayers,
            players,
            game,
            created_at: Instant::now(),
            rx,
            tx,
            paused: false,
        }
    }

    pub async fn run(&mut self) {
        println!("Room awaiting command...");
        while let Some(cmd) = self.rx.recv().await {
            match cmd {
                RoomCommand::ClientReady {
                    client_id,
                    ready,
                    response_tx,
                } => {
                    // Met à jour le joueur et envoie le message de statut au client
                    /*
                    let ready_state;
                    {

                        let player = match self.players.get_mut(&client_id) {
                            Some(p) => p,
                            None => {
                                return;
                            }
                        };

                        player.ready = !ready;
                        ready_state = player.ready;
                        println!("PLayer ready set to: {}", player.ready);

                        let _ =
                            send_to_player(player, &ServerMessage::Status { ready: ready_state });


                    }

                    // Maintenant on peut emprunter `self` à nouveau
                    let all_ready = self
                        .players
                        .values()
                        .all(|p| p.ready || p.role == PlayerRole::Spectator);

                    match self.mode {
                        GameMode::PlayerVsPlayer | GameMode::PlayerVsAI => {
                            if self.players.len() == 2 && all_ready {
                                self.status = RoomStatus::WaitingReady;
                            } else {
                                self.status = RoomStatus::WaitingPlayers;
                            }
                        }
                        GameMode::Sandbox => {
                            if self.players.len() == 1 && all_ready {
                                self.status = RoomStatus::WaitingReady;
                            } else {
                                self.status = RoomStatus::WaitingPlayers;
                            }
                        }
                        GameMode::AIvsAI => {
                            self.status = RoomStatus::WaitingReady;
                        }
                        _ => {}
                    }

                    // Diffuse le statut de la room à tous les joueurs
                    for p in self.players.values() {
                        let _ = send_to_player(
                            p,
                            &ServerMessage::RoomStatus {
                                status: self.status,
                            },
                        );
                    }
                    */
                }
                RoomCommand::JoinRoom {
                    client_id,
                    sender,
                    response_tx,
                } => {
                    if matches!(self.status, RoomStatus::Running | RoomStatus::Finished) {
                        let err = ServerMessage::Error {
                            msg: "Cannot join this room.".into(),
                        };
                        let json = serde_json::to_string(&err).unwrap();
                        let _ = response_tx.send(Message::Text(json.into()));
                    } else if self.players.contains_key(&client_id) {
                        let err = ServerMessage::Error {
                            msg: "You already joined this room.".into(),
                        };
                        let json = serde_json::to_string(&err).unwrap();
                        let _ = response_tx.send(Message::Text(json.into()));
                    } else {
                        let role = match self.mode {
                            GameMode::PlayerVsPlayer if self.players.len() == 1 => {
                                Some(PlayerRole::Black)
                            }
                            GameMode::AIvsAI | GameMode::PlayerVsPlayer => {
                                Some(PlayerRole::Spectator)
                            }
                            _ => None,
                        };
                        if let Some(role) = role {
                            self.players.insert(
                                client_id,
                                Player {
                                    id: client_id,
                                    role: role.clone(),
                                    ready: false,
                                    sender: Some(sender.clone()),
                                    kind: PlayerType::Human,
                                },
                            );

                            let resp = ServerMessage::Joined {
                                role,
                                room_id: self.id,
                                room_status: self.status,
                                host: false,
                                gamemod: self.mode.clone(),
                            };
                            let json = serde_json::to_string(&resp).unwrap();
                            let _ = response_tx.send(Message::Text(json.into()));
                        } else {
                            let err = ServerMessage::Error {
                                msg: "Unsupported or full room.".into(),
                            };
                            let json = serde_json::to_string(&err).unwrap();

                            let _ = response_tx.send(Message::Text(json.into()));
                        }
                    }
                }
                RoomCommand::StartGame { client_id } => {
                    /*
                    // Changer status, envoyer message à joueurs, démarrer IA si besoin
                    if matches!(
                        self.mode,
                        GameMode::PlayerVsPlayer | GameMode::PlayerVsAI | GameMode::Sandbox
                    ) && self.status == RoomStatus::WaitingReady
                    {
                        self.status = RoomStatus::Running;
                        for player in self.players.values() {
                            let _ = send_to_player(
                                player,
                                &ServerMessage::GameStarted {
                                    room_status: self.status,
                                    board: self.game.board.export_display_board(),
                                    turn: self.game.board.turn,
                                },
                            );
                        }
                        println!("Room {:?} game started", self.id);
                    } else if self.mode == GameMode::AIvsAI {
                        self.status = RoomStatus::Running;
                        for player in self.players.values() {
                            /*
                            let _ = send_to_player(
                                player,
                                &ServerMessage::GameStarted {
                                    room_status: self.status,
                                    board: self.game.board.export_display_board(),
                                    turn: self.game.board.turn,
                                },
                            );
                            */
                        }
                        let _ = self.tx.send(RoomCommand::AiMove);
                    }
                    */
                }
                RoomCommand::GetMoves { client_id, mv } => {
                    println!("Asking Moves in Room");
                    if self.status != RoomStatus::Running {
                        println!("room status error: {:?} instead of Running", self.status);
                        continue;
                    }
                    let player = self.players.get(&client_id);
                    if let Some(player) = player {
                        let mut t = mv.clone();

                        let mv = mv.trim().replace('"', "");

                        /*
                        let movelist = self.game.get_list_moves(mv);
                        match movelist {
                            Ok(moves) => {
                                println!("Moves: {:?}", moves);
                                send_to_player(player, &ServerMessage::LegalMoves { moves });
                            }
                            Err(s) => {
                                println!("Error: {}", s);
                                send_to_player(
                                    player,
                                    &ServerMessage::LegalMoves { moves: Vec::new() },
                                );
                            }
                        }
                        */
                    } else {
                        println!("error unwraping player");
                    }
                }
                RoomCommand::ClientMove { client_id, mv } => {
                    /*
                    let mv = mv.trim().replace('"', "");

                    let player = match self.players.get(&client_id) {
                        Some(p) => p.role.clone(),
                        None => {
                            println!("Player not found");
                            return;
                        }
                    };

                    // Étape 2 : Vérification du tour
                    if self.status != RoomStatus::Running {
                        if let Some(player) = self.players.get(&client_id) {
                            send_to_player(
                                player,
                                &ServerMessage::Error {
                                    msg: "The game hasn't started yet.".into(),
                                },
                            );
                        }
                        println!("Room status error: {:?} instead of Running", self.status);
                        continue;
                    }

                    let expected_color = self.game.board.turn;
                    let player_color = match player {
                        PlayerRole::White => Color::White,
                        PlayerRole::Black => Color::Black,
                        PlayerRole::Solo => expected_color,
                        _ => {
                            if let Some(player) = self.players.get(&client_id) {
                                send_to_player(
                                    player,
                                    &ServerMessage::Error {
                                        msg: "You are not allowed to make a move.".into(),
                                    },
                                );
                            }
                            println!("Player is not allowed to make a move");
                            return;
                        }
                    };
                    if player != PlayerRole::Solo && player_color != expected_color {
                        if let Some(player) = self.players.get(&client_id) {
                            send_to_player(
                                player,
                                &ServerMessage::Error {
                                    msg: "It's not your turn.".into(),
                                },
                            );
                        }
                        continue;
                    }

                    let move_result = self.game.make_move_algebraic(&mv);
                    match move_result {
                        Ok(_) => {
                            let turn = self.game.board.turn;
                            let king_pos = self.game.board.pieces[turn as usize][15].position;

                            println!("Moved: {}", mv);
                            // Préparer le message à diffuser
                            let state_msg = ServerMessage::State {
                                board: self.game.board.export_display_board(),
                                turn: self.game.board.turn,
                                counter: self.game.board.counter,
                                incheck: if self.game.board.is_attacked(&king_pos, turn) {
                                    Some(turn)
                                } else {
                                    None
                                },
                            };

                            // Diffuser à tous les joueurs (accès immuable)
                            for player in self.players.values() {
                                send_to_player(player, &state_msg);
                            }

                            // Vérifier si la partie est terminée
                            if self.game.board.is_game_over() {
                                let result = if self.game.board.is_checkmate(self.game.board.turn) {
                                    format!(
                                        "Checkmate! {:?} wins.",
                                        match self.game.board.turn {
                                            Color::White => Color::Black,
                                            Color::Black => Color::White,
                                        }
                                    )
                                } else {
                                    "Draw!".to_string()
                                };

                                // Update du status
                                self.status = RoomStatus::Finished;

                                // Envoyer le message de fin
                                let game_over_msg = ServerMessage::GameOver {
                                    room_status: self.status,
                                    result,
                                };

                                for player in self.players.values() {
                                    send_to_player(player, &game_over_msg);
                                }
                            }

                            // Si PlayerVsAI, envoyer un nouveau AiMove
                            if self.mode == GameMode::PlayerVsAI {
                                let _ = self.tx.send(RoomCommand::AiMove);
                            }
                        }
                        Err(e) => {
                            if let Some(player) = self.players.get(&client_id) {
                                send_to_player(
                                    player,
                                    &ServerMessage::Error {
                                        msg: format!("Invalid move: {}", e),
                                    },
                                );
                            }
                        }
                    }
                    */
                }
                RoomCommand::AiMove => {
                    /*
                    let ai_player_opt = self.players.values().find(|p| match &p.kind {
                        PlayerType::Ai { ai } => ai.color == self.game.board.turn,
                        _ => false,
                    });
                    if !self.paused {
                        if let Some(ai_player) = ai_player_opt {
                            if let PlayerType::Ai { ai } = &ai_player.kind {
                                let ai = ai.clone();
                                let board = self.game.board.clone();
                                let tx = self.tx.clone();
                                // Calcul du coup de l'IA
                                tokio::spawn(async move {
                                    let mv = ai.get_best_move(&board); // Peut être long

                                    if let Some(mv) = mv {
                                        // Reviens dans la loop de la room avec le résultat
                                        let mv = format!(
                                            "{}->{}",
                                            mv.0.to_algebraic(),
                                            mv.1.to_algebraic()
                                        );
                                        tx.send(RoomCommand::AIApplyMove { mv });
                                    }
                                });
                            }
                        }
                    }
                    */
                }
                RoomCommand::AIApplyMove { mv } => {
                    /*
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    match self.game.make_move_algebraic(&mv) {
                        Ok(_) => {
                            let turn = self.game.board.turn;
                            let king_pos = self.game.board.pieces[turn as usize][15].position;
                            println!("Moved");
                            for player in self.players.values() {
                                send_to_player(
                                    player,
                                    &ServerMessage::State {
                                        board: self.game.board.export_display_board(),
                                        turn: self.game.board.turn,
                                        counter: self.game.board.counter,
                                        incheck: if self.game.board.is_attacked(&king_pos, turn) {
                                            Some(turn)
                                        } else {
                                            None
                                        },
                                    },
                                );
                            }

                            if self.game.board.is_game_over() {
                                let result = if self.game.board.is_checkmate(self.game.board.turn) {
                                    format!(
                                        "Checkmate! {:?} wins.",
                                        match self.game.board.turn {
                                            Color::White => Color::Black,
                                            Color::Black => Color::White,
                                        }
                                    )
                                } else {
                                    "Draw!".to_string()
                                };

                                self.status = RoomStatus::Finished;
                                let game_over_msg = ServerMessage::GameOver {
                                    room_status: self.status,
                                    result: result.clone(),
                                };

                                for player in self.players.values() {
                                    send_to_player(player, &game_over_msg);
                                }
                                println!("Game Over: {}", result);
                                continue;
                            }

                            if self.mode == GameMode::PlayerVsAI {
                                self.tx.send(RoomCommand::AiMove);
                            }
                        }
                        Err(e) => {
                            println!("Not moved");
                            panic!()
                        }
                    }
                    */
                    /*
                    // Vérifier si c'est encore à une IA de jouer
                    let next_ai_turn = self.players.values().any(|p| match &p.kind {
                        PlayerType::Ai { ai } => ai.color == self.game.board.turn,
                        _ => false,
                    });

                    if next_ai_turn {
                        // Planifie le prochain coup IA
                        let _ = self.tx.send(RoomCommand::AiMove);
                    }
                    */
                }
                RoomCommand::PlayerQuit { client_id } => {
                    /*
                    println!("A player Want to quit");
                    let mut role = None;
                    // On enlève le joueur
                    if let Some(player) = self.players.remove(&client_id) {
                        send_to_player(&player, &ServerMessage::QuitGame);
                        role = Some(player.role);
                    }
                    println!("Client {} removed from room {}", client_id, self.id);

                    if let Some(role) = role
                    // Si PvP et en cours => victoire par forfait
                    {
                        if self.mode == GameMode::PlayerVsPlayer
                            && self.status == RoomStatus::Running
                            && role != PlayerRole::Spectator
                        {
                            if let Some(winner) = self
                                .players
                                .values()
                                .find(|p| matches!(p.role, PlayerRole::White | PlayerRole::Black))
                            {
                                self.status = RoomStatus::Finished;
                                let msg = ServerMessage::GameOver {
                                    room_status: self.status,
                                    result: format!(
                                        "A player quit the game!!\nVictory by forfeit for {:?} !!!",
                                        winner.role
                                    ),
                                };
                                for p in self.players.values() {
                                    let _ = send_to_player(p, &msg);
                                }
                                println!(
                                    "Player {} quit the game. Victory by forfeit for {:?}.",
                                    client_id, winner.role
                                );
                            }
                        }
                        if self.mode == GameMode::AIvsAI {
                            // On arrête la partie
                            self.status = RoomStatus::Finished;
                            self.paused = true;
                        }
                    }
                    */
                }
                RoomCommand::Shutdown { response_tx } => {
                    /*
                    println!("Room internal inactivity check");
                    let now = Instant::now();
                    let timeout = Duration::from_secs(300); // 5 min

                    let mut close = false;
                    // S'il reste des humains ?
                    let was_last_human = !self
                        .players
                        .values()
                        .any(|p| matches!(p.kind, PlayerType::Human));

                    let response = if self.status == RoomStatus::Running {
                        Message::Text(
                            serde_json::to_string(&ServerMessage::Error {
                                msg: "Running".into(),
                            })
                            .unwrap()
                            .into(),
                        )
                    } else if was_last_human || self.players.is_empty() {
                        println!("Room is empty, closing it.");
                        close = true;
                        Message::Text(
                            serde_json::to_string(&&ServerMessage::InternalClose {
                                id: self.id,
                                clients_id: Vec::new(),
                            })
                            .unwrap()
                            .into(),
                        )
                    } else {
                        let inactive_too_long = now.duration_since(self.created_at) > timeout;
                        let empty = self.players.is_empty();

                        if empty || inactive_too_long {
                            let mut clients_id = Vec::new();
                            println!("Room Detected as inactive");
                            for player in self.players.values() {
                                clients_id.push(player.id);
                                send_to_player(player, &ServerMessage::QuitGame);
                            }
                            close = true;
                            Message::Text(
                                serde_json::to_string(&ServerMessage::InternalClose {
                                    id: self.id,
                                    clients_id,
                                })
                                .unwrap()
                                .into(),
                            )
                        } else {
                            println!("Room always active");
                            Message::Text(
                                serde_json::to_string(&ServerMessage::Error {
                                    msg: "Running".into(),
                                })
                                .unwrap()
                                .into(),
                            )
                        }
                    };


                    response_tx.send(response);
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    if close {
                        break;
                    } else {
                        println!("Room {} is still active", self.id);
                        continue;
                    }
                    */
                }
                RoomCommand::StartSandboxGame => {
                    /*
                    if self.mode == GameMode::Sandbox {
                        self.status = RoomStatus::Running;
                        //TODO Check pour la config minimale (2 rois, ...)
                        send_game_state_to_clients(self);
                        for p in self.players.values() {
                            let _ = send_to_player(
                                p,
                                &ServerMessage::GameStarted {
                                    room_status: self.status,
                                    board: self.game.board.export_display_board(),
                                    turn: self.game.board.turn,
                                },
                            );
                        }
                    }
                    */
                }
                RoomCommand::AddPiece {
                    pos,
                    piece,
                    client_id,
                } => {
                    /*
                    if self.mode == GameMode::Sandbox
                        && matches!(
                            self.status,
                            RoomStatus::WaitingReady | RoomStatus::WaitingPlayers
                        )
                    {
                        self.game.board.add_piece(&format!("{}{}", piece, pos));
                        send_game_state_to_clients(self);
                        if let Some(player) = self.players.get(&client_id) {
                            let _ = send_to_player(
                                player,
                                &ServerMessage::SandboxPieceAdded { piece, pos },
                            );
                        }
                    }
                    */
                }
                RoomCommand::Pause { client_id } => {

                    /*
                    let player = match self.players.get(&client_id) {
                        Some(p) => p,
                        None => return,
                    };
                    if self.mode != GameMode::AIvsAI {
                        send_to_player(
                            player,
                            &ServerMessage::Error {
                                msg: "Pause only available in AI vs AI mode".to_string(),
                            },
                        );
                    }

                    match self.status {
                        RoomStatus::Running => {
                            self.status = RoomStatus::Paused;
                            self.paused = true;
                            send_to_player(
                                player,
                                &ServerMessage::PauseGame {
                                    room_status: self.status,
                                },
                            );
                        }
                        RoomStatus::Paused => {
                            self.status = RoomStatus::Running;
                            self.paused = false;
                            send_to_player(
                                player,
                                &ServerMessage::PauseGame {
                                    room_status: self.status,
                                },
                            );
                            self.tx.send(RoomCommand::AiMove);
                        }
                        _ => continue, // Ignore pause in other statuses
                    }
                    */
                }
            }
        }
        println!("Room {} stopped", self.id);
    }
}

pub enum RoomCommand {
    ClientMove {
        client_id: Uuid,
        mv: String,
    },
    ClientReady {
        client_id: Uuid,
        ready: bool,
        response_tx: Sender<Option<Message>>,
    },
    StartGame {
        client_id: Uuid,
    },
    JoinRoom {
        client_id: Uuid,
        sender: UnboundedSender<Message>,
        response_tx: Sender<Message>,
    },
    GetMoves {
        client_id: Uuid,
        mv: String,
    },
    AiMove,
    AIApplyMove {
        mv: String,
    },
    PlayerQuit {
        client_id: Uuid,
    },
    Shutdown {
        response_tx: Sender<Message>,
    },
    StartSandboxGame,
    AddPiece {
        pos: String,
        piece: String,
        client_id: Uuid,
    },
    Pause {
        client_id: Uuid,
    },
}
