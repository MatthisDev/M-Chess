use crate::app::state::ServerState;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub board: Vec<Vec<Option<String>>>,
    pub game_over: Option<String>,
    pub on_quit: Callback<()>,
}

#[function_component(Game)]
pub fn game(props: &GameProps) -> Html {
    let GameProps {
        board,
        game_over,
        on_quit,
    } = props;

    let onclick_quit = {
        let on_quit = on_quit.clone();
        Callback::from(move |_| on_quit.emit(()))
    };

    html! {
        <div>
            <h2>{ "Game Board" }</h2>
            // Ici tu peux ajouter ton rendu de plateau, pièces, etc.
            <button onclick={onclick_quit}>{ "Quit Game" }</button>
            {
                if let Some(result) = game_over {
                    html! { <p>{ format!("Game Over: {}", result) }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
