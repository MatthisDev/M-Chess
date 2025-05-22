use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum Page {
    Home,
    CreateGame,
    Game,
    Info,
    Install,
}

pub fn use_game_state() -> UseStateHandle<Rc<Page>> {
    use_state(|| Rc::new(Page::Home))
}


pub fn use_board_state() -> UseStateHandle<Vec<Vec<Option<String>>>> {
    use_state(|| vec![vec![None; 8]; 8])
}

pub fn use_possible_moves() -> UseStateHandle<Vec<String>> {
    use_state(|| Vec::new())
}

pub fn use_selected_piece() -> UseStateHandle<Option<String>> {
    use_state(|| None)
}
