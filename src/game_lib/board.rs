use crate::game_lib::piece::{Color, Piece, PieceType};
use crate::game_lib::position::Position;
use std::array::from_fn;

pub const BOARD_SIZE: usize = 8;
pub const NONE: usize = 52;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub squares: [[(isize, isize); BOARD_SIZE]; BOARD_SIZE],
    // tableau des pions
    //  -Blanc
    //  -Noir
    pub pieces: [[Piece; 16]; 2],
    pub turn: Color,
    pub history: Vec<(Position, Position, PieceType, (usize, usize))>,
}

impl Board {
    pub fn empty_init() -> Board {
        // init empty board
        let mut squares: [[(isize, isize); BOARD_SIZE]; BOARD_SIZE] =
            [[(-1, -1); BOARD_SIZE]; BOARD_SIZE];

        //pieces [0] => Black //piece [1] =>White
        let mut pieces: [[Piece; 16]; 2] = [
            [
                Piece::new(Color::Black, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Rook, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Rook, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Knight, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Knight, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Bishop, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Bishop, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::Queen, Position::new(NONE, NONE)),
                Piece::new(Color::Black, PieceType::King, Position::new(NONE, NONE)),
            ],
            [
                Piece::new(Color::White, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Pawn, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Rook, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Rook, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Knight, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Knight, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Bishop, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Bishop, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::Queen, Position::new(NONE, NONE)),
                Piece::new(Color::White, PieceType::King, Position::new(NONE, NONE)),
            ],
        ];

        Board {
            squares,
            pieces,
            turn: Color::White,
            history: Vec::new(),
        }
    }

    pub fn full_init() -> Board {
        let mut squares: [[(isize, isize); BOARD_SIZE]; BOARD_SIZE] =
            [[(-1, -1); BOARD_SIZE]; BOARD_SIZE];

        //pieces [0] => Black //piece [1] =>White
        let mut pieces: [[Piece; 16]; 2] = [
            [
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
            ],
            [
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
                Piece::new(Color::White, PieceType::Knight, Position::new(7, 1)),
                Piece::new(Color::White, PieceType::Knight, Position::new(7, 6)),
                Piece::new(Color::White, PieceType::Bishop, Position::new(7, 2)),
                Piece::new(Color::White, PieceType::Bishop, Position::new(7, 5)),
                Piece::new(Color::White, PieceType::Queen, Position::new(7, 3)),
                Piece::new(Color::White, PieceType::King, Position::new(7, 4)),
            ],
        ];

        // Put each piece in its cell on the board

        // Init Pawn
        for i in 0..BOARD_SIZE {
            squares[1][i] = (0, i as isize);
            squares[6][i] = (1, i as isize);
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

    // display in the terminal the board
    pub fn print_board(&mut self) {
        println!("  a b c d e f g h");
        for row in 0..BOARD_SIZE {
            print!("{} ", 8 - row);
            for col in 0..BOARD_SIZE {
                let position: Position = Position::new(row, col);
                let piece_option: Option<&Piece> = Piece::get_piece(&position, self);
                let piece: Option<&Piece> = piece_option;

                // if there is no piece
                if piece.is_none() {
                    print!(". ");
                    continue;
                }

                let piece: &Piece = piece.unwrap();
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
            }
            println!("{}", 8 - row);
        }
        println!("  a b c d e f g h");
    }

    // In the list of one type piece find a piece which is unused.
    fn find_unused_piece(
        &mut self,
        icolor: usize,
        ipiece: usize,
        pos: Position,
    ) -> Result<usize, &'static str> {
        let min: usize;
        let max: usize;
        match ipiece {
            0 => {
                min = 0;
                max = 7;
            }
            8 => {
                min = 8;
                max = 9;
            }
            10 => {
                min = 10;
                max = 11;
            }
            12 => {
                min = 12;
                max = 13;
            }
            14 => {
                min = 14;
                max = 14;
            }
            15 => {
                min = 15;
                max = 15;
            }
            _ => return Err("Wrong string format"),
        }

        for i in min..=max {
            if self.pieces[icolor][i].position.row == NONE
                || self.pieces[icolor][i].position.col == NONE
            {
                return Ok(i);
            }
        }

        Err("max of this piece")
    }

    /// Take a [`String`] in the grid and add a piece if the cell is empty.\
    /// String format: {b/w}{k/q/r/b/n/p}[a-h][0-8]
    ///
    ///
    /// # Example
    /// ```no_run
    /// use M_Chess::game_lib::game::Game;
    ///
    /// let mut game = Game::init(true); // empty board
    ///
    /// // add black Knight in e3
    /// game.board.add_piece("bne3"); // => Ok(true)
    /// // cannot add in a full cell
    /// game.board.add_piece("bne3"); // => Ok(false)
    /// game.board.add_piece("sfwf"); // => Err("Wrong string format")
    /// ```
    ///
    /// # Complexity
    /// `O(n)`
    pub fn add_piece(&mut self, piece_str: &str) -> Result<bool, &'static str> {
        if piece_str.chars().count() != 4 {
            return Err("Wrong string format");
        }

        // get color index for board.Pieces
        let icolor: usize;
        match &piece_str[..=0] {
            "b" => icolor = 0,
            "w" => icolor = 1,
            _ => return Err("Wrong string format"),
        }

        // get piece index for board.Pieces
        let ipiece: usize;
        match &piece_str[1..=1] {
            "p" => ipiece = 0,
            "r" => ipiece = 8,
            "n" => ipiece = 10,
            "b" => ipiece = 12,
            "q" => ipiece = 14,
            "k" => ipiece = 15,
            _ => return Err("Wrong string format"),
        }

        // get the position
        let board_pos: Position = match Position::from_algebraic(&piece_str[2..=3]) {
            Ok(val) => val,
            Err(err) => return Err("Wrong string format"),
        };

        // if there already a piece
        if self.squares[board_pos.row][board_pos.col] != (-1, -1) {
            return Ok(false);
        }

        let i: usize = match self.find_unused_piece(icolor, ipiece, board_pos) {
            Ok(val) => val,
            Err(err) => return Ok(false),
        };

        // add the piece on the board and link squares and pieces togather
        self.squares[board_pos.row][board_pos.col] = (icolor as isize, i as isize);
        self.pieces[icolor][i].position = board_pos;

        Ok(true)
    }

