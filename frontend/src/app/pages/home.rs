use crate::routes::Route;
use crate::ws::WsContext;
use game_lib::messages::{ClientMessage, ServerMessage};
use game_lib::sharedenums::GameMode;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub on_navigate_create_game: Callback<()>,
    pub on_join_room: Callback<Uuid>,
    pub join_error: Option<String>,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let room_id = use_state(|| "".to_string());

    let room_id = use_state(|| "".to_string());

    let oninput = {
        let room_id = room_id.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            room_id.set(input.value());
        })
    };

    let on_click_create = {
        let cb = props.on_navigate_create_game.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let on_click_join = {
        let cb = props.on_join_room.clone();
        let room_id = (*room_id).clone();
        Callback::from(move |_| {
            if !room_id.is_empty() {
                if let Ok(id) = Uuid::parse_str(&room_id) {
                    cb.emit(id);
                } else {
                    log::error!("Invalid room ID: {}", room_id);
                }
            }
        })
    };

    html! {
        <div>
            <button onclick={on_click_create}>{ "Créer une room" }</button>
            <div>
                <input
                    type="text"
                    placeholder="Entrer l'ID de la room"
                    value={(*room_id).clone()}
                    oninput={oninput}
                />
                <button onclick={on_click_join}>{ "Rejoindre la partie" }</button>
            </div>
            {
                if let Some(err) = &props.join_error {
                    html! { <p style="color: red;">{ err }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
