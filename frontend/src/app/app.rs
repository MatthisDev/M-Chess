use crate::app::pages::navbar::Navbar;
use crate::app::pages::not_found::NotFound;
use crate::routes::Route;
use crate::ws::{WsContext, WsProvider};
use crate::ws_context;
use game_lib::messages::{ClientMessage, ServerMessage};
use game_lib::piece::Color;
use game_lib::sharedenums::GameMode;
use game_lib::sharedenums::PlayerRole;
use gloo_storage::{LocalStorage, Storage};
use std::rc::Rc;
use std::str::FromStr;
use uuid::Uuid;
use web_sys::console;
use yew::prelude::*;
use yew_router::hooks::{use_navigator, use_route};
use yew_router::prelude::{use_location, NavigationError, Navigator};
use yew_router::{BrowserRouter, Switch};

use crate::app::pages::{create_game::CreateGame, game::Game, home::Home, info::Info};
use crate::app::state::{ServerAction, ServerState};

use super::pages::download::Download;
use super::pages::game::_GameProps::on_quit;

#[derive(Clone, PartialEq)]
enum Page {
    Home,
    CreateGame,
    Game,
    Info,
}

#[function_component(App)]
pub fn app() -> Html {
    let server_state = use_reducer_eq(ServerState::default);
    let on_message_state = use_state::<Option<Callback<ServerMessage>>, _>(|| None);

    let set_on_message = on_message_state.setter().clone();
    let on_message = (*on_message_state).clone();

    html! {
        <ContextProvider<UseReducerHandle<ServerState>> context={server_state.clone()}>
            <BrowserRouter>
                {
                    if let Some(on_message) = on_message {
                        html! {
                            <WsProvider {on_message}>
                                <AppInner />
                            </WsProvider>
                        }
                    } else {
                        html! {
                            <PrepareCallback set_on_message={set_on_message} server_state={server_state.clone()} />
                        }
                    }
                }
            </BrowserRouter>
        </ContextProvider<UseReducerHandle<ServerState>>>
    }
}

fn dispatch_server_message(
    msg: ServerMessage,
    dispatch: UseReducerHandle<ServerState>,
    navigator: Navigator,
) {
    match msg {
        ServerMessage::State {
            board,
            turn,
            counter,
            incheck,
        } => {
            dispatch.dispatch(ServerAction::SetBoard {
                board,
                turn,
                counter,
                incheck,
            });
        }
        ServerMessage::LegalMoves { moves } => {
            dispatch.dispatch(ServerAction::SetLegalMoves(moves));
        }
        ServerMessage::GameOver {
            room_status,
            result,
        } => {
            dispatch.dispatch(ServerAction::SetGameOver(result, room_status));
        }
        ServerMessage::Info { msg } => {
            dispatch.dispatch(ServerAction::SetInfo(msg));
        }
        ServerMessage::Joined {
            role,
            room_id,
            room_status,
            host,
            gamemod,
        } => {
            dispatch.dispatch(ServerAction::SetRole(role, room_id, room_status, gamemod));
            dispatch.dispatch(ServerAction::SetJoined(true, host, room_status));
            navigator.push(&Route::Game);
        }
        ServerMessage::QuitGame => {
            dispatch.dispatch(ServerAction::SetQuit);
            navigator.push(&Route::Home);
        }
        ServerMessage::Status { ready } => {
            web_sys::console::log_1(&format!("🟢 Ready status received: {}", ready).into());
            dispatch.dispatch(ServerAction::SetReady(ready));
        }
        ServerMessage::RoomStatus { status } => {
            web_sys::console::log_1(&format!("Room Status updated to {:?}", status).into());
            dispatch.dispatch(ServerAction::SetRoomStatus(status));
        }
        ServerMessage::Error { msg } => {
            web_sys::window()
                .unwrap()
                .alert_with_message(&format!("Server Error: {}", msg))
                .ok();
        }
        ServerMessage::GameStarted {
            room_status,
            board,
            turn,
        } => {
            dispatch.dispatch(ServerAction::SetBoard {
                board,
                turn,
                counter: 0,
                incheck: None,
            });
            dispatch.dispatch(ServerAction::SetRoomStatus(room_status));
        }
        ServerMessage::Ping => {
            web_sys::console::log_1(&"🏓 Ping received, sending pong...".into());
            dispatch.dispatch(ServerAction::Ping);
        }
        ServerMessage::PauseGame { room_status } => {
            dispatch.dispatch(ServerAction::Pausing);
            dispatch.dispatch(ServerAction::SetRoomStatus(room_status));
        }
        ServerMessage::CloseRoom { id } => {
            dispatch.dispatch(ServerAction::SetQuit);
        }
        _ => {
            web_sys::console::log_1(&format!("❓ Message inattendu: {:?}", msg).into());
        }
    }
}