    /// Take a [`String`] in the grid and remove a piece if the cell contains it.
    /// String format: [a-h][0-8]
    ///
    /// # Example
    /// ```no_run
    /// use M_Chess::game_lib::game::Game;
    ///
    /// let mut game = Game::init(false); // classic board
    ///
    /// game.board.remove_piece("e2"); // => Ok(true)
    /// // cannot remove if the cell is empty
    /// game.board.remove_piece("e2"); // => Ok(false)
    /// game.board.remove_piece("32"); // => Err(_)
    /// ```
    ///
    /// # Complexity
    /// `O(1)`
    pub fn remove_piece(&mut self, str_position: &str) -> Result<bool, &'static str> {
        if str_position.chars().count() != 2 {
            return Err("Wrong string format");
        }

        // get the position on the board
        let board_pos: Position = match Position::from_algebraic(str_position) {
            Ok(val) => val,
            Err(_) => return Err("Wrong string format"),
        };

        let (icolor, i) = self.squares[board_pos.row][board_pos.col];
        if icolor == -1 || i == -1 {
            return Ok(false);
        }

        let (icolor, i) = (icolor as usize, i as usize);
        self.pieces[icolor][i].position.row = NONE;
        self.pieces[icolor][i].position.col = NONE;
        self.squares[board_pos.row][board_pos.col] = (-1, -1);

