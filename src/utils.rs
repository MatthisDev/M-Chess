use std::sync::{atomic::AtomicU64, Arc};

use engine::{automation::ai::AI, sharedenums::PlayerRole};
use tokio::sync::mpsc::UnboundedSender;
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
