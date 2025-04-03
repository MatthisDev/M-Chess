use crate::game_lib::game::Game;
use yew::prelude::*;

pub struct Sandbox {
    game: Game, // Store the Game instance
}

pub enum Msg {
    InitBoard,
    AddPiece(String), // Add a piece to the board
}

impl Component for Sandbox {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Initialize a custom board (empty board)
        let game = Game::init(true);
        Self { game }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InitBoard => {
                // Reinitialize the board
                self.game = Game::init(true);
                true // Re-render the component
            }
            Msg::AddPiece(piece_str) => {
                // Use the add_piece method to add a piece to the board
                if let Ok(true) = self.game.board.add_piece(&piece_str) {
                    true // Re-render the component if the piece was added successfully
                } else {
                    false // Do not re-render if the piece could not be added
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::InitBoard)}>{ "Initialize Custom Board" }</button>
                <div class="chessboard">
                    { self.render_board(link) }
                </div>
            </div>
        }
    }
}

impl Sandbox {
    fn render_board(&self, link: &Scope<Self>) -> Html {
        html! {
            for (0..64).map(|i| {
                let position = format!(
                    "{}{}",
                    (b'a' + (i % 8) as u8) as char,
                    8 - (i / 8)
                );
                let piece = self.game.board.get_piece_at(i); // Example function to get a piece at a position
                html! {
                    <div class="square">
                        { piece.map(|p| p.to_string()).unwrap_or_default() }
                        <button onclick={link.callback(move |_| Msg::AddPiece(format!("wpa{}", position)))}>{ "Add Pawn" }</button>
                    </div>
                }
            })
        }
    }
}