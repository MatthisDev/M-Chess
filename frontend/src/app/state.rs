use crate::sharedenums::PlayerRole;
use game_lib::piece::Color;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ServerState {
    pub board: Vec<Vec<Option<String>>>,
    pub turn: Option<Color>,
    pub game_over: Option<String>,
    pub role: Option<PlayerRole>,
    pub room_id: Option<Uuid>,
    pub error: Option<String>,
    pub info: Option<String>,
    pub ready: bool,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            board: vec![vec![None; 8]; 8],
            turn: None,
            game_over: None,
            role: None,
            room_id: None,
            error: None,
            info: None,
            ready: false,
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
            }
            ServerAction::SetGameOver(result) => {
                new_state.game_over = Some(result);
            }
            ServerAction::SetInfo(msg) => {
                new_state.info = Some(msg);
            }
            ServerAction::SetRole(role, room_id) => {
                new_state.role = Some(role);
                new_state.room_id = Some(room_id);
            }
            ServerAction::SetRoomCreated(room_id) => {
                new_state.room_id = Some(room_id);
            }
            ServerAction::SetReady(ready) => {
                new_state.ready = ready;
            }
            ServerAction::Clear => {
                new_state = ServerState::default();
            }
        }

        Rc::new(new_state)
    }
}

pub enum ServerAction {
    SetBoard {
        board: Vec<Vec<Option<String>>>,
        turn: Color,
    },
    SetGameOver(String),
    SetInfo(String),
    SetRole(PlayerRole, uuid::Uuid),
    SetRoomCreated(uuid::Uuid),
    SetReady(bool),
    Clear,
}
