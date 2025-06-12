use engine::automation::ai::{Difficulty, AI};
use engine::game::Game;
use engine::piece::Color;
use futures::{SinkExt, StreamExt};
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

use crate::room::Room;
use crate::serverstate::{ServerState, SharedServerState};
use crate::{send_to_client, send_to_player};
use engine::messages::{ClientMessage, ServerMessage};
use engine::sharedenums::{GameMode, PlayerRole, RoomStatus};

pub fn to_player_role(color: Color) -> PlayerRole {
    match color {
        Color::White => PlayerRole::White,
        Color::Black => PlayerRole::Black,
    }
}
pub fn send_game_state_to_clients(room: &Room) {
    /*
    let board = room.game.board.export_display_board();
    let turn = room.game.board.turn;

    for player in room.players.values() {
        if let Some(sender) = &player.sender {
            let _ = sender.send(Message::Text(
                serde_json::to_string(&ServerMessage::State {
                    board: board.clone(),
                    turn,
                    counter: room.game.board.counter,
                    incheck: None,
                })
                .unwrap()
                .into(),
            ));
        }
    }
    */
}
