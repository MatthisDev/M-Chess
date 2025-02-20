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
        let mut pieces: [[Option<Piece>; 16]; 2] = [[None; 16]; 2]; //pieces [0] => Black //piece [1] =>White

        // Initialiser les pions
        for col in 0..BOARD_SIZE {
            squares[1][col] = (0, col as isize);
            pieces[0][col] = Some(Piece::new(Color::Black, PieceType::Pawn));

            pieces[1][col] = Some(Piece::new(Color::White, PieceType::Pawn));
            squares[6][col] = (1, col as isize);
        }

        // Initialiser les tours
        squares[0][0] = (0, 8);
        pieces[0][8] = Some(Piece::new(Color::Black, PieceType::Rook));
        squares[0][7] = (0, 9);
        pieces[0][9] = Some(Piece::new(Color::Black, PieceType::Rook));

        squares[7][0] = (1, 8);
        pieces[1][8] = Some(Piece::new(Color::White, PieceType::Rook));
        squares[7][7] = (1, 9);
        pieces[1][9] = Some(Piece::new(Color::White, PieceType::Rook));

        // Initialiser les cavaliers
        squares[0][1] = (0, 10);
        pieces[0][10] = Some(Piece::new(Color::Black, PieceType::Knight));
        squares[0][6] = (0, 11);
        pieces[0][11] = Some(Piece::new(Color::Black, PieceType::Knight));
        squares[7][1] = (1, 10);
        pieces[1][10] = Some(Piece::new(Color::White, PieceType::Knight));
        squares[7][6] = (1, 11);
        pieces[1][11] = Some(Piece::new(Color::White, PieceType::Knight));

        // Initialiser les fous
        squares[0][2] = (0, 12);
        pieces[0][12] = Some(Piece::new(Color::Black, PieceType::Bishop));
        squares[0][5] = (0, 13);
        pieces[0][13] = Some(Piece::new(Color::Black, PieceType::Bishop));
        squares[7][2] = (1, 12);
        pieces[1][12] = Some(Piece::new(Color::White, PieceType::Bishop));
        squares[7][5] = (1, 13);
        pieces[1][13] = Some(Piece::new(Color::White, PieceType::Bishop));

        // Initialiser les rois et reines
        squares[0][3] = (0, 14);
        pieces[0][14] = Some(Piece::new(Color::Black, PieceType::Queen));
        squares[0][4] = (0, 15);
        pieces[0][15] = Some(Piece::new(Color::Black, PieceType::King));
        squares[7][3] = (1, 14);
        pieces[1][14] = Some(Piece::new(Color::White, PieceType::Queen));
        squares[7][4] = (1, 15);
        pieces[1][15] = Some(Piece::new(Color::White, PieceType::King));

        Board {
            squares,
            pieces,
            turn: Color::White,
            history: Vec::new(),
        }
    }

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

    pub fn is_attacked(&self, position: Position, color: Color) -> bool {
        todo!();
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> bool {
        if let Some(mut piece) = self.squares[from.row][from.col] {
            if self.is_valid_move(from, to) {
                piece.has_moved = true;
                self.squares[to.row][to.col] = Some(piece);
                self.squares[from.row][from.col] = None;
                self.turn = if self.turn == Color::White {
                    Color::Black
                } else {
                    Color::White
                };
                return true;
            }
        }
        false
    }

    pub fn is_valid_move(&self, from: Position, to: Position) -> bool {
        if let Some(piece) = self.squares[from.row][from.col] {
            let valid_moves = piece.valid_moves(self, from);
            valid_moves.contains(&to)
        } else {
            false
        }
    }

    pub fn is_within_bounds(&self, position: Position) -> bool {
        position.row < BOARD_SIZE && position.col < BOARD_SIZE
    }

    pub fn is_king_in_check(&self, color: Color) -> bool {
        let king_position = self.find_king(color);
        if let Some(position) = king_position {
            self.is_attacked(position, color)
        } else {
            false
        }
    }

    fn find_king(&self, color: Color) -> Option<Position> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[row][col] {
                    if piece.color == color && piece.piece_type == PieceType::King {
                        return Some(Position::new(row, col));
                    }
                }
            }
        }
        None
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        if !self.is_king_in_check(color) {
            return false;
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[row][col] {
                    if piece.color == color {
                        let valid_moves = piece.valid_moves(self, Position::new(row, col));
                        for mv in valid_moves {
                            if self.is_move_safe(piece, Position::new(row, col), mv) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    fn is_move_safe(&self, piece: Piece, from: Position, to: Position) -> bool {
        let mut new_board = self.clone();
        new_board.move_piece(from, to);
        !new_board.is_king_in_check(piece.color)
    }

    pub fn can_castle(&self, king_position: Position, rook_position: Position) -> bool {
        let king = self.squares[king_position.row][king_position.col].unwrap();
        let rook = self.squares[rook_position.row][rook_position.col].unwrap();

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

        for col in min_col..max_col {
            if self.squares[king_position.row][col].is_some() {
                return false;
            }
        }

        // Vérifier que le roi n'est pas en échec et ne traverse pas des cases attaquées
        for col in min_col..=max_col {
            let pos = Position::new(king_position.row, col);
            if self.is_attacked(pos, king.color) {
                return false;
            }
        }

        true
    }

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
