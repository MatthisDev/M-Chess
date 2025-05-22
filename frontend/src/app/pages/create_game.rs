use crate::{routes::Route, sharedenums::GameMode};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, PartialEq)]
pub struct CreateGameProps {
    pub on_create_game: Callback<GameMode>,
}

#[function_component(CreateGame)]
pub fn create_game(props: &CreateGameProps) -> Html {
    let navigator = use_navigator().unwrap();
    let on_click_pvp = {
        let on_create_game = props.on_create_game.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            on_create_game.emit(GameMode::PlayerVsPlayer);
            navigator.push(&Route::Game);
        })
    };
    let on_click_p_ai = {
        let on_create_game = props.on_create_game.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            on_create_game.emit(GameMode::PlayerVsAI);
            navigator.push(&Route::Game);
        })
    };
    let on_click_ai_ai = {
        let on_create_game = props.on_create_game.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            on_create_game.emit(GameMode::AIvsAI);
            navigator.push(&Route::Game);
        })
    };
    let on_click_sandbox = {
        let on_create_game = props.on_create_game.clone();
        Callback::from(move |_| {
            on_create_game.emit(GameMode::Sandbox);
            navigator.push(&Route::Game);
        })
    };

    html! {
        <div>
            <h2>{ "Create a new game" }</h2>
            <button onclick={on_click_pvp}>{ "Player vs Player" }</button>
            <button onclick={on_click_p_ai}>{ "Play vs AI" }</button>
            <button onclick={on_click_ai_ai}>{ "AI vs AI" }</button>
            <button onclick={on_click_sandbox}>{ "Sandbox mode" }</button>
        </div>
    }
}
