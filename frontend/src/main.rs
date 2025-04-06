use yew::prelude::*;
use game_lib::game::Game;
use game_lib::board::Board;

#[derive(PartialEq, Clone, Debug)] // Add Clone and Debug traits
enum GameMode {
    None,
    Standard,
    Sandbox,
}

#[function_component(App)]
fn app() -> Html {
    let active_tab = use_state(|| "menu".to_string());
    let game_mode = use_state(|| GameMode::None);
    let game = use_state(|| None as Option<Game>);
    let selected_piece = use_state(|| None as Option<String>); // State to track the selected piece

    let set_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| active_tab.set(tab))
    };

    let start_game = {
        let game_mode = game_mode.clone();
        let game = game.clone();
        Callback::from(move |mode: GameMode| {
            game_mode.set(mode.clone());
            let new_game = match mode {
                GameMode::Standard => Some(Game::init(false)),
                GameMode::Sandbox => Some(Game::init(true)),
                GameMode::None => None,
            };
            game.set(new_game);
        })
    };

    let render_board = |game: &Game| {
        let board_state = game.board.get();
        let game = game.clone();
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
                                let selected_piece = selected_piece.clone();
                                Callback::from(move |_|
                                    {
                                        (*game).set({
                                            let mut game = game.clone();
                                            if let Some(piece) = &*selected_piece {
                                                game.board.add_piece(&format!("{}{}", piece, position));
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
        let pieces = vec![
            "wp", "bp", "wr", "br", "wn", "bn", "wb", "bb", "wq", "bq", "wk", "bk",
        ];
        let selected_piece = selected_piece.clone();

        html! {
            <div class="Pieces">
                <h3>{ "Pieces" }</h3>
                <div class="palette-pieces">
                    { for pieces.iter().map(|piece| {
                        let onclick = {
                            let selected_piece = selected_piece.clone();
                            let piece = piece.clone();
                            Callback::from(move |_| selected_piece.set(Some(piece.to_string())))
                        };

                        html! {
                            <div class="palette-piece">
                                <button class="invisible-button" {onclick}></button>
                                { piece }
                            </div>
                        }
                    }) }
                </div>
            </div>
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
                                <h1>{ "Welcome to M-Chess!" }</h1>
                                <button onclick={start_game.reform(|_| GameMode::Standard)}>{ "Start Game" }</button>
                                <button onclick={start_game.reform(|_| GameMode::Sandbox)}>{ "Start Sandbox" }</button>
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
                            if let Some(game) = &*game {
                                html! {
                                    <div class="game-area">
                                        { render_board(game) }
                                        { render_palette() }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        },
                        GameMode::Standard => {
                            if let Some(game) = &*game {
                                html! {
                                    <div class="game-area">
                                        { render_board(game) }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        },
                        GameMode::None => html! {},
                    }
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
