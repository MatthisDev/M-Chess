use game_lib::{automation::ai::Difficulty, piece::Color};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::sharedenums::GameMode;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    RoomCreated {
        room_id: Uuid,
    },
    Joined {
        color: Option<Color>,
        room_id: Uuid, // Some("White") / Some("Black") or None for spectator
    },
    GameStarted,
    State {
        board: Vec<Vec<Option<String>>>,
        turn: String,
    },
    GameOver {
        result: String,
    },
    Error {
        msg: String,
    },
    Status {
        ready: bool,
    },
    Info {
        msg: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    CreateRoom {
        mode: GameMode,
        difficulty: Option<Difficulty>, // for AI
    },
    JoinRoom {
        room_id: Uuid, // Uuid as string
    },
    Ready,
    StartGame,
    Move {
        mv: String,
    },
    Quit,
    Disconnect,
    Connect,
}
