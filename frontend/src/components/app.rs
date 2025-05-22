use crate::components::{board::render_board, palette::render_palette, navbar::render_navbar};
use crate::state::{use_game_state, Page};
use crate::handlers::{select_mode, quit_game};
use yew::prelude::*;
use crate::sharedenums::GameMode;
use std::rc::Rc;
#[function_component(App)]
pub fn app() -> Html {
    let current_page = use_game_state();
    let set_page = {
        let current_page = current_page.clone();
        Callback::from(move |page: Page| current_page.set(Rc::new(page)))
    };
    html! {
        <div class="app-container">
            { render_navbar(set_page.clone()) }
            <div class="content">
                {
                    match *current_page {
                        Page::Home => html! {
                            <div class="home-page">
                                <h1>{ "Welcome to M-Chess" }</h1>
                                <button onclick={set_page.reform(|_| Page::CreateGame)}>{ "Create Game" }</button>
                            </div>
                        },
                        Page::CreateGame => html! {
                            <div class="create-game-page">
                                <h2>{ "Choose Game Mode" }</h2>
                                <button onclick={select_mode(GameMode::PlayerVsPlayer)}>{ "Player vs Player" }</button>
                                <button onclick={select_mode(GameMode::PlayerVsAI)}>{ "Player vs AI" }</button>
                                <button onclick={select_mode(GameMode::AIvsAI)}>{ "AI vs AI" }</button>
                                <button onclick={select_mode(GameMode::Sandbox)}>{ "Sandbox" }</button>
                            </div>
                        },
                        Page::Game => html! {
                            <div class="game-page">
                                { render_board() }
                                <button onclick={quit_game}>{ "Quit Game" }</button>
                            </div>
                        },
                        _ => html! { <div>{ "Page not found" }</div> },
                    }
                }
            </div>
        </div>
    }
}