use crate::game_lib::board::{Board, BOARD_SIZE};
use crate::game_lib::piece::Piece;
use crate::game_lib::piece::{Color, PieceType};
use crate::game_lib::position::Position;

pub struct Game {
    pub board: Board,
    pub nb_turn: usize
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
    /// use M_Chess::game_lib::game::Game; 
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
        Game {
            board: if custom {Board::empty_init()} else {Board::full_init()},
            nb_turn: 0
        }
    }
    /*
     * Waited string format:
     * - <coo1>-><coo2>
     * - coo1 or coo2: [a-h][1-8]
     */
    fn parse_move_str(move_piece: &str) -> Result<(Position, Position), &'static str>{
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
        let rook_positions = [
            Position::new(king.position.row, 0), // Tour côté dame
            Position::new(king.position.row, 7), // Tour côté roi
        ];

        for rook_position in rook_positions.iter() {
            if (*to_pos == Position::new(king.position.row, king.position.col - 2)
                || *to_pos == Position::new(king.position.row, king.position.col + 2))
                && self.board.can_castle(&king.position, rook_position)
            {
                self.board.perform_castle(&king.position, rook_position);
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
    /// use M_Chess::game_lib::game::Game;
    ///
    /// let mut game = Game::init(false);
    /// game.make_move_algebraic("e2->e4"); // Ok(True)
    /// game.make_move_algebraic("e3->e4"); // Error(_)
    /// ```
    #[inline]
    pub fn make_move_algebraic(&mut self, moves: &str) -> Result<bool, &'static str> {
        let res = Self::parse_move_str(moves);
        if res.is_err() { return Err("parse_move_str: invalid send string: <{move_piece}>"); }

        let (from_pos, to_pos) = res.unwrap();

        // get the piece and if there is not return an error
        let piece: &Piece = {
            if let Some(mut piece) = Piece::get_piece(&from_pos, &self.board) {
                &piece.clone()
            } else {
                return Err("Invalid move: There is not piece here");
            }
        };

        if piece.color != self.board.turn { return Err("Mouvement invalide."); }
        //TODO
        // if self.board.is_king_in_check(turn) => if pion != roi || move protège le roi => false

        // rock situtation
        if piece.piece_type == PieceType::King && self.castle_situation(piece, &to_pos) {
            return Ok(true);
        }
        // if the piece can move + is moved
        if self.board.move_piece(&from_pos, &to_pos) {
            
            //get piece coo in the pieces Vec of the board
            let (x, y): (isize, isize) = self.board.squares[to_pos.row][to_pos.col];
            self.board
                .history
                .push((from_pos, to_pos, piece.piece_type, (x as usize, y as usize)));
            
            
            // if the king is in check due to the move
            if self.board.is_king_in_check(self.board.turn) {
        
                self.undo_move();
                return Err("Le roi est toujours en échec après ce mouvement.");
            
            }
            // change the turn
            else {
                self.board.turn = if self.board.turn == Color::White {
                    Color::Black
                } else {
                    Color::White
                };
            }
            
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
            if self.board.is_pat(self.board.turn) {
                println!("PAT! AUCUN JOUEUR GAGNE.");
                return Ok(false);      
            }

            Ok(true)
        } else {
            Err("Mouvement invalide.")
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
    /// use M_Chess::game_lib::game::Game;
    ///
    /// let mut game = Game::init(false);
    ///
    /// game.get_list_moves("e2".to_string()); // => ["e3", "e4"]
    /// game.get_list_moves("e3".to_string()); // => []
    /// game.get_list_moves("e32".to_string()); // => Err(_)
    /// ```
    pub fn get_list_moves(&self, cell: String) -> Result<Vec<String>, &'static str>{
         
        let mut result: Vec<String> = Vec::<String>::new();

        if cell.chars().count() != 2 {
            return Err("Wrong string format");
        }
        
        // convert into Position
        let position: Position =
            match Position::from_algebraic(&cell) {
                Ok(val) => val,
                Err(_) => return Err("Wrong string format")
            };
        
        // get the piece and if there is no piece just return an empty list
        let piece: &Piece = 
            match Piece::get_piece(&position, &self.board) {
                Some(val) => val,
                None => return Ok(vec![])
            };
        
        if piece.color != self.board.turn {
            return Ok(vec![]);
        }

        let lst_moves: Vec<Position> = piece.valid_moves(&self.board);

        for i in lst_moves.iter() {
            // convert Position -> String
            result.push(i.to_algebraic());
        }

        Ok(result)
    }
    

    fn undo_move(&mut self) {
        //undo until color change => for exeptional cases as castle or hysto empty
        while let Some((from, to, ptype, (x, y))) = self.board.history.pop() {
            if self.board.pieces[x][y].color != self.board.turn {
                self.board.move_piece(&to, &from);
            } else {
                self.board.history.push((from, to, ptype, (x, y)));
                break;
            }
        }
        self.board.turn = if self.board.turn == Color::White {
            Color::Black
        } else {
            Color::White
        };
    }
}
