use game_lib::board::Board;
use game_lib::game::{self, Game};
use yew::prelude::*;

#[derive(PartialEq, Clone, Debug)]
enum Page {
    Home,
    Game,
    Info,
    Install,
}

#[derive(PartialEq, Clone, Debug)]
enum GameMode {
    PlayerVsPlayer,
    PlayerVsAI,
    AIvsAI,
    Sandbox,
}

#[function_component(App)]
fn app() -> Html {
    let current_page = use_state(|| Page::Home);
    let game_mode = use_state(|| GameMode::Sandbox);
    let used_game = use_state(|| Game::init(game::GameMode::Sandbox));
    let selected_piece = use_state(|| None as Option<String>);
    let game_started = use_state(|| false);
    let game_ready = use_state(|| false);
    let game_over_message = use_state(|| None as Option<String>);
    let join_link = use_state(|| String::new());

    let switch_page = |page: Page| {
        let current_page = current_page.clone();
        Callback::from(move |_| current_page.set(page.clone()))
    };

    let start_game = {
        let game_mode = game_mode.clone();
        let used_game = used_game.clone();
        let current_page = current_page.clone();
        let game_ready = game_ready.clone();
        let game_started = game_started.clone();
        Callback::from(move |mode: GameMode| {
            game_mode.set(mode.clone());
            let gm = match mode {
                GameMode::Sandbox => game::GameMode::Sandbox,
                GameMode::PlayerVsPlayer => game::GameMode::PlayerVsPlayer,
                GameMode::PlayerVsAI => game::GameMode::PlayerVsAI,
                GameMode::AIvsAI => game::GameMode::AIvsAI,
            };
            used_game.set(Game::init(gm));
            game_ready.set(false);
            game_started.set(false);
            current_page.set(Page::Game);
        })
    };

    let trigger_game_start = {
        let game_started = game_started.clone();
        let game_over_message = game_over_message.clone();
        Callback::from(move |_| {
            game_started.set(true);
            game_over_message.set(None);
            // TODO: Connect to backend socket or trigger server sync here
        })
    };

    let quit_game = {
        let game_started = game_started.clone();
        let game_over_message = game_over_message.clone();
        Callback::from(move |_| {
            game_started.set(false);
            game_over_message.set(Some("You quit the game. You lost.".to_string()));
        })
    };

    let set_join_link = {
        let join_link = join_link.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            join_link.set(value);
        })
    };

    html! {
        <div class="app-container">
            <nav class="navbar">
                <button onclick={switch_page(Page::Home)} disabled={*game_started}>{ "Home" }</button>
                <button onclick={switch_page(Page::Info)} disabled={*game_started}>{ "Team Info" }</button>
                <button onclick={switch_page(Page::Install)} disabled={*game_started}>{ "Install" }</button>
            </nav>

            <div class="content">
                {
                    match *current_page {
                        Page::Home => html! {
                            <div class="home-page">
                                <h1>{ "Welcome to M-Chess" }</h1>
                                <div class="mode-buttons">
                                    <button onclick={start_game.reform(|_| GameMode::PlayerVsPlayer)}>{ "Player vs Player" }</button>
                                    <button onclick={start_game.reform(|_| GameMode::PlayerVsAI)}>{ "Player vs AI" }</button>
                                    <button onclick={start_game.reform(|_| GameMode::AIvsAI)}>{ "AI vs AI" }</button>
                                    <button onclick={start_game.reform(|_| GameMode::Sandbox)}>{ "Sandbox Mode" }</button>
                                </div>
                                <div class="join-section">
                                    <h3>{ "Join a Game" }</h3>
                                    <input type="text" placeholder="Enter game link..." value={(*join_link).clone()} oninput={set_join_link} />
                                    <button>{ "Join Game" }</button>
                                </div>
                            </div>
                        },
                        Page::Info => html! {
                            <div id="presentation" class="tab-content">
                                <h2>{ "Contexte du Projet" }</h2>
                                <p>{ "Les échecs sont un jeu stratégique intemporel qui attire des joueurs de tous âges et niveaux. Avec l'évolution des technologies, développer une plateforme de jeu d'échecs en ligne, intégrant une intelligence artificielle compétitive et une interface intuitive, représente une opportunité d'offrir une expérience moderne et immersive aux utilisateurs." }</p>
                                <p>{ "Ce projet vise à concevoir un jeu d'échecs accessible aussi bien en mode hors ligne qu'en multijoueur via Internet ou un réseau local. Le développement reposera sur le langage Rust pour la robustesse du backend, tandis que l'interface utilisateur s'appuiera sur des technologies web modernes." }</p>

                                <h2>{ "Présentation de l'équipe de développement" }</h2>
                                <h3>{ "Matthis Guillet" }</h3>
                                <p>{ "Étudiant en deuxième année à l'EPITA, passionné d'informatique et de mathématiques depuis le collège. Il aime explorer de nouvelles disciplines et réaliser des projets pratiques pour appliquer ses connaissances. Il souhaite évoluer dans les domaines de l'IA et de la robotique en tant qu'ingénieur ou chercheur." }</p>

                                <h3>{ "Martin Madier" }</h3>
                                <p>{ "Également étudiant en deuxième année à l'EPITA, il s'intéresse à l'informatique et aux nouvelles technologies. Ce projet est une opportunité pour lui d'acquérir des compétences en IA et en programmation en Rust, tout en découvrant le fonctionnement d'une application web basée sur des requêtes API." }</p>

                                <h3>{ "Martin Pasquier" }</h3>
                                <p>{ "Étudiant en deuxième année à l’EPITA, passionné par l'informatique et la technologie. Il consacre du temps à des projets annexes afin d'approfondir ses compétences techniques. Doté d’une solide expérience en développement web, il s'intéresse aussi à la gestion de projets avec Git. En tant que chef de groupe, il assurera la coordination des différentes étapes du projet." }</p>

                                <h3>{ "Matteo Wermert" }</h3>
                                <p>{ "Étudiant en deuxième année à l’EPITA, fasciné par les échecs, un jeu qu'il admire pour sa richesse stratégique et son élégance. Ce projet lui permet d'associer cette passion à l’apprentissage du langage Rust, lui offrant ainsi une opportunité d’acquérir des compétences précieuses pour son avenir." }</p>

                                <h2>{ "Objectifs du Projet" }</h2>
                                <h3>{ "Objectifs Généraux" }</h3>
                                <p>{ "Notre objectif est de développer un jeu d'échecs en ligne complet et fonctionnel, doté d'une intelligence artificielle capable de calculer des coups optimaux selon différents niveaux de difficulté. Une interface web performante et intuitive garantira une expérience utilisateur fluide et agréable." }</p>

                                <h3>{ "Objectifs Spécifiques" }</h3>
                                <p>{ "Le projet permettra aux utilisateurs de jouer contre une intelligence artificielle ou contre un autre joueur, en local ou en ligne. L'interface web sera conçue pour offrir une navigation rapide et intuitive grâce à une architecture web moderne." }</p>
                            </div>
                        },
                        Page::Install => html! {
                            <div class="install-page">
                                <h2>{ "Installation" }</h2>
                                <a href="static/M-Chess_Project.zip" download="M-Chess_Project.zip">{ "Download the project" }</a><br />
                                <a href="static/M-Chess_Report.pdf" download="M-Chess_Report.pdf">{ "Download the report (PDF)" }</a>
                            </div>
                        },
                        Page::Game => html! {
                            <div class="game-page">
                                <GameBoard
                                    game_mode={(*game_mode).clone()}
                                    used_game={used_game.clone()}
                                    selected_piece={selected_piece.clone()}
                                    game_started={game_started.clone()}
                                    game_ready={game_ready.clone()}
                                    game_over_message={game_over_message.clone()}
                                    on_start={trigger_game_start.clone()}
                                    on_quit={quit_game.clone()}
                                />
                            </div>
                        },
                    }
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct GameBoardProps {
    game_mode: GameMode,
    used_game: UseStateHandle<Game>,
    selected_piece: UseStateHandle<Option<String>>,
    game_started: UseStateHandle<bool>,
    game_ready: UseStateHandle<bool>,
    game_over_message: UseStateHandle<Option<String>>,
    on_start: Callback<MouseEvent>,
    on_quit: Callback<MouseEvent>,
}

#[function_component(GameBoard)]
fn game_board(props: &GameBoardProps) -> Html {
    let GameBoardProps {
        game_mode,
        used_game,
        selected_piece,
        game_started,
        game_ready,
        game_over_message,
        on_start,
        on_quit,
    } = props;
    let board_state = used_game.board.get();
    let selected_cell = use_state(|| None as Option<String>);

    let move_piece = Callback::from({
        let used_game = used_game.clone();
        let game_mode = game_mode.clone();
        let game_started = game_started.clone();
        let game_over_message = game_over_message.clone();
        move |move_str: String| {
            used_game.set({
                let mut game = (*used_game).clone();
                if let Err(err) = game.make_move_algebraic(&move_str) {
                    web_sys::console::log_1(&format!("Invalid move: {}", err).into());
                } else if game.board.is_game_over() {
                    game_started.set(false);
                    let message = if game.board.is_checkmate(game.board.turn) {
                        format!(
                            "Checkmate! {} won.",
                            if game.board.turn == game_lib::piece::Color::White {
                                "Black"
                            } else {
                                "White"
                            }
                        )
                    } else {
                        "Pat! No one won".to_string()
                    };
                    game_over_message.set(Some(message));
                }
                game
            });
        }
    });

    let render_board = html! {
        <div class="board">
            { for board_state.iter().enumerate().map(|(row_idx, row)| {
                html! {
                    { for row.iter().enumerate().map(|(col_idx, cell)| {
                        let is_dark = (row_idx + col_idx) % 2 == 1;
                        let mut cell_class = if is_dark { "cell dark" } else { "cell light" }.to_string();
                        let position = format!("{}{}", (b'a' + col_idx as u8) as char, row_idx);

                        if selected_cell.as_deref() == Some(&position) {
                            cell_class.push_str(" selected");
                        }

                        let onclick = {
                            let selected_cell = selected_cell.clone();
                            let selected_piece = selected_piece.clone();
                            let used_game = used_game.clone();
                            let move_piece = move_piece.clone();
                            let game_mode = game_mode.clone();
                            let game_started = game_started.clone();
                            Callback::from(move |_| {
                                if *game_started {
                                    if game_mode == GameMode::Sandbox || game_mode == GameMode::PlayerVsPlayer || game_mode == GameMode::PlayerVsAI {
                                        if let Some(from) = &*selected_cell {
                                            move_piece.emit(format!("{}->{}", from, position));
                                            selected_cell.set(None);
                                        } else {
                                            selected_cell.set(Some(position.clone()));
                                        }
                                    }
                                } else if game_mode == GameMode::Sandbox {
                                    if let Some(piece) = &*selected_piece {
                                        let mut game = (*used_game).clone();
                                        game.board.add_piece(&format!("{}{}", piece, position));
                                        used_game.set(game);
                                    } else {
                                        web_sys::console::log_1(&"No piece selected!".into());
                                    }
                                }
                            })
                        };

                        html! {
                            <div class={cell_class} {onclick}>
                                {
                                    if cell != ".." {
                                        html! { <div class="piece">{ get_piece_emoji(cell) }</div> }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        }
                    }) }
                }
            }) }
        </div>
    };

    let render_palette = || {
        if !**game_started && *game_mode == GameMode::Sandbox {
            let pieces = vec![
                ("wp", "♙"),
                ("bp", "♟"),
                ("wr", "♖"),
                ("br", "♜"),
                ("wn", "♘"),
                ("bn", "♞"),
                ("wb", "♗"),
                ("bb", "♝"),
                ("wq", "♕"),
                ("bq", "♛"),
                ("wk", "♔"),
                ("bk", "♚"),
            ];

            html! {
                <div class="Pieces">
                    <div class="palette-pieces">
                        { for pieces.iter().map(|(piece, emoji)| {
                            let is_selected = selected_piece.as_ref().map_or(false, |sel| sel == piece);
                            let onclick = {
                                let selected_piece = selected_piece.clone();
                                let piece = piece.to_string();
                                Callback::from(move |_| selected_piece.set(Some(piece.clone())))
                            };
                            html! {
                                <div class="palette-piece">
                                    <button class={classes!("piece-button", if is_selected {"selected"} else {""})} {onclick}>
                                        { emoji }
                                    </button>
                                </div>
                            }
                        }) }
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    };

    html! {
        <div>
            <h2>{ "M-Chess Game" }</h2>
            if let Some(msg) = &**game_over_message {
                <p class="game-over">{ msg }</p>
            }
            { render_board }
            { render_palette() }
            if !**game_started {
                <button onclick={on_start.clone()}>{ "Start Game" }</button>
            } else {
                <button onclick={on_quit.clone()}>{ "Quit Game" }</button>
            }
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
    yew::Renderer::<App>::new().render();
}
