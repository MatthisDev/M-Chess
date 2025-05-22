use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use game_lib::{
    automation::ai::{Difficulty, AI},
    game::Game,
};
use tokio::{sync::mpsc::UnboundedSender, time::Instant};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::sharedenums::{GameMode, PlayerRole};

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
    pub last_active: Instant,
    pub hb: Instant,
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
