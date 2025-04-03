use wasm_bindgen::prelude::*;
use yew::prelude::*;
use game_lib::game::Game; // Import Game from game_lib

enum Msg {
    InitGameBoard,
    InitSandboxBoard,
    SelectPiece(usize),
    MovePiece(usize),
}

#[derive(Clone, Copy)]
enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteRook,
    BlackRook,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

struct Model {
    link: ComponentLink<Self>,
    game: Option<Game>, // Store the Game instance
    board: [Option<Piece>; 64],
    selected: Option<usize>,
    last_move: Option<(usize, usize)>,
    is_sandbox: bool, // Track whether we're in the Sandbox tab
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut board = [None; 64];
        // Initialize an empty board by default
        Self {
            link,
            game: None,
            board,
            selected: None,
            last_move: None,
            is_sandbox: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InitGameBoard => {
                self.is_sandbox = false;
                self.game = Some(Game::init(false)); // Standard board
                self.board = [None; 64];
                // Initialize the board with standard chess setup
                true
            }
            Msg::InitSandboxBoard => {
                self.is_sandbox = true;
                self.game = Some(Game::init(true)); // Custom board
                self.board = [None; 64]; // Empty board
                true
            }
            Msg::SelectPiece(index) => {
                self.selected = Some(index);
                true
            }
            Msg::MovePiece(index) => {
                if let Some(selected) = self.selected {
                    if selected != index {
                        self.board[index] = self.board[selected];
                        self.board[selected] = None;
                        self.last_move = Some((selected, index));
                        self.selected = None;
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="tabs">
                    <button onclick=self.link.callback(|_| Msg::InitGameBoard)>{ "Game" }</button>
                    <button onclick=self.link.callback(|_| Msg::InitSandboxBoard)>{ "Sandbox" }</button>
                </div>
                { self.chessboard() }
                { self.view_last_move() }
                { self.view_board_state() } // Add the board state view here
            </div>
        }
    }
}

impl Model {
    fn chessboard(&self) -> Html {
        let squares = (0..64).map(|i| {
            let class = if (i / 8 + i % 8) % 2 == 0 {
                "white"
            } else {
                "black"
            };
            let piece = match self.board[i] {
                Some(Piece::WhitePawn) => "♙",
                Some(Piece::BlackPawn) => "♟︎",
                Some(Piece::WhiteRook) => "♖",
                Some(Piece::BlackRook) => "♜",
                Some(Piece::WhiteKnight) => "♘",
                Some(Piece::BlackKnight) => "♞",
                Some(Piece::WhiteBishop) => "♗",
                Some(Piece::BlackBishop) => "♝",
                Some(Piece::WhiteQueen) => "♕",
                Some(Piece::BlackQueen) => "♛",
                Some(Piece::WhiteKing) => "♔",
                Some(Piece::BlackKing) => "♚",
                None => "",
            };
            let onclick = if self.selected.is_some() {
                self.link.callback(move |_| Msg::MovePiece(i))
            } else {
                self.link.callback(move |_| Msg::SelectPiece(i))
            };
            html! {
                <div class={format!("square {}", class)} onclick=onclick>
                    { piece }
                </div>
            }
        });

        html! {
            <div class="chessboard">
                { for squares }
            </div>
        }
    }

    fn view_board_state(&self) -> Html {
        let board_state = self.board.iter().enumerate().map(|(i, piece)| {
            let piece_str = match piece {
                Some(Piece::WhitePawn) => "♙",
                Some(Piece::BlackPawn) => "♟︎",
                Some(Piece::WhiteRook) => "♖",
                Some(Piece::BlackRook) => "♜",
                Some(Piece::WhiteKnight) => "♘",
                Some(Piece::BlackKnight) => "♞",
                Some(Piece::WhiteBishop) => "♗",
                Some(Piece::BlackBishop) => "♝",
                Some(Piece::WhiteQueen) => "♕",
                Some(Piece::BlackQueen) => "♛",
                Some(Piece::WhiteKing) => "♔",
                Some(Piece::BlackKing) => "♚",
                None => ".",
            };
            if i % 8 == 7 {
                format!("{}<br>", piece_str) // Add a line break after every 8 squares
            } else {
                format!("{} ", piece_str)
            }
        }).collect::<String>();

        html! {
            <div class="board-state">
                <h3>{ "Board State:" }</h3>
                <pre>{ Html::from_html_unchecked(board_state.into()) }</pre>
            </div>
        }
    }

    fn view_last_move(&self) -> Html {
        if let Some((from, to)) = self.last_move {
            let from_coord = format!("{}{}", (from % 8 + 97) as u8 as char, 8 - from / 8);
            let to_coord = format!("{}{}", (to % 8 + 97) as u8 as char, 8 - to / 8);
            html! {
                <p>{ format!("Last move: {} -> {}", from_coord, to_coord) }</p>
            }
        } else {
            html! { <p>{ "No moves yet" }</p> }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
