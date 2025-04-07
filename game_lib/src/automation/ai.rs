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

impl AI {
    pub fn new(difficulty: Difficulty, color: Color) -> Self {
        AI { difficulty, color }
    }

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
            for mv in Piece::valid_moves(piece.position, &mut board.clone()) {
                let mut new_board = board.clone();
                new_board.move_piece(&piece.position, &mv);
                new_board.turn = new_board.turn.opposite(); // Update the turn after making a move

                let move_value =
                    self.recursive_minimax(&mut new_board, 0, depth, false, i32::MIN, i32::MAX);

                if move_value > best_value {
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
            for mv in Piece::valid_moves(piece.position, &mut board.clone()) {
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
            let move_value =
                self.recursive_minimax(board, depth + 1, max_depth, !is_maximizing, alpha, beta);

            // Undo the move
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
}
