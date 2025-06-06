use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::board::{self, Board, BOARD_SIZE, EMPTY_CELL, EMPTY_POS, NONE};
use crate::piece::Piece;
use crate::piece::{Color, PieceType};
use crate::position::Position;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub board: Board,
    pub nb_turn: usize,
}

impl Game {
    /// Take `bool` and create an instance of [`Game`]
    ///
    /// custom:
    /// - True = init an empty board. You must add pieces.
    /// - False = init an classic board. You cannot add new pieces.
    ///
    /// # Example
    /// ```no_run
    /// use game_lib::game::Game;
    ///
    /// let mut game1 = Game::init(false);
    /// game1.board.print_board(); // classic/full board
    ///
    /// let mut game2 = Game::init(true);
    /// game2.board.print_board(); // empty board
    /// ```
    ///
    /// # See more
    /// [`Board::add_piece`] and [`Board::remove_piece`]
    pub fn init(custom: bool) -> Self {
        use Color::*;

        let mut game = Game {
            board: if custom {
                Board::empty_init()
            } else {
                Board::full_init()
            },
            nb_turn: 0,
        };

        game
    }
    /*
     * Waited string format:
     * - <coo1>-><coo2>
     * - coo1 or coo2: [a-h][1-8]
     */
    fn parse_move_str(move_piece: &str) -> Result<(Position, Position), &'static str> {
        let count: usize = move_piece.chars().count();

        // cannot send
        if count < 6 || &move_piece[2..=3] != "->" {
            return Err("parse_move_str: invalid send string: <{move_piece}>");
        }

        let move_piece = &move_piece[0..6];

        let from_pos: Result<Position, &str> = Position::from_algebraic(&move_piece[0..=1]);
        let to_pos = Position::from_algebraic(&move_piece[4..=5]);

        if from_pos.is_err() || to_pos.is_err() {
            return Err("parse_move_str: invalid send string: <{move_piece}>");
        }

        let from_pos = from_pos.unwrap();
        let to_pos = to_pos.unwrap();

