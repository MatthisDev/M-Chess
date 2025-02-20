use crate::game_lib::board::{Board, BOARD_SIZE};
use crate::game_lib::piece::{Color, PieceType};
use crate::game_lib::position::Position;

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn init() -> Self {
        Game {
            board: Board::init(),
        }
    }
    /*
     * Waited string format:
     * - <coo1>-><coo2>
     * - coo1 or coo2: [a-h][1-8]
     */
    fn parse_move_str(move_piece: &str) -> (Position, Position) {
        let count: usize = move_piece.chars().count();

        // cannot send
        if count != 6 || &move_piece[2..=3] != "->" {
            panic!("parse_move_str: invalid send string");
        }

        let from_pos: Position = Position::from_algebraic(&move_piece[0..=1]);
        let to_pos: Position = Position::from_algebraic(&move_piece[4..=5]);

        (from_pos, to_pos)
    }

    pub fn make_move_algebraic(&mut self, moves: &str) -> Result<bool, &'static str> {
        let (from_pos, to_pos) = Self::parse_move_str(moves);

        if let Some(piece) = self.board.squares[from_pos.row][from_pos.col] {
            if piece.piece_type == PieceType::King {
                // Vérifier si le mouvement est un roque
                let rook_positions = [
                    Position::new(from_pos.row, 0), // Tour côté dame
                    Position::new(from_pos.row, 7), // Tour côté roi
                ];

                for rook_position in rook_positions.iter() {
                    if self.board.can_castle(from_pos, *rook_position)
                        && (to_pos == Position::new(from_pos.row, from_pos.col - 2)
                            || to_pos == Position::new(from_pos.row, from_pos.col + 2))
                    {
                        self.board.perform_castle(from_pos, *rook_position);
                        self.board.history.push((
                            from_pos,
                            to_pos,
                            self.board.squares[from_pos.row][from_pos.col], //Option<Piece> de la case
                        ));
                        return Ok(true);
                    }
                }
            }
        }
        if self.board.is_valid_move(from_pos, to_pos) {
            self.board.move_piece(from_pos, to_pos);
            self.board.history.push((
                from_pos,
                to_pos,
                self.board.squares[from_pos.row][from_pos.col],
            ));

            if self.board.is_king_in_check(self.board.turn) {
                self.undo_move();
                return Err("Le roi est toujours en échec après ce mouvement.");
            }
            println!("Success!");

            if self.board.is_checkmate(self.board.turn) {
                println!(
                    "Échec et mat! Le joueur {} a gagné.",
                    if self.board.turn == Color::White {
                        "Noir"
                    } else {
                        "Blanc"
                    }
                );
                // Game End
                return Ok(false);
            }
            Ok(true)
        } else {
            Err("Mouvement invalide.")
        }
    }

    fn undo_move(&mut self) {
        if let Some((from, to, _)) = self.board.history.pop() {
            self.board.move_piece(to, from);
            self.board.turn = if self.board.turn == Color::White {
                Color::Black
            } else {
                Color::White
            };
        }
    }
}
