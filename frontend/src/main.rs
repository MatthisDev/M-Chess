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
    let mut game = use_state(|| None as Option<Game>); // Track the current game state

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
                GameMode::Standard => Game::init(false), // Initialize a standard game
                GameMode::Sandbox => Game::init(true),   // Initialize a sandbox game
                GameMode::None => return,
            };
            game.set(Some(new_game)); // Update the game state
        })
    };

    let render_board = |game: &Game| {
        let board_state = game.board.get(); // Get the board as a matrix of strings
        html! {
            <div class="board">
                { for board_state.iter().enumerate().map(|(row_idx, row)| {
                    html! {
                        { for row.iter().enumerate().map(|(col_idx, cell)| {
                            let is_dark = (row_idx + col_idx) % 2 == 1; // Alternate colors based on row and column indices
                            let cell_class = if is_dark { "cell dark" } else { "cell light" };
                            html! {
                                <div class={cell_class}>
                                    {
                                        if cell != ".." {
                                            format!("{}", cell) // Display the piece (e.g., "wp", "bk")
                                        } else {
                                            "".to_string() // Empty cell
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

    // Render the palette for sandbox mode
    let render_palette = || {
        let pieces = vec![
            "wp", "bp", "wr", "br", "wn", "bn", "wb", "bb", "wq", "bq", "wk", "bk",
        ]; // List of pieces
        html! {
            <div class="palette">
                <h3>{ "Palette" }</h3>
                <div class="palette-pieces">
                    { for pieces.iter().map(|piece| {
                        html! {
                            <div class="palette-piece">
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
                                html! { <p>{ "Initializing game..." }</p> }
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
                                html! { <p>{ "Initializing game..." }</p> }
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
