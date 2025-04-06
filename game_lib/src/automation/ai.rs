
use crate::board::Board;
use crate::game::*;
use crate::piece::{Color, Piece, PieceType};
use crate::position::Position;

use crate::board::NONE;
use crate::position;
use std::collections::HashMap;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct AI {
    difficulty: Difficulty,
    color: Color,
}
/*
impl AI {
    pub fn new(difficulty: Difficulty, color: Color) -> Self {
        AI { difficulty, color }
    }

    fn evaluate_board(&self, board: &Board) -> i32 {
        let mut score = 0;

        // Material value
        for piece in board.pieces[self.color as usize].iter() {
            if piece.position.row != NONE && piece.position.col != NONE {
                score += piece.piece_type.get_value();
            }
        }
        for piece in board.pieces[self.color.opposite() as usize].iter() {
            if piece.position.row != NONE && piece.position.col != NONE {
                score -= piece.piece_type.get_value();
            }
        }

        // Positional bonuses
        for piece in board.pieces[self.color as usize].iter() {
            if piece.position.row != NONE && piece.position.col != NONE {
                match piece.piece_type {
                    PieceType::Pawn => {
                        if piece.position.is_center() {
                            score += 10; // Bonus for pawns in the center
                        }
                        if board.is_pawn_isolated(&piece.position) {
                            score -= 5; // Penalty for isolated pawns
                        }
                        if board.is_pawn_doubled(&piece.position) {
                            score -= 5; // Penalty for doubled pawns
                        }
                    }
                    PieceType::Rook => {
                        if board.is_open_file(piece.position.col) {
                            score += 20; // Bonus for rooks on open files
                        }
                    }
                    PieceType::Knight => {
                        if piece.position.is_center() {
                            score += 15; // Bonus for knights in the center
                        }
                    }
                    PieceType::Bishop => {
                        score += 10; // General bonus for bishops
                    }
                    PieceType::King => {
                        if board.is_king_exposed(&piece.position) {
                            score -= 30; // Penalty for exposed king
                        }
                    }
                    _ => {}
                }
            }
        }

        // Opponent's positional penalties
        for piece in board.pieces[self.color.opposite() as usize].iter() {
            if piece.position.row != NONE && piece.position.col != NONE {
                match piece.piece_type {
                    PieceType::Pawn => {
                        if piece.position.is_center() {
                            score -= 10; // Penalty for opponent's pawns in the center
                        }
                        if board.is_pawn_isolated(&piece.position) {
                            score += 5; // Bonus for opponent's isolated pawns
                        }
                        if board.is_pawn_doubled(&piece.position) {
                            score += 5; // Bonus for opponent's doubled pawns
                        }
                    }
                    PieceType::Rook => {
                        if board.is_open_file(piece.position.col) {
                            score -= 20; // Penalty for opponent's rooks on open files
                        }
                    }
                    PieceType::Knight => {
                        if piece.position.is_center() {
                            score -= 15; // Penalty for opponent's knights in the center
                        }
                    }
                    PieceType::King => {
                        if board.is_king_exposed(&piece.position) {
                            score += 30; // Bonus for opponent's exposed king
                        }
                    }
                    _ => {}
                }
            }
        }

        // Mobility (number of valid moves)
        let my_mobility = board.get_all_valid_moves(self.color).len() as i32;
        let opponent_mobility = board.get_all_valid_moves(self.color.opposite()).len() as i32;
        score += my_mobility - opponent_mobility;

        score
    }

    /*
    pub fn get_best_move(&self, board: &Board) -> Option<(Position, Position)> {
        let mut best_move = None;
        let mut best_value = i32::MIN;
        let depth = match self.difficulty {
            Difficulty::Easy => 1,
            Difficulty::Medium => 3,
            Difficulty::Hard => 5,
        };

        let mut transposition_table = HashMap::new();

        // Iterate over all possible moves for the AI's pieces
        for piece in board.pieces[self.color as usize].iter() {
            if piece.position.row == NONE || piece.position.col == NONE {
                println!("Skipping unused piece");
                continue; // Skip unused pieces
            }
            println!("Valid moves: {:?}", piece.valid_moves(board));
            for mv in piece.valid_moves(board) {
                let mut new_board = board.clone();
                new_board.move_piece(&piece.position, &mv);
                println!("Moved piece to virtual {:?}", mv);
                new_board.turn = new_board.turn.opposite(); // Update the turn after making a move

                let move_value = self.iterative_minimax(
                    &mut new_board,
                    depth,
                    false,
                    i32::MIN,
                    i32::MAX,
                    &mut transposition_table,
                );

                if move_value > best_value {
                    println!(
                        "Updating best move: {:?} -> {:?}, value: {}",
                        piece.position, mv, move_value
                    );
                    best_value = move_value;
                    best_move = Some((piece.position, mv));
                }
            }
        }

        match best_move {
            Some((from, to)) => Some((from, to)),
            None => {
                println!("No valid moves found for AI");
                None // Default move if no valid moves
            }
        }
    }

    fn iterative_minimax(
        &self,
        board: &mut Board,
        max_depth: i32,
        is_maximizing: bool,
        mut alpha: i32,
        mut beta: i32,
        transposition_table: &mut HashMap<u64, i32>,
    ) -> i32 {
        let mut stack = Vec::new();
        let mut best_value_stack = Vec::new(); // Stack to track best_value for each depth
        let mut board_stack = Vec::new(); // Stack to track board state for each depth

        // Initialize the stack with the root node
        stack.push((0, is_maximizing, alpha, beta, None));
        best_value_stack.push(if is_maximizing { i32::MIN } else { i32::MAX });
        board_stack.push(board.clone()); // Push the initial board state

        while let Some((depth, is_maximizing, mut alpha, mut beta, prev_move)) = stack.pop() {
            // Get the current best_value for this depth
            let mut best_value = best_value_stack.pop().unwrap();
            let mut current_board = board_stack.pop().unwrap(); // Get the board state for this depth

            if let Some((from, to)) = prev_move {
                println!("Undoing move: {:?} -> {:?}", from, to);
                current_board.undo_move();
            }

            if depth == max_depth || current_board.is_game_over() {
                let eval = self.evaluate_board(&current_board);
                best_value = if is_maximizing {
                    best_value.max(eval)
                } else {
                    best_value.min(eval)
                };

                // Propagate best_value back to the parent level
                if let Some(parent_best_value) = best_value_stack.last_mut() {
                    if is_maximizing {
                        *parent_best_value = (*parent_best_value).max(best_value);
                    } else {
                        *parent_best_value = (*parent_best_value).min(best_value);
                    }
                }
                continue;
            }

            let mut moves = Vec::new();
            let color = if is_maximizing {
                self.color
            } else {
                self.color.opposite()
            };

            for piece in current_board.pieces[color as usize].iter() {
                if piece.position.row == NONE || piece.position.col == NONE {
                    continue;
                }
                for mv in piece.valid_moves(&current_board) {
                    moves.push((piece.position, mv));
                }
            }

            moves.sort_by_key(|(from, to)| {
                let mut priority = 0;

                // Prioritize safe moves
                if current_board.is_safe_move(from, to) {
                    priority -= 100;
                }

                // Prioritize castling
                if current_board.is_castling_move(from, to) {
                    priority -= 90;
                }

                // Prioritize moves involving strong pieces
                if let Some(piece) = Piece::get_piece(from, &current_board) {
                    match piece.piece_type {
                        PieceType::Queen => priority -= 80,
                        PieceType::Rook => priority -= 70,
                        PieceType::Bishop => priority -= 60,
                        PieceType::Knight => priority -= 50,
                        PieceType::Pawn => priority -= 40,
                        _ => {}
                    }
                }

                // Prioritize captures
                if current_board.is_capture(to) {
                    if let Some(captured_piece) = Piece::get_piece(from, &current_board) {
                        priority -= captured_piece.piece_type.get_value();
                    }
                }

                if current_board.is_pawn_double_move(from, to) {
                    priority -= 30;
                }
                priority
            });

            // Push the current state back onto the stack
            stack.push((depth, is_maximizing, alpha, beta, None));
            best_value_stack.push(best_value);
            board_stack.push(current_board.clone()); // Push the current board state

            // Push child nodes onto the stack
            for (from, to) in moves {
                let mut new_board = current_board.clone(); // Clone the board for the child node
                if new_board.move_piece(&from, &to) {
                    new_board.turn = if is_maximizing {
                        self.color.opposite()
                    } else {
                        self.color
                    };

                    stack.push((depth + 1, !is_maximizing, alpha, beta, Some((from, to))));
                    best_value_stack.push(if !is_maximizing { i32::MIN } else { i32::MAX });
                    board_stack.push(new_board); // Push the new board state

                    if is_maximizing {
                        alpha = alpha.max(best_value);
                        if beta <= alpha {
                            break;
                        }
                    } else {
                        beta = beta.min(best_value);
                        if beta <= alpha {
                            break;
                        }
                    }
                }
            }
        }

        // Return the best_value for the root node
        best_value_stack.pop().unwrap()
    }
    */

    pub fn get_best_move(&self, board: &Board) -> Option<(Position, Position)> {
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
                println!("Skipping unused piece");
                continue; // Skip unused pieces
            }
            for mv in piece.valid_moves(board) {
                let mut new_board = board.clone();
                new_board.move_piece(&piece.position, &mv);
                println!("Moved piece to virtual {:?}", mv);
                new_board.turn = new_board.turn.opposite(); // Update the turn after making a move

                let move_value =
                    self.recursive_minimax(&mut new_board, 0, depth, false, i32::MIN, i32::MAX);

                if move_value > best_value {
                    println!(
                        "Updating best move: {:?} -> {:?}, value: {}",
                        piece.position, mv, move_value
                    );
                    best_value = move_value;
                    best_move = Some((piece.position, mv));
                }
            }
        }

        match best_move {
            Some((from, to)) => Some((from, to)),
            None => {
                println!("No valid moves found for AI");
                None // Default move if no valid moves
            }
        }
    }

    fn recursive_minimax(
        &self,
        board: &mut Board,
        depth: i32,
        max_depth: i32,
        is_maximizing: bool,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        board.turn = if is_maximizing {
            self.color
        } else {
            self.color.opposite()
        };

        // Base case: if we reach max depth or the game is over
        if depth == max_depth || board.is_game_over() {
            let eval = self.evaluate_board(board);
            return eval;
        }

        let mut best_value = if is_maximizing { i32::MIN } else { i32::MAX };

        // Get the color of the current player
        let color = if is_maximizing {
            self.color
        } else {
            self.color.opposite()
        };

        // Generate all valid moves for the current player
        let mut moves = Vec::new();
        for piece in board.pieces[color as usize].iter() {
            if piece.position.row == NONE || piece.position.col == NONE {
                continue;
            }
            for mv in piece.valid_moves(board) {
                moves.push((piece.position, mv));
            }
        }

        // Sort moves by priority (optional, improves alpha-beta pruning efficiency)
        moves.sort_by_key(|(from, to)| {
            let mut priority = 0;

            // Prioritize safe moves
            if board.is_safe_move(from, to) {
                priority -= 100;
            }

            // Prioritize castling
            if board.is_castling_move(from, to) {
                priority -= 90;
            }

            // Prioritize moves involving strong pieces
            if let Some(piece) = Piece::get_piece(from, board) {
                match piece.piece_type {
                    PieceType::Queen => priority -= 80,
                    PieceType::Rook => priority -= 70,
                    PieceType::Bishop => priority -= 60,
                    PieceType::Knight => priority -= 50,
                    PieceType::Pawn => priority -= 40,
                    _ => {}
                }
            }

            // Prioritize captures
            if board.is_capture(to) {
                if let Some(captured_piece) = Piece::get_piece(to, board) {
                    priority -= captured_piece.piece_type.get_value();
                }
            }

            if board.is_pawn_double_move(from, to) {
                priority -= 30;
            }

            priority
        });

        // Explore each move
        for (from, to) in moves {
            // Make the move
            board.move_piece(&from, &to);

            // Recursively evaluate the move
            println!("Entering recursive_minimax with depth: {}", depth + 1);
            let move_value =
                self.recursive_minimax(board, depth + 1, max_depth, !is_maximizing, alpha, beta);
            println!("Returned from recursive_minimax with value: {}", move_value);

            // Undo the move
            println!("Undoing move: {:?} -> {:?}", from, to);
            board.undo_move();
            board.turn = if is_maximizing {
                self.color.opposite()
            } else {
                self.color
            };

            // Update best_value
            if is_maximizing {
                best_value = best_value.max(move_value);
                alpha = alpha.max(best_value);
            } else {
                best_value = best_value.min(move_value);
                beta = beta.min(best_value);
            }

            // Alpha-beta pruning
            if beta <= alpha {
                break;
            }
        }

        best_value
    }
}
*/
