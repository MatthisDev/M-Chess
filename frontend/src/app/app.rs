use crate::app::pages::navbar::Navbar;
use crate::app::pages::not_found::NotFound;
use crate::messages::{ClientMessage, ServerMessage};
use crate::routes::Route;
use crate::sharedenums::GameMode;
use crate::sharedenums::PlayerRole;
use crate::ws::WsContext;
use crate::ws_context;
use game_lib::piece::Color;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::console;
use yew::prelude::*;
use yew_router::hooks::{use_navigator, use_route};
use yew_router::prelude::{use_location, NavigationError, Navigator};
use yew_router::{BrowserRouter, Switch};

use crate::app::pages::{create_game::CreateGame, game::Game, home::Home, info::Info};
use crate::app::state::{ServerAction, ServerState};

use super::pages::download::Download;

#[derive(Clone, PartialEq)]
enum Page {
    Home,
    CreateGame,
    Game,
    Info,
}

#[function_component(App)]
pub fn app() -> Html {
    let page = use_state(|| Page::Home);
    let server_state = use_reducer_eq(ServerState::default);

    let on_server_message = {
        let dispatch = server_state.clone();
        Callback::from(move |msg: ServerMessage| {
            dispatch_server_message(msg, dispatch.clone());
        })
    };
    let ctx_opt = use_context::<WsContext>();
    let Some(ctx) = ctx_opt else {
        return html! {
            <div class="error">
                <h2>{ "WebSocket not connected" }</h2>
            </div>
        };
    };

    let on_create_game = {
        let ctx = ctx.clone();
        let page = page.clone();
        Callback::from(move |mode: GameMode| {
            ctx.send(ClientMessage::CreateRoom {
                mode,
                difficulty: None,
            });
            page.set(Page::Game);
        })
    };

    let on_quit = {
        let ctx = ctx.clone();
        Callback::from(move |_: ()| {
            ctx.send(ClientMessage::Quit);
        })
    };

    html! {
        <BrowserRouter>
            <AppInner/>
        </BrowserRouter>
    }
}

fn dispatch_server_message(msg: ServerMessage, dispatch: UseReducerHandle<ServerState>) {
    match msg {
        ServerMessage::State { board, turn } => {
            dispatch.dispatch(ServerAction::SetBoard { board, turn });
        }
        ServerMessage::GameOver { result } => {
            dispatch.dispatch(ServerAction::SetGameOver(result));
        }
        ServerMessage::Info { msg } => {
            dispatch.dispatch(ServerAction::SetInfo(msg));
        }
        ServerMessage::Joined { role, room_id } => {
            dispatch.dispatch(ServerAction::SetRole(role, room_id));
        }
        ServerMessage::RoomCreated { room_id } => {
            dispatch.dispatch(ServerAction::SetRoomCreated(room_id));
        }
        ServerMessage::QuitGame => {
            dispatch.dispatch(ServerAction::Clear);
        }
        ServerMessage::Status { ready } => {
            dispatch.dispatch(ServerAction::SetReady(ready));
        }
        ServerMessage::Error { msg } => {
            web_sys::window()
                .unwrap()
                .alert_with_message(&format!("Server Error: {}", msg))
                .ok();
        }
        _ => {
            // Ignore Ping, SandboxPieceAdded for now
        }
    }
}

#[function_component(AppInner)]
fn app_inner() -> Html {
    let current_route = use_route::<Route>().unwrap_or(Route::NotFound);
    console::log_1(&format!("Current route: {:?}", current_route).into());

    let navigator = use_navigator().expect("navigator not available");
    let join_error = use_state(|| None as Option<String>);
    let ctx = use_context::<WsContext>().expect("WsContext missing");

    // Callback pour créer une room : navigue vers CreateGame
    let on_navigate_create_game = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::CreateGame);
        })
    };

    // Callback pour rejoindre une room
    let on_join_room = {
        let navigator = navigator.clone();
        let join_error = join_error.clone();
        let ctx = ctx.clone();

        Callback::from(move |room_id: Uuid| {
            join_error.set(None); // reset error

            // Exemple: envoi message JoinRoom au serveur
            ctx.send(ClientMessage::JoinRoom {
                room_id: room_id.clone(),
            });

            // TODO: ici, on attend la réponse serveur pour savoir si join ok
            // Dans l’intervalle, on peut garder un état "en attente" (non montré ici)

            // Pour l’exemple, on imagine que la réponse est asynchrone,
            // il faut gérer cela dans ton WebSocket handler (en dessous).
        })
    };
    {
        let join_error = join_error.clone();
        let navigator = navigator.clone();
        use_effect_with_deps(
            move |msg| {
                // ici, tu devrais écouter les messages serveur en global ou dans un contexte
                // Exemple fictif de gestion de message serveur (tu dois adapter)
                // Si le serveur a envoyé un ServerMessage::Joined, on change de page
                // Si erreur, on set join_error à Some("Erreur ...")

                // NOTE : tu dois implémenter cette logique dans ton handler WebSocket central,
                // par exemple en mettant le résultat dans un state global ou en envoyant des callbacks.

                || ()
            },
            (),
        );
    }

    html! {
        <main>
            {
                if matches!(current_route, Route::Home | Route::Info | Route::Download) {
                    html! { <Navbar /> }
                } else {
                    html! {}
                }
            }
            <Switch<Route> render={switch_with_props(
                on_navigate_create_game.clone(),
                on_join_room.clone(),
                (*join_error).clone(),
            )} />
        </main>
    }
}

fn switch_with_props(
    on_navigate_create_game: Callback<()>,
    on_join_room: Callback<Uuid>,
    join_error: Option<String>,
) -> impl Fn(Route) -> Html {
    move |route| match route {
        Route::Home => html! {
            <Home
                on_navigate_create_game={on_navigate_create_game.clone()}
                on_join_room={on_join_room.clone()}
                join_error={join_error.clone()}
            />
        },
        Route::Info => html! { <Info /> },
        Route::Download => html! { <Download /> },
        Route::CreateGame => html! {
            <CreateGame on_create_game={Callback::from(|_| {})} />
        },
        Route::Game => {
            let empty_board: Vec<Vec<Option<String>>> = vec![];
            let game_over: Option<String> = None;

            html! {
                <Game
                    board={empty_board}
                    game_over={game_over}
                    on_quit={Callback::from(|_| {})}
                />
            }
        }
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
    }
}