        Ok((from_pos, to_pos))
    }

    fn castle_situation(&mut self, king: &Piece, to_pos: &Position) -> bool {
        // Vérifier si le mouvement est un roque
        if king.piece_type != PieceType::King(0) {
            return false;
        }

        let rook_positions = [
            Position::new(king.position.row, 0), // Tour côté dame
            Position::new(king.position.row, 7), // Tour côté roi
        ];

        let to_king_pos = [
            Position::new(king.position.row, king.position.col - 2),
            Position::new(king.position.row, king.position.col + 2),
        ];

        for i in 0..2 {
            if *to_pos == to_king_pos[i]
                && self.board.can_castle(&king.position, &rook_positions[i])
            {
                self.board
                    .perform_castle(&king.position, &rook_positions[i]);
                return true;
            }
        }

        false
    }
    /// Try to move a [`Piece`] on the [`Board`] instance.\
    /// Take a `String` with the format `"from_cell->to_cell"`. And cell's regex is [a-h][0-8]
    ///
    /// Return `Result<bool, &'static str>`
    /// - `Ok(true)` for valid moves
    /// - `Ok(false)` for check mat or pat moves
    /// - `Err(_)` for invalid moves
    ///
    /// # Example
    ///
    /// ```no_run
    /// use game_lib::game::Game;
    ///
    /// let mut game = Game::init(false);
    /// game.make_move_algebraic("e2->e4"); // Ok(True)
    /// game.make_move_algebraic("e3->e4"); // Error(_)
    /// ```
    #[inline]
    pub fn make_move_algebraic(&mut self, moves: &str) -> Result<bool, &'static str> {
        let res = Self::parse_move_str(moves);
        if res.is_err() {
            return Err("parse_move_str: invalid send string: <{move_piece}>");
        }

        let (from_pos, to_pos) = res.unwrap();

        // get the piece and if there is not return an error
        let piece: &Piece = {
            if let Some(mut piece) = Piece::get_piece(&from_pos, &self.board) {
                &piece.clone()
            } else {
                return Err("Invalid move: There is not piece here");
            }
        };

        if piece.color != self.board.turn {
            return Err("Invalid movement.");
        }

        // upgrade pawn
        if !self.board.waiting_upgrade.is_none() {
            return Err("Waiting upgrade.");
        } else if self.perform_upgrade("q".to_string()) {
            return Ok(true);
        }

        // castle situtation
        if self.castle_situation(piece, &to_pos) {
            self.board.turn = self.board.turn.opposite();
            return Ok(true);
        }

        // if the piece can move + is moved
        if self.board.move_piece(&from_pos, &to_pos) {
            self.perform_upgrade("q".to_string());

            self.board.turn = self.board.turn.opposite();

            println!("Success!");

            // check if there is a checkmate condition
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

            // PAT SITUATION
            if self.board.is_pat(self.board.turn) || self.board.counter == 51 {
                println!("PAT! AUCUN JOUEUR GAGNE.");
                return Ok(false);
            }

            Ok(true)
        } else {
            Err("Invalid movement.")
        }
    }

    /// Take a `&str` with the format `"cell"`.
    ///
    /// Return a `Vec` of all the movement possible for a cell.\
    /// If the cell is empty or the `Piece` is the wrong color. Then the list will be empty.\
    /// If the king is in check then the list is all the movement for this `Piece` that cover the king.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use game_lib::game::Game;
    ///
    /// let mut game = Game::init(false);
    ///
    /// game.get_list_moves("e2".to_string()); // => ["e3", "e4"]
    /// game.get_list_moves("e3".to_string()); // => []
    /// game.get_list_moves("e32".to_string()); // => Err(_)
    /// ```
    pub fn get_list_moves(&mut self, cell: String) -> Result<Vec<String>, &'static str> {
        let mut result: Vec<String> = Vec::<String>::new();

        if cell.chars().count() != 2 {
            return Err("Wrong string format: too long or to short");
        }

        // convert into Position
        let position: Position = match Position::from_algebraic(&cell) {
            Ok(val) => val,
            Err(_) => return Err("Wrong string format: conversion to Positon"),
        };

        // get the piece and if there is no piece just return an empty list
        let piece: &Piece = match Piece::get_piece(&position, &self.board) {
            Some(val) => val,
            None => return Ok(vec![]),
        };

        // let lst_moves: Vec<Position> = piece.valid_moves(&self.board);
        let lst_moves: Vec<Position> = Piece::valid_moves(position, &mut self.board);

        for i in lst_moves.iter() {
            // convert Position -> String
            result.push(i.to_algebraic());
        }

        Ok(result)
    }

    /// This function will undo the last move made on the board.\
    /// It will remove the last piece moved and restore the previous state of the board.
    /// The function will also update the turn of the player.
    /// no update of the has_moved for rook and king is implemented for the moment.
    pub fn undo_move(&mut self) {
        self.board.undo_move();
    }

    pub fn skip_turn(&mut self) {
        self.board.turn = self.board.turn.opposite();
        self.board.counter += if self.board.turn == Color::White {
            1
        } else {
            0
        };
    }

    /// Check the state of the board if a pawn has to be upgrade it's return coo
    ///
    /// # Example
    /// ```no_run
    /// use game_lib::game::Game;
    ///
    /// let mut game = Game::init(false);
    ///
    /// game.has_to_upgrade(); // => None
    /// // do moves until upgrade situation
    /// game.has_to_upgrade(); // => Some(a0)
    /// ```
    pub fn has_to_upgrade(&self) -> Option<String> {
        match self.board.waiting_upgrade {
            Some(position) => Some(position.to_algebraic()),
            None => None,
        }
    }

    /// Upgrade the current pawn to upgrade on the board.
    ///
    /// Return true if the upgrade type is right
    /// Return false otherwise
    ///
    /// piece_type format:
    /// "q": Queen | "n": Knight | "r": Rook | "b": Bishop
    /// (return false for other piece's type)
    ///
    /// # Example
    /// ```no_run
    /// use game_lib::game::Game;
    ///
    /// let mut game = Game::init(false);
    ///
    /// game.perform_upgrade("b".to_string()); // => false
    ///
    /// // do moves until upgrade situation
    ///
    /// match game.has_to_upgrade() {
    ///     Some(str_pos) => game.perform_upgrade("b".to_string()), // => true
    ///     None => false
    /// };
    /// ```
    pub fn perform_upgrade(&mut self, piece_type: String) -> bool {
        let position = match self.board.waiting_upgrade {
            Some(position) => position,
            _ => return false,
        };

        let piece_type: PieceType = match PieceType::from_string(piece_type) {
            p @ (PieceType::Queen | PieceType::Knight | PieceType::Bishop | PieceType::Rook(_)) => {
                p
            }
            _ => return false,
        };

        match Piece::get_piece_mut(&position, &mut self.board) {
            Some(piece) if piece.piece_type == PieceType::Pawn => piece.piece_type = piece_type,
            _ => return false,
        }

        // reset waiting state
        self.board.waiting_upgrade = None;

        true
    }
}
