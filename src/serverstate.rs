use crate::{
    now_timestamp,
    room::{Room, RoomCommand},
    utils::{Client, Player, PlayerType},
};
use game_lib::{
    automation::ai::{Difficulty, AI},
    game::Game,
    piece::Color,
};
use game_lib::{
    messages::ServerMessage,
    sharedenums::{GameMode, PlayerRole, RoomStatus},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::{
    sync::{
        mpsc::{self, UnboundedSender},
        oneshot::{self, Sender},
    },
    time::Instant,
};
use uuid::Uuid;

pub struct ServerState {
    pub clients: HashMap<Uuid, Client>,
    pub room_senders: HashMap<Uuid, UnboundedSender<RoomCommand>>,
}
pub type SharedServerState = Arc<Mutex<ServerState>>;

impl ServerState {
    pub fn create_room(
        &mut self,
        client_id: Uuid,
        mode: GameMode,
        difficulty: Option<Difficulty>,
    ) -> Option<ServerMessage> {
        /*
        let room_id = Uuid::new_v4();
        let (tx, rx) = mpsc::unbounded_channel();
        let game = Game::init(matches!(mode, GameMode::Sandbox));
        let mut client = self.clients.get_mut(&client_id)?;
        client.room_id = Some(room_id);

        let mut players = HashMap::new();
        let status = match mode {
            GameMode::PlayerVsPlayer => RoomStatus::WaitingPlayers,
            _ => RoomStatus::WaitingReady,
        };
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
        println!("Here!!");
        let mut room_actor = Room::new(room_id, mode.clone(), players, game, rx, tx.clone());

        // Lancer la task asynchrone pour gérer la room
        tokio::spawn(async move {
            room_actor.run().await;
        });

        self.room_senders.insert(room_id, tx);
        println!("Created");
        Some(ServerMessage::Joined {
            role,
            room_id,
            room_status: status,
            host: true,
            gamemod: mode,
        })
        */
        None
    }

    pub fn set_player_ready(&mut self, client_id: Uuid, client_state: bool) {
        let client = match self.clients.get(&client_id) {
            Some(r) => r,
            None => return,
        };

        let room_id = match client.room_id {
            Some(r) => r,
            None => return,
        };

        let room = match self.room_senders.get(&room_id) {
            Some(r) => r,
            None => return,
        };

        let (response_tx, response_rx) = oneshot::channel();
        let cmd = RoomCommand::ClientReady {
            client_id,
            ready: client_state,
            response_tx,
        };
        room.send(cmd).is_err();
    }

    pub fn start_game(&mut self, client_id: Uuid) {
        let client = match self.clients.get(&client_id) {
            Some(c) => c,
            None => return,
        };
        let room_id = match client.room_id {
            Some(id) => id,
            None => return,
        };
        let room = match self.room_senders.get_mut(&room_id) {
            Some(r) => r,
            None => return,
        };

        let cmd = RoomCommand::StartGame { client_id };

        room.send(cmd);
    }

    pub fn get_moves(&mut self, client_id: Uuid, mv: String) {
        let room_id;
        if let Some(client) = self.clients.get(&client_id) {
            if let Some(id) = client.room_id {
                room_id = id;
            } else {
                return;
            }
        } else {
            return;
        };

        let room = match self.room_senders.get_mut(&room_id) {
            Some(r) => r,
            None => return,
        };
        let mv: String = mv.trim().replace('"', "");
        let cmd = RoomCommand::GetMoves { client_id, mv };
        room.send(cmd);
    }

    pub fn make_move(&mut self, client_id: Uuid, mv: String) {
        let mv = mv.trim().replace('"', "");
        let room_id;
        {
            if let Some(client) = self.clients.get(&client_id) {
                if let Some(id) = client.room_id {
                    room_id = id;
                } else {
                    return;
                }
            } else {
                return;
            }
        }
        let room = match self.room_senders.get(&room_id) {
            Some(r) => r,
            None => return,
        };
        let cmd = RoomCommand::ClientMove { client_id, mv };
        room.send(cmd);
    }

    pub fn start_sandbox_game(&mut self, room_id: Uuid) {
        if let Some(room) = self.room_senders.get(&room_id) {
            room.send(RoomCommand::StartSandboxGame);
        }
    }

    pub fn add_piece(&mut self, room_id: Uuid, pos: String, piece: String, client_id: Uuid) {
        if let Some(room) = self.room_senders.get(&room_id) {
            room.send(RoomCommand::AddPiece {
                pos,
                piece,
                client_id,
            });
        }
    }

    pub fn get_inactives(&self) -> Vec<Uuid> {
        self.clients
            .iter()
            .filter_map(|(&id, client)| {
                if now_timestamp() - client.hb.load(std::sync::atomic::Ordering::SeqCst) > 300 {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()

        /*    // Appeler quit() pour chaque client inactif
         for client_id in &to_remove {
             self.quit(*client_id).await; // Appel async
         }

         //Supprimer les clients du HashMap
         for id in &to_remove {
             self.clients.remove(id);
             println!("Removed inactive client {:?}", id);
         }

        */
    }

    pub fn remove_client(&mut self, client_id: &Uuid) {
        self.clients.remove(client_id);
    }

    pub fn collect_inactive_rooms(&mut self) -> Vec<(Uuid, mpsc::UnboundedSender<RoomCommand>)> {
        let mut to_remove = vec![];

        for (room_id, sender) in self.room_senders.iter() {
            to_remove.push((*room_id, sender.clone()));
        }

        to_remove
    }

    pub fn remove_room(&mut self, room_id: Uuid) {
        self.room_senders.remove(&room_id);
    }

    pub fn toggle_pause_game(&mut self, room_id: Uuid, client_id: Uuid) {
        let room = match self.room_senders.get(&room_id) {
            Some(s) => s,
            None => {
                return;
            }
        };

        room.send(RoomCommand::Pause { client_id });
    }
}
