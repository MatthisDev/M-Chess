use crate::game_lib::board::Board;
use crate::game_lib::game::*;
use crate::game_lib::piece::Color;
use crate::game_lib::position::Position;

use super::board::NONE;
use super::position;

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

struct AI {
    difficulty: Difficulty,
}

impl AI {
    fn new(difficulty: Difficulty) -> Self {
        AI { difficulty }
    }

    fn evaluate_board(&self, board: &Board) -> i32 {
        0
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
        if depth == 0 || board.is_game_over() {
            return self.evaluate_board(board);
        }

        if is_maximizing {
            let mut max_eval = i32::MIN;

            // Get all possible moves for the maximizing player
            for piece in board.pieces[Color::White as usize].iter() {
                if piece.position.row == NONE || piece.position.col == NONE {
                    //todo! change value of eaten pieces
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
            for piece in board.pieces[Color::Black as usize].iter() {
                if piece.position.row == NONE || piece.position.col == NONE {
                    //todo!  change value of eaten pieces
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

    fn get_best_move(&self, board: &Board) -> (Position, Position) {
        let mut best_move = None;
        let mut best_value = i32::MIN;

        // Iterate over all possible moves for the AI's pieces
        for piece in board.pieces[Color::White as usize].iter() {
            if piece.position.row == NONE || piece.position.col == NONE {
                //todo! change value of eaten pieces
                continue; // Skip unused pieces
            }

            for mv in piece.valid_moves(board) {
                let mut new_board = board.clone();
                new_board.move_piece(&piece.position, &mv);

                let move_value = self.minimax(&new_board, 3, false, i32::MIN, i32::MAX); // Depth = 3

                if move_value > best_value {
                    best_value = move_value;
                    best_move = Some((piece.position, mv));
                }
            }
        }

        best_move.unwrap_or((Position::new(0, 0), Position::new(1, 0))) // Default move if no valid moves
    }
}
