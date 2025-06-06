use crate::{routes::Route, ws::WsContext};
use game_lib::automation::ai::Difficulty;
use game_lib::{messages::ClientMessage, sharedenums::GameMode};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, PartialEq)]
pub struct CreateGameProps {
    pub on_create_game: Callback<(GameMode, Option<Difficulty>)>, // ajout difficulté
}

#[function_component(CreateGame)]
pub fn create_game(props: &CreateGameProps) -> Html {
    let selected_mode = use_state(|| None as Option<GameMode>);
    let ctx = use_context::<WsContext>().expect("WsContext missing");
    let navigator = use_navigator().unwrap();

    let on_mode_click = {
        let selected_mode = selected_mode.clone();
        let ctx = ctx.clone();
        Callback::from(move |mode: GameMode| {
            selected_mode.set(Some(mode.clone()));
            if matches!(mode, GameMode::PlayerVsPlayer | GameMode::Sandbox) {
                ctx.send(ClientMessage::CreateRoom {
                    mode,
                    difficulty: None,
                });
            }
        })
    };

    let on_difficulty_click = {
        let selected_mode = selected_mode.clone();
        let ctx = ctx.clone();
        Callback::from(move |difficulty: Difficulty| {
            if let Some(mode) = (*selected_mode).clone() {
                ctx.send(ClientMessage::CreateRoom {
                    mode,
                    difficulty: Some(difficulty),
                });
            }
        })
    };

    html! {
        <div class="create-game-container">
            <button class="create-game-button" onclick={Callback::from(move |_| navigator.push(&Route::Home))}>{ "Retour" }</button>
            <h2 class="create-game-title">{ "Create a new game" }</h2>
            <div class="create-game-buttons">
                <button class="create-game-button" onclick={on_mode_click.reform(|_| GameMode::PlayerVsPlayer)}>{ "Player vs Player" }</button>
                <button class="create-game-button" onclick={on_mode_click.reform(|_| GameMode::PlayerVsAI)}>{ "Player vs AI" }</button>
                <button class="create-game-button" onclick={on_mode_click.reform(|_| GameMode::AIvsAI)}>{ "AI vs AI" }</button>
                <button class="create-game-button" onclick={on_mode_click.reform(|_| GameMode::Sandbox)}>{ "Sandbox mode" }</button>
            </div>
            {
                if let Some(mode) = (*selected_mode).clone() {
                    if matches!(mode, GameMode::PlayerVsAI | GameMode::AIvsAI) {
                        html! {
                            <div class="create-game-difficulty">
                                <p>{ "Select difficulty:" }</p>
                                <button class="create-game-button" onclick={on_difficulty_click.reform(|_| Difficulty::Easy)}>{ "Easy" }</button>
                                <button class="create-game-button" onclick={on_difficulty_click.reform(|_| Difficulty::Medium)}>{ "Medium" }</button>
                                <button class="create-game-button" onclick={on_difficulty_click.reform(|_| Difficulty::Hard)}>{ "Hard" }</button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
