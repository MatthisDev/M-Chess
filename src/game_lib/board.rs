use crate::game_lib::piece::{Color, Piece, PieceType};
use crate::game_lib::position::Position;
use std::array::from_fn;

pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub squares: [[(isize, isize); BOARD_SIZE]; BOARD_SIZE],
    // tableau des pions
    //  -Blanc
    //  -Noir
    pub pieces: [[Option<Piece>; 16]; 2],
    pub turn: Color,
    pub history: Vec<(Position, Position, Option<Piece>)>,
}

impl Board {
    pub fn init() -> Self {
        let mut squares: [[(isize, isize); BOARD_SIZE]; BOARD_SIZE] =
            [[(-1, -1); BOARD_SIZE]; BOARD_SIZE];

        //pieces [0] => Black //piece [1] =>White
        let mut pieces: [[Piece; 16]; 2] = [[
                Piece::new(Color::Black, PieceType::Pawn, Position::new(1, 0)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(1, 1)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(1, 2)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(1, 3)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(1, 4)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(1, 5)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(1, 6)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(1, 7)),
                Piece::new(Color::Black, PieceType::Rook, Position::new(0, 0)),
                Piece::new(Color::Black, PieceType::Rook, Position::new(0, 7)),
                Piece::new(Color::Black, PieceType::Knight, Position::new(0, 1)),
                Piece::new(Color::Black, PieceType::Knight, Position::new(0, 6)),
                Piece::new(Color::Black, PieceType::Bishop, Position::new(0, 2)),
                Piece::new(Color::Black, PieceType::Bishop, Position::new(0, 5)),
                Piece::new(Color::Black, PieceType::Queen, Position::new(0, 3)),
                Piece::new(Color::Black, PieceType::King, Position::new(0, 4)),
                ],[
                Piece::new(Color::White, PieceType::Pawn, Position::new(6, 0)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(6, 1)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(6, 2)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(6, 3)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(6, 4)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(6, 5)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(6, 6)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(6, 7)),
                Piece::new(Color::White, PieceType::Rook, Position::new(7, 0)),
                Piece::new(Color::White, PieceType::Rook, Position::new(7, 7)),
                Piece::new(Color::White, PieceType::Knigth, Position::new(7, 1)),
                Piece::new(Color::White, PieceType::Knigth, Position::new(7, 6)),
                Piece::new(Color::White, PieceType::Bishop, Position::new(7, 2)),
                Piece::new(Color::White, PieceType::Bishop, Position::new(7, 5)),
                Piece::new(Color::White, PieceType::Queen, Position::new(7, 3)),
                Piece::new(Color::White, PieceType::King, Position::new(7, 4)),
                ]]; 
        
        // Put each piece in its cell on the board
        
        // Init Pawn
        for i in 0..BOARD_SIZE {
            squares[1][i] = (0, i);
            squares[6][i] = (1, i);
        }

        // Inint Rooks
        squares[0][0] = (0, 8); // black
        squares[0][7] = (0, 9); // black

        squares[7][0] = (1, 8); // white
        squares[7][7] = (1, 9); // white

        // Init Knigths
        squares[0][1] = (0, 10); // black
        squares[0][6] = (0, 11); // black

        squares[7][1] = (1, 10); // white
        squares[7][6] = (1, 11); // white

        // Init Bishops
        squares[0][2] = (0, 12); // black
        squares[0][5] = (0, 13); // black

        squares[7][2] = (1, 12); // white
        squares[7][5] = (1, 13); // white

        // Init Kings and Queens 
        squares[0][3] = (0, 14); // black
        squares[0][4] = (0, 15); // black

        squares[7][3] = (1, 14); // white
        squares[7][4] = (1, 15); // white

        Board {
            squares,
            pieces,
            turn: Color::White,
            history: Vec::new(),
        }
    }

