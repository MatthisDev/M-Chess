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
    let game = use_state(|| None as Option<Game>); // Track the current game state

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

    // Render the board from the current game
    let render_board = |game: &Game| {
        html! {
            <div class="board">
                { for game.board.squares.iter().enumerate().map(|(row_idx, row)| {
                    html! {
                        <div class="row">
                            { for row.iter().enumerate().map(|(col_idx, _square)| {
                                html! {
                                    <div class="cell">
                                        { format!("({}, {})", row_idx, col_idx) } // Placeholder for piece rendering
                                    </div>
                                }
                            }) }
                        </div>
                    }
                }) }
            </div>
        }
    };

    html! {
        <div>
            <nav>
                <button onclick={set_tab.reform(|_| "menu".to_string())}>{ "Menu" }</button>
                <button onclick={set_tab.reform(|_| "description".to_string())}>{ "Description" }</button>
                <button onclick={set_tab.reform(|_| "install".to_string())}>{ "Install" }</button>
            </nav>
            <div>
                {
                    match active_tab.as_str() {
                        "menu" => html! {
                            <div>
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
                        GameMode::Standard | GameMode::Sandbox => {
                            if let Some(game) = &*game {
                                html! {
                                    <div>
                                        <h2>{ format!("Game Mode: {:?}", game_mode) }</h2>
                                        <h3>{ format!("Turn: {:?}", game.board.turn) }</h3>
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
