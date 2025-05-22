use game_lib::piece::Color;
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
