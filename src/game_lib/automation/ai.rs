use crate::game_lib::board::Board;
use crate::game_lib::game::*;
use crate::game_lib::piece::{Color, PieceType};
use crate::game_lib::position::Position;

use crate::game_lib::board::NONE;
use crate::game_lib::position;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct AI {
    difficulty: Difficulty,
    color: Color,
}

impl AI {
    pub fn new(difficulty: Difficulty, color: Color) -> Self {
        AI { difficulty, color }
    }

    fn evaluate_board(&self, board: &Board) -> i32 {
        let mut score = 0;

        // Add material value for white pieces (AI's pieces)
        for piece in board.pieces[self.color as usize].iter() {
            if piece.position.row != NONE && piece.position.col != NONE {
                score += piece.piece_type.get_value(); // Assume `value()` returns the piece's material value
            }
        }

        // Subtract material value for black pieces (opponent's pieces)
        for piece in board.pieces[self.color.opposite() as usize].iter() {
            if piece.position.row != NONE && piece.position.col != NONE {
                score -= piece.piece_type.get_value();
            }
        }

        // Add positional bonuses (optional)
        for piece in board.pieces[self.color as usize].iter() {
            if piece.piece_type == PieceType::Pawn && piece.position.is_center() {
                score += 10; // Bonus for pawns in the center
            }
        }

        for piece in board.pieces[self.color.opposite() as usize].iter() {
            if piece.piece_type == PieceType::Pawn && piece.position.is_center() {
                score -= 10; // Penalty for opponent's pawns in the center
            }
        }

        score
    }

    fn minimax(
        &self,
        board: &Board,
        depth: i32,
        is_maximizing: bool,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        // Base case: if depth is 0 or the game is over, evaluate the board
        // board.print_board();
        if depth == 0 || board.is_game_over() {
            return self.evaluate_board(board);
        }

        if is_maximizing {
            let mut max_eval = i32::MIN;

            // Get all possible moves for the maximizing player
            for piece in board.pieces[self.color as usize].iter() {
                if piece.position.row == NONE || piece.position.col == NONE {
                    continue; // Skip unused pieces
                }

                for mv in piece.valid_moves(board) {
                    let mut new_board = board.clone();
                    new_board.move_piece(&piece.position, &mv);

                    let eval = self.minimax(&new_board, depth - 1, false, alpha, beta);
                    max_eval = max_eval.max(eval);
                    alpha = alpha.max(eval);

                    if beta <= alpha {
                        break; // Beta cutoff
                    }
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;

            // Get all possible moves for the minimizing player
            for piece in board.pieces[self.color.opposite() as usize].iter() {
                if piece.position.row == NONE || piece.position.col == NONE {
                    continue; // Skip unused pieces
                }

                for mv in piece.valid_moves(board) {
                    let mut new_board = board.clone();
                    new_board.move_piece(&piece.position, &mv);

                    let eval = self.minimax(&new_board, depth - 1, true, alpha, beta);
                    min_eval = min_eval.min(eval);
                    beta = beta.min(eval);

                    if beta <= alpha {
                        break; // Alpha cutoff
                    }
                }
            }
            min_eval
        }
    }

    pub fn get_best_move(&self, board: &Board) -> (Position, Position) {
        let mut best_move = None;
        let mut best_value = i32::MIN;
        let depth = match self.difficulty {
            Difficulty::Easy => 1,
            Difficulty::Medium => 3,
            Difficulty::Hard => 5,
        };

        // Iterate over all possible moves for the AI's pieces
        for piece in board.pieces[self.color as usize].iter() {
            if piece.position.row == NONE || piece.position.col == NONE {
                continue; // Skip unused pieces
            }

            for mv in piece.valid_moves(board) {
                let mut new_board = board.clone();
                new_board.move_piece(&piece.position, &mv);

                let move_value = self.minimax(&new_board, depth, false, i32::MIN, i32::MAX); // Depth = 3

                if move_value > best_value {
                    best_value = move_value;
                    best_move = Some((piece.position, mv));
                }
            }
        }

        match best_move {
            Some((from, to)) => (from, to),
            None => (Position::new(0, 0), Position::new(1, 0)), // Default move if no valid moves
        }
    }
}
