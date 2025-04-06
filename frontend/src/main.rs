use yew::prelude::*;
use game_lib::game::{self, Game};
use game_lib::board::Board;

#[derive(PartialEq, Clone, Debug)] // Add Clone and Debug traits
enum GameMode {
    Standard,
    Sandbox,
}

#[function_component(App)]
fn app() -> Html {
    let active_tab = use_state(|| "menu".to_string());
    let game_mode = use_state(|| GameMode::Sandbox);
    let used_game = use_state(|| Game::init(true));
    let selected_piece = use_state(|| None as Option<String>); // State to track the selected piece
    let game_started = use_state(|| false); // State to track if the game has started

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
        Callback::from(move |_| {
            game_started.set(true);
            selected_piece.set(None); // Start the game without resetting the board
        })
    };

    let render_board = {
        let board_state = (*used_game).board.get().clone();
        let selected_piece = selected_piece.clone();
        html! {
            <div class="board">
                { for board_state.iter().enumerate().map(|(row_idx, row)| {
                    html! {
                        { for row.iter().enumerate().map(|(col_idx, cell)| {
                            let is_dark = (row_idx + col_idx) % 2 == 1;
                            let cell_class = if is_dark { "cell dark" } else { "cell light" };
                            let position = format!("{}{}", (b'a' + col_idx as u8) as char, 8 - row_idx);

                            let onclick = {
                                let used_game = used_game.clone();
                                let selected_piece = selected_piece.clone();
                                Callback::from(move |_| {
                                    used_game.set({
                                        let mut game = (*used_game).clone();
                                        if let Some(piece) = &*selected_piece {
                                            game.board.add_piece(&format!("{}{}", piece, position)).unwrap_or(false);
                                        }
                                        game
                                    })
                                })
                            };

                            html! {
                                <div class={cell_class}>
                                    <button class="invisible-button" {onclick}></button>
                                    {
                                        if cell != ".." {
                                            format!("{}", cell)
                                        } else {
                                            "".to_string()
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
                ("wp", "♙"), ("bp", "♟"), // Pawns
                ("wr", "♖"), ("br", "♜"), // Rooks
                ("wn", "♘"), ("bn", "♞"), // Knights
                ("wb", "♗"), ("bb", "♝"), // Bishops
                ("wq", "♕"), ("bq", "♛"), // Queens
                ("wk", "♔"), ("bk", "♚"), // Kings
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
                        "description" => html! { <h1>{ "Description Section" }</h1> },
                        "install" => html! { <h1>{ "Install Section" }</h1> },
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
