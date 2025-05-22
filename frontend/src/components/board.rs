use crate::state::{use_board_state, use_possible_moves, use_selected_piece};
use yew::prelude::*;

pub fn render_board() -> Html {
    let (board_state, _) = use_board_state();
    let (possible_moves, _) = use_possible_moves();
    let (selected_piece, set_selected_piece) = use_selected_piece();

    html! {
        <div class="board">
            { for board_state.iter().enumerate().map(|(row_idx, row)| html! {
                <div class="row">
                    { for row.iter().enumerate().map(|(col_idx, cell)| {
                        let is_dark = (row_idx + col_idx) % 2 == 1;
                        let class = if is_dark { "cell dark" } else { "cell light" };

                        html! {
                            <div class={class}>
                                { cell.as_ref().map_or("", |p| p).to_string() }
                            </div>
                        }
                    }) }
                </div>
            }) }
        </div>
    }
}