    // #FIXME
    // display in the terminal the board
    pub fn print_board(&self) {
        println!("  a b c d e f g h");
        for row in 0..BOARD_SIZE {
            print!("{} ", 8 - row);
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[row][col] {
                    let piece_char = match piece.piece_type {
                        PieceType::King => 'K',
                        PieceType::Queen => 'Q',
                        PieceType::Rook => 'R',
                        PieceType::Bishop => 'B',
                        PieceType::Knight => 'N',
                        PieceType::Pawn => 'P',
                    };
                    let color_char = if piece.color == Color::White {
                        piece_char.to_ascii_uppercase()
                    } else {
                        piece_char.to_ascii_lowercase()
                    };
                    print!("{} ", color_char);
                } else {
                    print!(". ");
                }
            }
            println!("{}", 8 - row);
        }
        println!("  a b c d e f g h");
    }
    
    // #FIXME
    pub fn is_attacked(&self, position: &Position, color: Color) -> bool {
        todo!()
    }

    // move a piece to a position
    // must be to position in args because:
    // we dont know if there is a piece on the "from" position
    pub fn move_piece(&mut self, from: &Position, to: &Position) -> bool {

        let (i, j): (isize, isize) = self.squares[from.row][from.col];
        if i == -1 && j == -1 {return false;}
        let (i, j): (usize, usize) = (i as usize, j as usize);

        let mut piece: &Piece =
            if let Some(piece) = Piece::get_piece(from, &self) { piece } 
            else {return false}; // handle the case when there is no piece on the cell 

        if self.is_valid_move(piece, to) { 
            piece.has_moved = true;
            piece.position.row = to.row;
            piece.position.col = to.col;

            // update the case where the piece is now
            self.squares[to.row][to.col] = (i, j);
            // remove the old case where the piece moved
            self.squares[from.row][from.col] = (-1, -1);

            // change the turn
            self.turn = if self.turn == Color::White {
                Color::Black
            } else {
                Color::White
            };
            return true;
        }
        false
    }

    // #FIXME
    // look at if piece can do the move
    pub fn is_valid_move(&self, piece: &Piece, to: &Position) -> bool {

        // get all the valid moves on this postion
        let valid_moves: Vec<Position> = piece.valid_moves(&self);

        // if the move to do is in the valid move
        valid_moves.contains(&to)
    }

    pub fn is_within_bounds(&self, position: &Position) -> bool {
        position.row < BOARD_SIZE && position.col < BOARD_SIZE
    }

    pub fn is_king_in_check(&self, color: Color) -> bool {
        // get the king 
        let king: Piece = self.pieces[color][15];

        if king == None {
            panic!("is_king_in_check: Where is the king (not in the pieces's array"); 
        }

        self.is_attacked(king.position);
    }

    // #FIXME
    pub fn is_checkmate(&self, color: Color) -> bool {
        if !self.is_king_in_check(color) {
            return false;
        }

        let enemie_color: usize = if color == 1 {0} else {1};

        for enemies_i in 0..=15 {
            let piece: Piece = self.pieces[enemie_color][enemie_i];

            let valid_moves: Vec<Position> = piece.valid_moves(self, Position::new(row, col));

            // check if for all the valid move there is one move that put the king in check
            for mv in valid_moves {
                // if the move is safe for the king
                if self.is_move_safe(&piece, mv) {return false;}
            }
        }

        true
    }

    // #FIXME
    fn is_move_safe(&self, piece: &Piece, to: &Position) -> bool {
        let mut new_board = self.clone();
        new_board.move_piece(piece.position, to);
        !new_board.is_king_in_check(piece.color)
    }

    // #FIXME
    pub fn can_castle(&self, king_position: &Position, rook_position: &Position) -> bool {
        let (ik, jk) = self.squares[king_position.row][king_position.col];
        let (ir, jr) = self.squares[rook_position.row][rook_position.col];

        // we admit that it's valid positions
        let (ik, jk): (usize, usize) = (ik as usize, jk as usize);
        let (ir, jr): (usize, usize) = (ir as usize, jr as usize);

        let king: Piece = self.pieces[ik][jk];
        let rook: Piece = self.pieces[ir][jr];

        // Vérifier que le roi et la tour n'ont pas bougé
        if king.has_moved || rook.has_moved {
            return false;
        }

        // Vérifier qu'il n'y a pas de pièces entre le roi et la tour
        let (min_col, max_col) = if king_position.col < rook_position.col {
            (king_position.col + 1, rook_position.col)
        } else {
            (rook_position.col + 1, king_position.col)
        };

        // for each case between the king and the rook
        for col in min_col..max_col { 
            let pos = Position::new(king_position.row, col);

            // we check if there is piece on it
            if self.squares[king_position.row][col] != (-1, -1){
                return false;
            }
            // and if not, if the case is attacked
            else if self.is_attacked(pos, king.color) {
                return false;
            }
        }

        // finally we check if the rook is attacked
        if self.is_attacked(rook_positeon, rook.color) {
            return false;
        }

        true
    }

    // #FIXME
    pub fn perform_castle(&mut self, king_position: Position, rook_position: Position) {
        let new_king_position = if rook_position.col < king_position.col {
            // Grand roque
            Position::new(king_position.row, king_position.col - 2)
        } else {
            // Petit roque
            Position::new(king_position.row, king_position.col + 2)
        };

        let new_rook_position = if rook_position.col < king_position.col {
            // Grand roque
            Position::new(rook_position.row, rook_position.col + 3)
        } else {
            // Petit roque
            Position::new(rook_position.row, rook_position.col - 2)
        };

        // Déplacer le roi et la tour
        self.move_piece(king_position, new_king_position);
        self.move_piece(rook_position, new_rook_position);
    }

    //fn upgrade()
    //
    //
}
