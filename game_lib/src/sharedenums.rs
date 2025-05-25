use crate::piece::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsAI,
    AIvsAI,
    Sandbox,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerRole {
    White,
    Black,
    Spectator,
    Solo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomStatus {
    WaitingPlayers,
    WaitingReady,
    Running,
    Finished,
    Paused,
}