#[function_component(AppInner)]
fn app_inner() -> Html {
    let current_route = use_route::<Route>().unwrap_or(Route::NotFound);
    let navigator = use_navigator().expect("navigator not available");
    let ctx = use_context::<WsContext>().expect("WsContext missing");
    let server_state = use_context::<UseReducerHandle<ServerState>>().expect("ServerState missing");

    {
        let server_state = server_state.clone();
        use_effect_with_deps(
            move |route| {
                // Sauvegarde le last_page dans le ServerState ET dans localStorage
                LocalStorage::set("last_page", route.to_string())
                    .expect("failed to store last_page");
                || ()
            },
            current_route.clone(),
        );
    }

    {
        let navigator = navigator.clone();
        let ctx = ctx.clone();
        use_effect_with_deps(
            move |&joined| {
                // Redirect to game if joined
                if joined {
                    navigator.push(&Route::Game);
                }

                || ()
            },
            server_state.joined,
        );
    }
    {
        let ctx = ctx.clone();
        let server_guard = server_state.clone();
        use_effect_with_deps(
            move |&ping| {
                if ping {
                    ctx.send(ClientMessage::Pong);
                    server_guard.dispatch(ServerAction::ResetPing);
                }
                || ()
            },
            server_state.ping,
        );
    }
    {
        let ctx = ctx.clone();
        use_effect_with_deps(
            move |&(room_id, ingame)| {
                web_sys::console::log_1(&"Ingame mais pas de Room?".to_string().into());
                if room_id.is_none() && ingame {
                    ctx.send(ClientMessage::Quit);
                }
                || ()
            },
            (server_state.room_id, server_state.ingame),
        );
    }

    let on_navigate_create_game = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::CreateGame);
        })
    };

    let on_join_room = {
        let ctx = ctx.clone();
        Callback::from(move |room_id: Uuid| {
            ctx.send(ClientMessage::JoinRoom { room_id });
        })
    };

    let on_quit_game = {
        let ctx = ctx.clone();
        Callback::from(move |_: ()| {
            ctx.send(ClientMessage::Quit);
        })
    };

    html! {
        <main>
            {
                if matches!(current_route, Route::Home | Route::Info | Route::Download|Route::NotFound) {
                    html! { <Navbar /> }
                } else {
                    html! {}
                }
            }
            <Switch<Route> render={switch_with_props(
                on_navigate_create_game.clone(),
                on_join_room.clone(),
                None,
                on_quit_game.clone()
            )} />
        </main>
    }
}

fn switch_with_props(
    on_navigate_create_game: Callback<()>,
    on_join_room: Callback<Uuid>,
    join_error: Option<String>,
    on_quit_game: Callback<()>,
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
                <Game on_quit={on_quit_game.clone()} />
            }
        }
        Route::NotFound => html! { <NotFound /> },
    }
}

#[derive(Properties, PartialEq)]
struct PrepareCallbackProps {
    set_on_message: UseStateSetter<Option<Callback<ServerMessage>>>,
    server_state: UseReducerHandle<ServerState>,
}

#[function_component(PrepareCallback)]
fn prepare_callback(props: &PrepareCallbackProps) -> Html {
    let navigator = use_navigator().expect("navigator not available");

    {
        let dispatch = props.server_state.clone();
        let navigator = navigator.clone();
        let set_on_message = props.set_on_message.clone();

        use_effect_with_deps(
            move |_| {
                let dispatch_clone = dispatch.clone();
                let navigator_clone = navigator.clone();
                let set_on_message_clone = set_on_message.clone();
                let navigator_for_redirect = navigator.clone();
                // Prépare le callback WebSocket
                let callback = Callback::from(move |msg: ServerMessage| {
                    web_sys::console::log_1(&format!("📩 Message reçu: {:?}", msg).into());
                    dispatch_server_message(
                        msg.clone(),
                        dispatch_clone.clone(),
                        navigator_clone.clone(),
                    );
                });
                set_on_message_clone.set(Some(callback));

                // Redirection automatique dans le prochain "tick"

                let stored_page = LocalStorage::get::<String>("last_page").ok();
                if let Some(page_str) = stored_page {
                    if let Ok(route) = Route::from_str(&page_str) {
                        navigator_for_redirect.push(&route);
                    }
                }

                || ()
            },
            (),
        );
    }

    html! {
        // Affiche un écran de chargement simple le temps que le callback soit prêt
        <div>{ "Initialisation..." }</div>
    }
}
