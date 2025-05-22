use crate::app::app::App;
use crate::app::pages::{
    create_game::CreateGame, game::Game, home::Home, info::Info, not_found::NotFound,
};
use crate::messages::ServerMessage;
use crate::routes::Route;
use crate::ws::ws::WsProvider;
use yew::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    // Create a callback for handling WebSocket me  ssages
    let on_message = Callback::from(|msg: ServerMessage| {
        log::debug!("Root received message: {:?}", msg);
    });

    html! {
        <WsProvider {on_message}>
            <App />
        </WsProvider>
    }
}
