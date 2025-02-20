use yew::prelude::*;

enum Msg {
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
    board: [Option<Piece>; 64],
    selected: Option<usize>,
    last_move: Option<(usize, usize)>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut board = [None; 64];
        // Initialize pawns
        for i in 8..16 {
            board[i] = Some(Piece::WhitePawn);
            board[40 + i] = Some(Piece::BlackPawn);
        }
        // Initialize other pieces
        board[0] = Some(Piece::WhiteRook);
        board[1] = Some(Piece::WhiteKnight);
        board[2] = Some(Piece::WhiteBishop);
        board[3] = Some(Piece::WhiteQueen);
        board[4] = Some(Piece::WhiteKing);
        board[5] = Some(Piece::WhiteBishop);
        board[6] = Some(Piece::WhiteKnight);
        board[7] = Some(Piece::WhiteRook);

        board[56] = Some(Piece::BlackRook);
        board[57] = Some(Piece::BlackKnight);
        board[58] = Some(Piece::BlackBishop);
        board[59] = Some(Piece::BlackQueen);
        board[60] = Some(Piece::BlackKing);
        board[61] = Some(Piece::BlackBishop);
        board[62] = Some(Piece::BlackKnight);
        board[63] = Some(Piece::BlackRook);

        Self { link, board, selected: None, last_move: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
                { self.chessboard() }
                { self.view_last_move() }
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
