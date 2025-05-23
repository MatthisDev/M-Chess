use crate::sharedenums::GameMode;
use crate::sharedenums::PlayerRole;
use crate::sharedenums::RoomStatus;
use game_lib::{automation::ai::Difficulty, piece::Color};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Joined {
        role: PlayerRole,
        room_id: Uuid, // Some("White") / Some("Black") or None for spectator
        room_status: RoomStatus,
        host: bool,
    },
    GameStarted {
        room_status: RoomStatus,
        board: Vec<Vec<Option<String>>>,
        turn: Color,
    },
    State {
        board: Vec<Vec<Option<String>>>,
        turn: Color,
    },
    GameOver {
        room_status: RoomStatus,
        result: String,
    },
    Error {
        msg: String,
    },
    Status {
        ready: bool,
    },
    LegalMoves {
        moves: Vec<String>,
    },
    RoomStatus {
        status: RoomStatus,
    },
    PauseGame {
        room_status: RoomStatus,
    },
    Info {
        msg: String,
    },
    SandboxPieceAdded {
        piece: String,
        pos: String,
    },
    QuitGame,
    Ping,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    //Game
    CreateRoom {
        mode: GameMode,
        difficulty: Option<Difficulty>, // for AI
    },
    JoinRoom {
        room_id: Uuid, // Uuid as string
    },
    Ready {
        state: bool,
    },
    StartGame,
    Move {
        mv: String,
    },
    GetLegalMoves {
        mv: String,
    },
    Quit,
    // Server connection
    Connect,
    Disconnect,
    // Sandbox
    StartSandboxGame,
    AddPiece {
        piece: String,
        pos: String,
    },
    PauseRequest,
    Pong,
}
