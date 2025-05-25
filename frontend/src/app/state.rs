use game_lib::piece::Color;
use game_lib::sharedenums::{GameMode, PlayerRole, RoomStatus};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;

use crate::routes::Route;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerState {
    pub host: bool,
    pub ping: bool,
    pub joined: bool,
    pub room_id: Option<Uuid>,
    pub gamemod: Option<GameMode>,
    pub room_status: Option<RoomStatus>,
    pub ready: bool,
    pub role: Option<PlayerRole>,
    //Game Run
    pub legals_moves: Vec<String>,
    pub board: Vec<Vec<Option<String>>>,
    pub turn: Option<Color>,
    pub counter: usize,
    pub game_over: Option<String>,
    pub paused: bool,
    //Other
    pub info: Option<String>,
    pub error: Option<String>,
    pub last_page: Option<Route>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            host: false,
            ping: false,
            joined: false,
            room_id: None,
            gamemod: None,
            room_status: None,
            ready: false,
            role: None,
            legals_moves: Vec::new(),
            board: vec![vec![None; 8]; 8],
            turn: None,
            counter: 0,
            game_over: None,
            info: None,
            error: None,
            last_page: None,
            paused: false,
        }
    }
}

impl Reducible for ServerState {
    type Action = ServerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();

        match action {
            ServerAction::SetBoard {
                board,
                turn,
                counter,
            } => {
                new_state.board = board;
                new_state.turn = Some(turn);
                new_state.legals_moves = Vec::new();
                new_state.counter = counter;
            }
            ServerAction::SetGameOver(result, room_status) => {
                new_state.game_over = Some(result);
                new_state.room_status = Some(room_status);
            }
            ServerAction::SetInfo(msg) => {
                new_state.info = Some(msg);
            }
            ServerAction::SetRole(role, room_id, room_status, gamemod) => {
                new_state.role = Some(role);
                new_state.room_id = Some(room_id);
                new_state.room_status = Some(room_status);
                new_state.gamemod = Some(gamemod)
            }
            ServerAction::SetReady(ready) => {
                new_state.ready = ready;
            }
            ServerAction::SetRoomStatus(status) => {
                new_state.room_status = Some(status);
            }
            ServerAction::SetError(error) => {
                new_state.error = Some(error);
            }
            ServerAction::SetJoined(joined, host, room_status) => {
                new_state.joined = joined;
                new_state.room_status = Some(room_status);
                new_state.host = host;
                web_sys::console::log_1(
                    &format!(
                        "role: {:?}, gammemod: {:?}",
                        new_state.role, new_state.gamemod
                    )
                    .into(),
                );
            }
            ServerAction::SetLegalMoves(mv) => {
                new_state.legals_moves = mv;
            }
            ServerAction::SetQuit => {
                new_state = ServerState::default();
            }
            ServerAction::Ping => {
                new_state.ping = true;
            }
            ServerAction::ResetPing => new_state.ping = false,
            ServerAction::SetLastPage(route) => {
                new_state.last_page = Some(route);
            }
            ServerAction::Pausing => {
                new_state.paused = !new_state.paused;
            }
        }

        Rc::new(new_state)
    }
}

pub enum ServerAction {
    SetBoard {
        board: Vec<Vec<Option<String>>>,
        turn: Color,
        counter: usize,
    },
    SetLegalMoves(Vec<String>),
    SetGameOver(String, RoomStatus),
    SetInfo(String),
    SetRole(PlayerRole, uuid::Uuid, RoomStatus, GameMode),
    SetReady(bool),
    SetRoomStatus(RoomStatus),
    SetError(String),
    SetJoined(bool, bool, RoomStatus),
    SetQuit,
    Ping,
    ResetPing,
    SetLastPage(Route),
    Pausing,
}