        Ok(true)
    }

    // move a piece to a position
    // must be to position in args because:
    // we dont know if there is a piece on the "from" position
    pub fn move_piece(&mut self, from: &Position, to: &Position) -> bool {
        let piece = {
            // Scope temporaire pour l'emprunt mutable de `self`
            if let Some(piece) = Piece::get_piece(from, self) {
                piece
            } else {
                return false;
            }
        };

        if piece.piece_type == PieceType::Pawn {
            if self.is_valid_move(piece, to) {
                if from == to {
                    // call upgrade
                    let (i, j): (isize, isize) = self.squares[from.row][from.col];

                    //self.pieces[i as usize][j as usize].piece_type = get_type()=>input de l'api??;
                } else {
                    self.move_(from, to);
                }
                true
            } else {
                false
            }
        } else if self.is_valid_move(piece, to) {
            self.move_(from, to);
            true
        } else {
            false
        }
    }

    fn move_(&mut self, from: &Position, to: &Position) {
        let (i, j): (isize, isize) = self.squares[from.row][from.col];

        self.pieces[i as usize][j as usize].has_moved = true;
        self.pieces[i as usize][j as usize].position.row = to.row;
        self.pieces[i as usize][j as usize].position.col = to.col;

        // update the case where the piece is now
        self.squares[to.row][to.col] = (i, j);
        // remove the old case where the piece moved
        self.squares[from.row][from.col] = (-1, -1);
    }

    /// Return a matrix of a board. Each cell contains a [`String`].
    /// String format:
    /// - empty: ".."
    /// - piece: {b/w}{p/n/b/r/q/k}
    ///
    /// # Example
    /// ```no_run
    /// use M_Chess::game_lib::game::Game;
    /// use M_Chess::game_lib::board::BOARD_SIZE;
    ///
    /// let mut game = Game::init(false);
    ///
    /// let str_board: [[String; BOARD_SIZE]; BOARD_SIZE] = game.board.get();
    ///
    /// // |"br"|"bk"|"bb"|"bq"|"bk"|"bb"|"bk"|"br"|
    /// // |"bp"|"bp"|"bp"|"bp"|"bp"|"bp"|"bp"|"bp"|
    /// // |".."|".."|".."|".."|".."|".."|".."|".."|
    /// // |".."|".."|".."|".."|".."|".."|".."|".."|
    /// // |".."|".."|".."|".."|".."|".."|".."|".."|
    /// // |".."|".."|".."|".."|".."|".."|".."|".."|
    /// // |"wp"|"wp"|"wp"|"wp"|"wp"|"wp"|"wp"|"wp"|
    /// // |"wr"|"wk"|"wb"|"wq"|"wk"|"wb"|"wk"|"wr"|
    ///
    ///
    /// ```
    pub fn get(&self) -> [[String; BOARD_SIZE]; BOARD_SIZE] {
        let mut str_board: [[String; BOARD_SIZE]; BOARD_SIZE] =
            std::array::from_fn(|_| std::array::from_fn(|_| String::from("..")));

        // iter on each cell of the board
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let (icolor, ipiece) = self.squares[i][j];
                if (icolor, ipiece) == (-1, -1) {
                    continue;
                }

                let (icolor, ipiece) = (icolor as usize, ipiece as usize);
                let piece: &Piece = &self.pieces[icolor][ipiece];

                str_board[i][j] = piece.to_string();
            }
        }

        str_board
    }

    //color of the king to check
    pub fn is_attacked(&self, position: &Position, color: Color) -> bool {
        let ennemy_color = if color == Color::Black {
            Color::White
        } else {
            Color::Black
        };
        for i in 0..15 {
            let ennemy_moves = self.pieces[ennemy_color as usize][i].valid_moves(self);
            if ennemy_moves.contains(position) {
                return true;
            }
        }
        false
    }

    // look at if piece can do the move
    //FIXME
    //don't take moves that put the king in mate
    pub fn is_valid_move(&self, piece: &Piece, to: &Position) -> bool {
        // get all the valid moves on this postion
        let valid_moves: Vec<Position> = piece.valid_moves(self);

        // if the move to do is in the valid move
        valid_moves.contains(to)
    }

    pub fn is_within_bounds(&self, position: &Position) -> bool {
        position.row < BOARD_SIZE && position.col < BOARD_SIZE
    }

    //color of the king to check
    pub fn is_king_in_check(&self, color: Color) -> bool {
        // get the king
        let king: &Piece = &self.pieces[color as usize][15];

        self.is_attacked(&king.position, king.color)
    }

    // FIXME
    // Check if one color is in checkmate => no moves available and attacked
    //color of the king of the team to play
    pub fn is_checkmate(&self, color: Color) -> bool {
        if !self.is_king_in_check(color) {
            return false;
        }
        //Sous fonction pour check si les moves du roi sont safes
        //pour chaques positions de ses valid moves si
        //      dans les valids moves d'au moins un pions adverse
        //=> à utiliser pour le path <=> is_checkmate sans le check ('_')
        //TODO remove la suite + faire les comment au dessus

        let king = self.pieces[color as usize][15];
        let set_of_move = king.valid_moves(self);

        // if the king can move
        set_of_move.is_empty()
    }

    pub fn is_pat(&self, color: Color) -> bool {
        for i in 0..16 {
            let piece_moves = self.pieces[color as usize][i].valid_moves(self);
            if !piece_moves.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn can_castle(&self, king_position: &Position, rook_position: &Position) -> bool {
        let king: &Piece = if let Some(king) = Piece::get_piece(king_position, self) {
            king
        } else {
            panic!("Where is the king, not update board")
        };
        // if the rook is at its default position we continue else we can not castle
        let rook: &Piece = if let Some(rook) = Piece::get_piece(rook_position, self) {
            rook
        } else {
            return false;
        };

        // Vérifier que le roi et la tour n'ont pas bougé
        if king.has_moved || rook.has_moved {
            return false;
        }

        // Vérifier qu'il n'y a pas de pièces entre le roi et la tour
        let (min_col, max_col) = if king.position.col < rook.position.col {
            (king.position.col + 1, rook.position.col)
        } else {
            (rook.position.col + 1, king.position.col)
        };

        // for each cell between the king and the rook
        for col in min_col..max_col {
            let pos = Position::new(king.position.row, col);

            // we check if there is piece on it
            if self.squares[king.position.row][col] != (-1, -1) {
                return false;
            }
            // and if not, if the cell is attacked
            else if self.is_attacked(&pos, king.color) {
                return false;
            }
        }

        // finally we check if the rook is attacked
        if self.is_attacked(&rook.position, rook.color) {
            return false;
        }

        true
    }

    pub fn perform_castle(&mut self, king_position: &Position, rook_position: &Position) {
        let new_king_position: Position = if rook_position.col < king_position.col {
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

        //get pieces
        let (x_king, y_king): (isize, isize) = self.squares[king_position.row][king_position.col];
        let mut king = self.pieces[x_king as usize][y_king as usize];
        let (x_rook, y_rook): (isize, isize) = self.squares[rook_position.row][rook_position.col];
        let mut rook = self.pieces[x_rook as usize][y_rook as usize];

        //update history
        self.history.push((
            king.position,
            new_king_position,
            PieceType::King,
            (x_king as usize, y_king as usize), // EXCEPTIONNELLE SITUATION
                                                //Option<Piece> de la case
        ));

        self.history.push((
            rook.position,
            new_rook_position,
            PieceType::Rook,
            (x_rook as usize, y_rook as usize), // EXCEPTIONNELLE SITUATION
                                                //Option<Piece> de la case
        ));
        // Déplacer le roi et la tour
        king.has_moved = true;
        king.position.row = new_king_position.row;
        king.position.col = new_king_position.col;

        // update the case where the king is now
        self.squares[new_king_position.row][new_king_position.col] = (x_king, y_king);
        // remove the old case where the king moved
        self.squares[king_position.row][king_position.col] = (-1, -1);

        // update the case where the rook is now
        self.squares[new_rook_position.row][new_rook_position.col] = (x_rook, y_rook);
        // remove the old case where the rook moved
        self.squares[rook_position.row][rook_position.col] = (-1, -1);

        // change the turn
        self.turn = if self.turn == Color::White {
            Color::Black
        } else {
            Color::White
        };
    }

    pub fn is_game_over(&self) -> bool {
        self.is_checkmate(Color::White)
            || self.is_checkmate(Color::Black)
            || self.is_pat(Color::White)
            || self.is_pat(Color::Black)
    }
}
