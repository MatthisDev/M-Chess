use game_lib::board::Board;
use game_lib::game::{self, Game};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(PartialEq, Clone, Debug)] // Add Clone and Debug traits
enum GameMode {
    Standard,
    Sandbox,
}

fn get_piece_emoji(piece: &str) -> &str {
    match piece {
        "wp" => "♙", // White Pawn
        "bp" => "♟", // Black Pawn
        "wr" => "♖", // White Rook
        "br" => "♜", // Black Rook
        "wn" => "♘", // White Knight
        "bn" => "♞", // Black Knight
        "wb" => "♗", // White Bishop
        "bb" => "♝", // Black Bishop
        "wq" => "♕", // White Queen
        "bq" => "♛", // Black Queen
        "wk" => "♔", // White King
        "bk" => "♚", // Black King
        _ => "",     // Empty cell or invalid piece
    }
}

#[function_component(App)]
fn app() -> Html {
    let active_tab = use_state(|| "menu".to_string());
    let game_mode = use_state(|| GameMode::Sandbox);
    let used_game = use_state(|| Game::init(true));
    let selected_piece = use_state(|| None as Option<String>); // State to track the selected piece
    let game_started = use_state(|| false); // State to track if the game has started
    let game_over_message = use_state(|| None as Option<String>);

    let set_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| active_tab.set(tab))
    };

    let start_game = {
        let game_mode = game_mode.clone();
        let game_started = game_started.clone();
        let used_game_clone = used_game.clone();
        Callback::from(move |mode: GameMode| {
            match mode {
                GameMode::Standard => {
                    game_mode.set(GameMode::Standard);
                    game_started.set(true); // Start the game without resetting the board
                }
                GameMode::Sandbox => {
                    game_mode.set(GameMode::Sandbox);
                    game_started.set(false); // Reset the game
                    used_game_clone.set(Game::init(true)); // Reset the board
                }
            }
        })
    };

    let start_game_from_menu = {
        let game_mode = game_mode.clone();
        let game_started = game_started.clone();
        let used_game_clone = used_game.clone();
        Callback::from(move |_| {
            game_mode.set(GameMode::Standard); // Set the game mode to Standard
            game_started.set(true); // Start the game
            used_game_clone.set(Game::init(false)); // Initialize a full board with all pieces
        })
    };

    let start_game_from_palette = {
        let game_started = game_started.clone();
        let selected_piece = selected_piece.clone();
        let game_mode = game_mode.clone();
        Callback::from(move |_| {
            game_started.set(true); // Mark the game as started
            selected_piece.set(None); // Clear the selected piece
            game_mode.set(GameMode::Standard); // Switch to Standard mode for moving pieces
        })
    };

    let move_piece = {
        let used_game = used_game.clone();
        let game_mode = game_mode.clone();
        let game_started = game_started.clone();
        let game_over_message = game_over_message.clone();
        Callback::from(move |move_str: String| {
            used_game.set({
                let mut game = (*used_game).clone();
                if let Err(err) = game.make_move_algebraic(&move_str) {
                    web_sys::console::log_1(&format!("Invalid move: {}", err).into());
                } else {
                    // Vérifiez si la partie est terminée
                    if game.board.is_game_over() {
                        game_started.set(false); // Arrêtez la partie
                        let message = if game.board.is_checkmate(game.board.turn) {
                            format!(
                                "Checkmate ! {} won.",
                                if game.board.turn == game_lib::piece::Color::White {
                                    "Black"
                                } else {
                                    "White"
                                }
                            )
                        } else {
                            "Pat ! No one won".to_string()
                        };
                        game_over_message.set(Some(message));
                    }
                }
                game
            });
        })
    };

    let render_board = {
        let board_state = (*used_game).board.get().clone();
        let selected_piece = selected_piece.clone(); // Track the selected piece
        let used_game = used_game.clone(); // Clone the game state
        let game_mode = game_mode.clone(); // Clone the game mode state
        let selected_cell = use_state(|| None as Option<String>); // Track the selected cell for movement

        html! {
            <div class="board">
                { for board_state.iter().enumerate().map(|(row_idx, row)| {
                    html! {
                        { for row.iter().enumerate().map(|(col_idx, cell)| {
                            let is_dark = (row_idx + col_idx) % 2 == 1;
                            let mut cell_class = if is_dark { "cell dark".to_string() } else { "cell light".to_string() };

                            let position = format!("{}{}", (b'a' + col_idx as u8) as char,  row_idx);

                            // Add the "selected" class if this cell is selected
                            if let Some(selected) = &*selected_cell {
                                if selected == &position {
                                    cell_class = format!("{} selected", cell_class);
                                }
                            }

                            let onclick = {
                                let selected_piece = selected_piece.clone();
                                let used_game = used_game.clone();
                                let game_mode = game_mode.clone();
                                let selected_cell = selected_cell.clone();
                                let move_piece = move_piece.clone();
                                Callback::from(move |_| {
                                    if *game_mode == GameMode::Sandbox {
                                        // Sandbox mode: Place or remove a piece
                                        if let Some(piece) = &*selected_piece {
                                            let mut game = (*used_game).clone();
                                            game.board.add_piece(&format!("{}{}", piece, position));
                                            used_game.set(game); // Update the game state
                                        } else {
                                            web_sys::console::log_1(&"No piece selected!".into());
                                        }
                                    } else if *game_mode == GameMode::Standard {
                                        // Standard mode: Move a piece
                                        if let Some(from) = &*selected_cell {
                                            // Attempt to move the piece
                                            let move_str = format!("{}->{}", from, position);
                                            move_piece.emit(move_str);
                                            selected_cell.set(None); // Reset selection
                                        } else {
                                            // Select the current cell
                                            selected_cell.set(Some(position.clone()));
                                        }
                                    }
                                })
                            };

                            html! {
                                <div class={cell_class} {onclick}>
                                    {
                                        if cell != ".." {
                                            html! {
                                                <div class="piece">{ get_piece_emoji(cell) }</div>
                                            }
                                        } else {
                                            html! { "" }
                                        }
                                    }
                                </div>
                            }
                        }) }
                    }
                }) }
            </div>
        }
    };

    let render_palette = || {
        if *game_started {
            html! {} // Do not render the palette if the game has started
        } else {
            let pieces = vec![
                ("wp", "♙"),
                ("bp", "♟"), // Pawns
                ("wr", "♖"),
                ("br", "♜"), // Rooks
                ("wn", "♘"),
                ("bn", "♞"), // Knights
                ("wb", "♗"),
                ("bb", "♝"), // Bishops
                ("wq", "♕"),
                ("bq", "♛"), // Queens
                ("wk", "♔"),
                ("bk", "♚"), // Kings
            ];
            let selected_piece = selected_piece.clone();

            html! {
                <div class="Pieces">
                    <div class="palette-pieces">
                        { for pieces.iter().map(|(piece, emoji)| {
                            let is_selected = selected_piece.as_ref().map_or(false, |selected| selected == piece);
                            let onclick = {
                                let selected_piece = selected_piece.clone();
                                let piece = (*piece).to_string();
                                Callback::from(move |_| selected_piece.set(Some(piece.clone())))
                            };
                            html! {
                                <div class="palette-piece">
                                    <button
                                        class={classes!("piece-button", if is_selected { "selected" } else { "" })}
                                        {onclick}
                                    >
                                        { emoji }
                                    </button>
                                </div>
                            }
                        }) }
                    </div>
                    <button class="start-game-button" onclick={start_game_from_palette}>{ "Start Game" }</button>
                </div>
            }
        }
    };

    html! {
        <div class="app-container">
            <nav class="navbar">
                <button onclick={set_tab.reform(|_| "menu".to_string())}>{ "Menu" }</button>
                <button onclick={set_tab.reform(|_| "description".to_string())}>{ "Description" }</button>
                <button onclick={set_tab.reform(|_| "install".to_string())}>{ "Install" }</button>
            </nav>
            <div class="content">
                {
                    match active_tab.as_str() {
                        "menu" => html! {
                            <div class="menu-container">
                                <div class="menu-buttons">
                                    <button onclick={start_game_from_menu}>{ "Start Game" }</button>
                                    <button onclick={start_game.reform(|_| GameMode::Sandbox)}>{ "Start Sandbox" }</button>
                                </div>
                            </div>
                        },
                        "description" => html! {
                            <>
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
                            </>
                        },
                        "install" => html! {
                            <>
                                <div id="download" class="tab-content">
                                    <h2>{ "Download" }</h2>
                                    <a href="../project.zip" download="M-Chess_Project.zip">{ "Download the project" }</a><br />
                                    <a href="../M-Chess_Report.pdf" download="M-Chess_Report.pdf">{ "Download the report (PDF)" }</a>
                                </div>
                            </>
                        },
                        _ => html! { <h1>{ "404: Not Found" }</h1> },
                    }
                }
                {
                    match *game_mode {
                        GameMode::Sandbox => {
                            html! {
                                <div class="game-area">
                                    { render_board }
                                    { render_palette() }
                                </div>
                            }
                        },
                        GameMode::Standard => {
                            html! {
                                <div class="game-area">
                                    { render_board }
                                </div>
                            }
                        },
                    }
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
