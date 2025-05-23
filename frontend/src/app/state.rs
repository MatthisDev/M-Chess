use game_lib::piece::Color;
use game_lib::sharedenums::{PlayerRole, RoomStatus};
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ServerState {
    pub host: bool,
    pub ping: bool,
    pub joined: bool,
    pub room_id: Option<Uuid>,
    pub room_status: Option<RoomStatus>,
    pub ready: bool,
    pub role: Option<PlayerRole>,
    //Game Run
    pub legals_moves: Vec<String>,
    pub board: Vec<Vec<Option<String>>>,
    pub turn: Option<Color>,
    pub game_over: Option<String>,
    //Other
    pub info: Option<String>,
    pub error: Option<String>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            host: false,
            ping: false,
            joined: false,
            room_id: None,
            room_status: None,
            ready: false,
            role: None,
            legals_moves: Vec::new(),
            board: vec![vec![None; 8]; 8],
            turn: None,
            game_over: None,
            info: None,
            error: None,
        }
    }
}

impl Reducible for ServerState {
    type Action = ServerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();

        match action {
            ServerAction::SetBoard { board, turn } => {
                new_state.board = board;
                new_state.turn = Some(turn);
                new_state.legals_moves = Vec::new()
            }
            ServerAction::SetGameOver(result, room_status) => {
                new_state.game_over = Some(result);
                new_state.room_status = Some(room_status);
            }
            ServerAction::SetInfo(msg) => {
                new_state.info = Some(msg);
            }
            ServerAction::SetRole(role, room_id, room_status) => {
                new_state.role = Some(role);
                new_state.room_id = Some(room_id);
                new_state.room_status = Some(room_status);
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
        }

        Rc::new(new_state)
    }
}

pub enum ServerAction {
    SetBoard {
        board: Vec<Vec<Option<String>>>,
        turn: Color,
    },
    SetLegalMoves(Vec<String>),
    SetGameOver(String, RoomStatus),
    SetInfo(String),
    SetRole(PlayerRole, uuid::Uuid, RoomStatus),
    SetReady(bool),
    SetRoomStatus(RoomStatus),
    SetError(String),
    SetJoined(bool, bool, RoomStatus),
    SetQuit,
    Ping,
    ResetPing,
}
