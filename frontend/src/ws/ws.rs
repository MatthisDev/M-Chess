use std::rc::Rc;
use std::sync::{Arc, Mutex};

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::{platform::spawn_local, prelude::*};

use game_lib::messages::{ClientMessage, ServerMessage};

#[derive(Clone, PartialEq)]
pub struct WsContext {
    pub sender: Callback<ClientMessage>,
    pub connected: UseStateHandle<bool>,
    pub on_message: Callback<ServerMessage>,
}

#[derive(Default)]
struct InnerWsState {
    ws: Option<Arc<Mutex<SplitSink<WebSocket, Message>>>>,
}

#[derive(Properties, PartialEq)]
pub struct WsProviderProps {
    pub children: Children,
    pub on_message: Callback<ServerMessage>,
}

#[function_component(WsProvider)]
pub fn ws_provider(props: &WsProviderProps) -> Html {
    let inner_state = use_state(InnerWsState::default);
    let connected = use_state(|| false);

    let send_msg = {
        let inner_state = inner_state.clone();
        Callback::from(move |msg: ClientMessage| {
            if let Some(sender) = &inner_state.ws {
                let serialized = serde_json::to_string(&msg).unwrap();
                let sender = sender.clone();
                spawn_local(async move {
                    if let Ok(mut locked_sender) = sender.lock() {
                        let _ = locked_sender.send(Message::Text(serialized)).await;
                    }
                });
            }
        })
    };

    {
        let inner_state = inner_state.clone();
        let connected = connected.clone();
        let on_message = props.on_message.clone();

        use_effect_with_deps(
            move |_| {
                let cleanup_state = inner_state.clone();
                let cleanup_connected = connected.clone();

                spawn_local(async move {
                    let location = web_sys::window().unwrap().location();
                    let host = location.host().unwrap(); // ex: "mchess.fr" ou "mchess.fr:8080"
                    let ws_url = format!("ws://{}/ws", host);

                    match WebSocket::open(&ws_url) {
                        Ok(ws) => {
                            let (tx, mut rx) = ws.split();
                            let tx = Arc::new(Mutex::new(tx));

                            inner_state.set(InnerWsState {
                                ws: Some(tx.clone()),
                            });
                            connected.set(true);

                            while let Some(Ok(Message::Text(txt))) = rx.next().await {
                                log::info!("Received from server: {}", txt);
                                if let Ok(msg) = serde_json::from_str::<ServerMessage>(&txt) {
                                    log::info!("Parsed message: {:?}", msg);
                                    on_message.emit(msg);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("WebSocket connection failed: {:?}", e);
                        }
                    }
                });

                move || {
                    if let Some(ws) = cleanup_state.ws.clone() {
                        spawn_local(async move {
                            if let Ok(mut sender) = ws.lock() {
                                if let Err(e) = sender.close().await {
                                    log::error!("Failed to close WebSocket: {:?}", e);
                                }
                            }
                            cleanup_state.set(Default::default());
                            cleanup_connected.set(false);
                        });
                    }
                }
            },
            (),
        );
    }

    html! {
        <ContextProvider<WsContext> context={WsContext {
            sender: send_msg,
            connected,
            on_message: props.on_message.clone(),
        }}>
            { for props.children.iter() }
        </ContextProvider<WsContext>>
    }
}

impl WsContext {
    pub fn send(&self, msg: ClientMessage) {
        self.sender.emit(msg)
    }
}

fn get_or_create_client_id() -> String {
    if let Ok(client_id) = LocalStorage::get::<String>("client_id") {
        client_id
    } else {
        let new_id = Uuid::new_v4().to_string();
        LocalStorage::set("client_id", &new_id).expect("failed to set client_id");
        new_id
    }
}
