use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use game_lib::automation::ai::Difficulty;
use gloo_net::websocket::{futures::WebSocket, Message};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub mod messages;
pub mod sharedenums;
use frontend::messages::{ClientMessage, ServerMessage};
use frontend::sharedenums::GameMode;
#[derive(PartialEq, Clone, Debug)]
enum Page {
    Home,
    Game,
    Info,
    Install,
    CreateGame,
}

#[function_component(App)]
fn app() -> Html {
    let current_page = use_state(|| Page::Home);
    let game_started = use_state(|| false);
    let board_state = use_state(|| vec![vec![None::<String>; 8]; 8]);
    let game_over_message = use_state(|| None as Option<String>);
    let join_link = use_state(|| String::new());
    let selected_mode = use_state(|| None as Option<GameMode>);
    let ws_sender = use_mut_ref(|| None as Option<SplitSink<WebSocket, Message>>);

    let connect = {
        let ws_sender = ws_sender.clone();
        let current_page = current_page.clone();

        Callback::from(move |_: Option<SplitSink<WebSocket, Message>>| {
            log::info!("Connecting to WebSocket");
            let ws_sender = ws_sender.clone();
            spawn_local(async move {
                let ws = match WebSocket::open("ws://127.0.0.1:9001") {
                    Ok(ws) => ws,
                    Err(err) => {
                        log::error!("Failed to open WebSocket: {:?}", err);
                        return;
                    }
                };

                let (tx, mut rx): (SplitSink<WebSocket, Message>, SplitStream<WebSocket>) =
                    ws.split();

                *ws_sender.borrow_mut() = Some(tx);
                log::info!("WebSocket sender initialized");

                let msg = serde_json::to_string(&ClientMessage::Connect).unwrap();

                if let Some(sender) = ws_sender.borrow_mut().as_mut() {
                    log::debug!("Sending Connect message: {}", msg);
                    let _ = sender.send(Message::Text(msg)).await;
                }
            });
        })
    };

    use_effect_with_deps(
        move |_| {
            log::info!("Automatically calling connect");
            connect.emit(None);
            || ()
        },
        (), // Empty dependency array ensures this runs only once
    );

    log::info!("App component initialized");

    let switch_home = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            log::info!("Switching to Home page");
            current_page.set(Page::Home);
        })
    };

    let switch_info = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            log::info!("Switching to Info page");
            current_page.set(Page::Info);
        })
    };

    let switch_install = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            log::info!("Switching to Install page");
            current_page.set(Page::Install);
        })
    };

    let switch_create_game = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            log::info!("Switching to Create Game page");
            current_page.set(Page::CreateGame);
        })
    };

    let select_mode = {
        let selected_mode = selected_mode.clone();
        let ws_sender = ws_sender.clone();

        Callback::from(move |mode: GameMode| {
            log::info!("Game mode selected: {:?}", mode);
            selected_mode.set(Some(mode.clone()));

            let msg = serde_json::to_string(&ClientMessage::CreateRoom {
                mode,
                difficulty: None,
            })
            .unwrap();

            let ws_sender = ws_sender.clone();
            spawn_local(async move {
                if ws_sender.borrow().is_none() {
                    log::error!("WebSocket sender is not initialized");
                } else {
                    log::debug!("WebSocket sender is initialized");
                }

                if let Some(sender) = ws_sender.borrow_mut().as_mut() {
                    log::debug!("Sending CreateGame message: {}", msg);
                    let _ = sender.send(Message::Text(msg)).await;
                }
            });
        })
    };

    let quit_game = {
        let game_started = game_started.clone();
        let game_over_message = game_over_message.clone();
        let ws_sender = ws_sender.clone();

        Callback::from(move |_| {
            log::info!("Quitting game");
            let msg = serde_json::to_string(&ClientMessage::Quit).unwrap();

            let ws_sender = ws_sender.clone();
            spawn_local(async move {
                if let Some(sender) = ws_sender.borrow_mut().as_mut() {
                    log::debug!("Sending Quit message: {}", msg);
                    let _ = sender.send(Message::Text(msg)).await;
                }
            });

            game_started.set(false);
            game_over_message.set(Some("You quit the game. You lost.".to_string()));
        })
    };

    let set_join_link = {
        let join_link = join_link.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            log::debug!("Join link updated: {}", value);
            join_link.set(value);
        })
    };

    let game_board = html! {
        <div class="board">
            { for board_state.iter().enumerate().map(|(row_idx, row)| html! {
                <div class="row">
                    { for row.iter().enumerate().map(|(col_idx, cell)| {
                        let is_dark = (row_idx + col_idx) % 2 == 1;
                        let class = if is_dark { "cell dark" } else { "cell light" };
                        html! {
                            <div class={class}>
                                { cell.as_ref().map_or("", |p| get_piece_emoji(p)).to_string() }
                            </div>
                        }
                    }) }
                </div>
            }) }
        </div>
    };

    html! {
        <div class="app-container">
                <nav class="navbar">
                    <button onclick={switch_home} disabled={*game_started}>{ "Home" }</button>
                    <button onclick={switch_info} disabled={*game_started}>{ "Team Info" }</button>
                    <button onclick={switch_install} disabled={*game_started}>{ "Install"     }</button>
            </nav>

                <div class="content">
                      {
                      match *current_page {
                            Page::Home => html! {
                                <div class="    home-page">
                                <h1>{ "Welcome to M-Chess" }</h1>
                                    <div class="mode-buttons">
                                        <button onclick={switch_create_game}>{ "Create Game" }</button>
                                    </div>
                                    <div class="join-section">
                                        <h3>{ "Join a G    ame" }</h3>
                                    <input type="text" placeholder="Enter game link..." value={(*join_link).clone()} oninput={set_join_link} />
                                    </div>
                                </div>
                            },
                            Page::Info => html! { <div id="presentation" class="ta    b-content">
                        <h2>{ "Contexte du Pro    jet" }</h2>
                        <p>{ "Les échecs sont un jeu stratégique intemporel qui attire des joueurs de tous âges et niveaux. Avec l'évolution des technologies, développer une plateforme de jeu d'échecs en ligne, intégrant une intelligence artificielle compétitive et une interface intuitive, représente une opportunité d'offrir une expérience moderne et immersive aux utilisateurs." }</p>
                            <p>{ "Ce projet vise à concevoir un jeu d'échecs accessible aussi bien en mode hors ligne qu'en multijoueur via Internet ou un réseau local. Le développement reposera sur le langage Rust pour la robustesse du backend, tandis que l'interface utilisateur s'appuiera sur des technologies web modernes." }</p>

                            <h2>{ "Présentation de l'équipe de développement" }</h2>
                            <h3>{ "Matthis Guillet" }</h3>
                            <p>{ "Étudiant en deuxième année à l'EPITA, passionné d'informatique et de mathématiques depuis le collège. Il aime explorer de nouvelles disciplines et réaliser des projets pratiques pour appliquer ses connaissances. Il souhaite évoluer dans les domaines de l'IA et de la robotique en tant qu'ingénieur ou chercheur." }</p>

                            <h3>{ "Martin Mad    ier" }</h3>
                        <p>{ "Également étudiant en deuxième année à l'EPITA, il s'intéresse à l'informatique et aux nouvelles technologies. Ce projet est une opportunité pour lui d'acquérir des compétences en IA et en programmation en Rust, tout en découvrant le fonctionnement d'une application web basée sur des requêtes API." }</p>

                            <h3>{ "Martin Pasquier" }</h3>
                            <p>{ "Étudiant en deuxième année à l’EPITA, passionné par l'informatique et la technologie. Il consacre du temps à des projets annexes afin d'approfondir ses compétences techniques. Doté d’une solide expérience en développement web, il s'intéresse aussi à la gestion de projets avec Git. En tant que chef de groupe, il assurera la coordination des différentes étapes du projet." }</p>

                            <h3>{ "Matteo Wermert" }</h3>
                            <p>{ "Étudiant en deuxième année à l’EPITA, fasciné par les échecs, un jeu qu'il admire pour sa richesse stratégique et son élégance. Ce projet lui permet d'associer cette passion à l’apprentissage du langage Rust, lui offrant ainsi une opportunité d’acquérir des compétences précieuses pour son avenir." }</p>

                            <h2>{ "Objectifs du Pro    jet" }</h2>
                        <h3>{ "Objectifs Généraux" }</h3>
                            <p>{ "Notre objectif est de développer un jeu d'échecs en ligne complet et fonctionnel, doté d'une intelligence artificielle capable de calculer des coups optimaux selon différents niveaux de difficulté. Une interface web performante et intuitive garantira une expérience utilisateur fluide et agréable." }</p>

                            <h3>{ "Objectifs Spécifiques" }</h3>
                            <p>{ "Le projet permettra aux utilisateurs de jouer contre une intelligence artificielle ou contre un autre joueur, en local ou en ligne. L'interface web sera conçue pour offrir une navigation rapide et intuitive grâce à une architecture web moderne." }</p>
                            </div> },
                        Page::Install => html! {<div class="install-page">
                            <h2>{ "Installation" }</h2>
                            <a href="static/M-Chess_Project.zip" download="M-Chess_Project.zip">{ "Download the project" }</a><br />
                            <a href="static/M-Chess_Report.pdf" download="M-Chess_Report.pdf">{ "Download the report (    PDF)" }</a>
                    </    div> },
                            Page::Game =>     html! {
                            <div class="game-page">
                                    { game_board.clone() }
                                if let Some(msg) = game_over_message.as_ref() {
                                    <p class="game-over">{ msg }</p>
                                 }
                                    if *game_started {
                                        <button onclick={quit_game}    >{ "Qui    t Game" }</button>
                                    }
                                </div    >
                        },
                        Page::CreateGame => html! {
                                <div class="create-game-page">
                                    <h2>{ "Choose Game Mode" }</h2>
                                <div class="mode-buttons">
                                    <button onclick={select_mode.clone().reform(|_| GameMode::PlayerVsPlayer)}>{ "Player vs Player" }</button>
                                        <button onclick={select_mode.clone().reform(|_| GameMode::PlayerVsAI)}>{ "Player vs AI" }</button>
                                        <button onclick={select_mode.clone().reform(|_| GameMode::AIvsAI)}>{ "AI vs AI    " }</button>
                                    <button onclick={select_mode.clone().reform(|_| GameMode::Sandbox)  }>   { "Sandbox"  }</button>
                                    </div>
                            </div>
                        },
                    }
                }
            </div>
        </div>
    }
}

fn get_piece_emoji(piece: &str) -> &str {
    match piece {
        "wp" => "♙",
        "bp" => "♟",
        "wr" => "♖",
        "br" => "♜",
        "wn" => "♘",
        "bn" => "♞",
        "wb" => "♗",
        "bb" => "♝",
        "wq" => "♕",
        "bq" => "♛",
        "wk" => "♔",
        "bk" => "♚",
        _ => "",
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    log::info!("Starting M-Chess frontend");

    yew::Renderer::<App>::new().render();
}
