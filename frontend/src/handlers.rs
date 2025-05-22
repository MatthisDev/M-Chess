use crate::state::Page;
use crate::sharedenums::GameMode;
use yew::prelude::*;

pub fn select_mode(mode: GameMode) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        log::info!("Selected mode: {:?}", mode);
    })
}

pub fn quit_game() -> Callback<MouseEvent> {
    Callback::from(move |_| {
        log::info!("Game quit");
    })
}

pub fn change_theme(theme: String) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        log::info!("Theme changed to: {}", theme);
    })
}