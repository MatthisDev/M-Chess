use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use game_lib::sharedenums::{GameMode, PlayerRole, RoomStatus};
use game_lib::{
    automation::ai::{Difficulty, AI},
    game::Game,
};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::{sync::mpsc::UnboundedSender, time::Instant};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    pub room_id: Option<Uuid>,
    pub sender: UnboundedSender<Message>,
    pub hb: Arc<AtomicU64>,
}

#[derive(Debug, Clone)]
pub enum PlayerType {
    Human,
    Ai { ai: AI }, // tu peux même ajouter un champ `name`, `strategy`, etc.
}

#[derive(Debug)]
pub struct Player {
    pub id: Uuid,
    pub role: PlayerRole,
    pub ready: bool,
    pub sender: Option<UnboundedSender<Message>>,
    pub kind: PlayerType,
}

#[derive(Debug)]
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
