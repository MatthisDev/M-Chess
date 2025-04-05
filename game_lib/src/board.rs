use crate::piece::{Color, Piece, PieceType};
use crate::position::Position;
use std::array::from_fn;

pub const BOARD_SIZE: usize = 8;
pub const NONE: usize = 52;
pub const EMPTY_CELL: (isize, isize) = (NONE as isize, NONE as isize);
pub const EMPTY_POS: Position = Position {
    row: NONE,
    col: NONE,
};

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: [[(isize, isize); BOARD_SIZE]; BOARD_SIZE],
    // tableau des pions
    //  -Blanc
    //  -Noir
    pub pieces: [[Piece; 16]; 2],
    pub turn: Color,
    pub history: Vec<(Position, Position, PieceType, (usize, usize), bool)>,
}

impl Board {
    pub fn empty_init() -> Board {
        // init empty board
        let mut squares: [[(isize, isize); BOARD_SIZE]; BOARD_SIZE] =
            [[EMPTY_CELL; BOARD_SIZE]; BOARD_SIZE];

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
            [[EMPTY_CELL; BOARD_SIZE]; BOARD_SIZE];

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
    pub fn print_board(&self) {
        println!("  a b c d e f g h");
        for row in 0..BOARD_SIZE {
            print!("{} ", row);
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
            println!("{}", row);
        }
        println!("  a b c d e f g h");
    }

    // In the list of one type piece find a piece which is unused.
    fn find_unused_piece(&mut self, icolor: usize, ipiece: usize) -> Result<usize, &'static str> {
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
    /// use m_chess::game_lib::game::Game;
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
        let icolor: usize = match &piece_str[..=0] {
            "b" => 0,
            "w" => 1,
            _ => return Err("Wrong string format"),
        };

        // get piece index for board.Pieces
        let ipiece: usize = match &piece_str[1..=1] {
            "p" => 0,
            "r" => 8,
            "n" => 10,
            "b" => 12,
            "q" => 14,
            "k" => 15,
            _ => return Err("Wrong string format"),
        };

        // get the position
        let board_pos: Position = match Position::from_algebraic(&piece_str[2..=3]) {
            Ok(val) => val,
            Err(err) => return Err("Wrong string format"),
        };

        // if there already a piece
        if self.squares[board_pos.row][board_pos.col] != EMPTY_CELL {
            return Ok(false);
        }

        let i: usize = match self.find_unused_piece(icolor, ipiece) {
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
    /// use m_chess::game_lib::game::Game;
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
        if (icolor, i) == EMPTY_CELL {
            return Ok(false);
        }

        let (icolor, i) = (icolor as usize, i as usize);
        self.pieces[icolor][i].position.row = NONE;
        self.pieces[icolor][i].position.col = NONE;
        self.squares[board_pos.row][board_pos.col] = EMPTY_CELL;

        Ok(true)
    }

    // move a piece to a position
    // must be to position in args because:
    // we dont know if there is a piece on the "from" position
    pub fn move_piece(&mut self, from: &Position, to: &Position) -> bool {
        // check if the move is valid (according to chess rules and piece range)
        // and try to update to its new position
        self.is_valid_move(from, to) && self.update_position(from, to)
    }

    // update without checking chess rules and replacing pieces
    fn update_position(&mut self, piece_pos: &Position, to: &Position) -> bool {
        let (x_piece, y_piece): (usize, usize) = match self.squares[piece_pos.row][piece_pos.col] {
            (x, y) if (x, y) == EMPTY_CELL => return false,
            (x, y) => (x as usize, y as usize),
        };

        let (x_piece_to, y_piece_to): (usize, usize) = match self.squares[to.row][to.col] {
            (x, y) if (x, y) == EMPTY_CELL => (NONE, NONE),
            (x, y) => (x as usize, y as usize),
        };

        // if there is something on the cell we eat a piece
        if (NONE, NONE) != (x_piece_to, y_piece_to) {
            // push the info in the history
            self.history.push((
                *to,
                EMPTY_POS,
                self.pieces[x_piece_to][y_piece_to].piece_type,
                (x_piece_to, y_piece_to),
                true,
            ));
            self.pieces[x_piece_to][y_piece_to].position = EMPTY_POS;
        }

        // update history
        self.history.push((
            *piece_pos,
            *to,
            self.pieces[x_piece][y_piece].piece_type,
            (x_piece, y_piece), // EXCEPTIONNELLE SITUATION
            false,              //Option<Piece> de la case
        ));

        // link the piece to its new coo
        match Piece::get_piece_mut(piece_pos, self) {
            Some(piece) => {
                piece.position.row = to.row;
                piece.position.col = to.col;

                if piece.piece_type == PieceType::King {
                    piece.has_moved = true;
                }
            }
            None => return false,
        }

        // unlink the piece on the cell to
        match Piece::get_piece_mut(to, self) {
            Some(piece) => piece.position = EMPTY_POS,
            None => (),
        }

        // update board's cells
        self.squares[to.row][to.col] = self.squares[piece_pos.row][piece_pos.col];
        self.squares[piece_pos.row][piece_pos.col] = EMPTY_CELL;

        true
    }

    /// Return a matrix of a board. Each cell contains a [`String`].
    /// String format:
    /// - empty: ".."
    /// - piece: {b/w}{p/n/b/r/q/k}
    ///
    /// # Example
    /// ```no_run
    /// use m_chess::game_lib::game::Game;
    /// use m_chess::game_lib::board::BOARD_SIZE;
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
                if (icolor, ipiece) == EMPTY_CELL {
                    continue;
                }

                let (icolor, ipiece) = (icolor as usize, ipiece as usize);
                let piece: &Piece = &self.pieces[icolor][ipiece];

                str_board[i][j] = piece.the_string();
            }
        }

        str_board
    }

    //color of the king to check
    pub fn is_attacked(&self, position: &Position, color: Color) -> bool {
        let ennemy_color = color.opposite();

        for i in 0..15 {
            // ignore eaten pieces

            if self.pieces[ennemy_color as usize][i].position != EMPTY_POS {
                continue;
            } else if self.pieces[ennemy_color as usize][i].is_valid_move(self, position) {
                return true;
            }
        }

        // handle king case
        self.pieces[ennemy_color as usize][15].is_in_king_hitbox(position)
    }

    // look at if piece can do the move
    //FIXME
    //don't take moves that put the king in mate
    pub fn is_valid_move(&mut self, piece_pos: &Position, to: &Position) -> bool {
        // Algo:
        // - Si le move est possible. On le simule.
        let piece: &Piece = match Piece::get_piece(piece_pos, self) {
            Some(piece) => piece,
            None => return false,
        };

        if !piece.is_valid_move(self, to) {
            return false;
        }

        // Si le move est simulable == n'implique pas un echec de notre propre roi.
        !self.put_in_check_simulation(piece_pos, to)
    }

    // Function for simulation
    fn modif_simulation(
        &mut self,
        to: &Position,
        piece_pos: &Position,
        (icolor_to, ipiece_to): (&mut isize, &mut isize),
        (icolor, ipiece): (&mut isize, &mut isize),
    ) -> bool {
        // if there is a piece on the 'to_cell' then it's remove it
        match Piece::get_piece_mut(to, self) {
            Some(to_piece) => {
                to_piece.position.row = NONE;
                to_piece.position.col = NONE;
            }
            None => (),
        }

        // change the cell of the piece
        match Piece::get_piece_mut(piece_pos, self) {
            Some(piece_mut) => {
                piece_mut.position.row = to.row;
                piece_mut.position.row = to.col;
            }
            None => return false,
        }

        // save the piece on both cells
        (*icolor_to, *ipiece_to) = self.squares[to.row][to.col];
        (*icolor, *ipiece) = self.squares[piece_pos.row][piece_pos.col];

        // move
        self.squares[to.row][to.col] = self.squares[piece_pos.row][piece_pos.col];
        self.squares[piece_pos.row][piece_pos.col] = EMPTY_CELL;

        true
    }

    fn get_back_simulation(
        &mut self,
        to: &Position,
        piece_pos: &Position,
        (icolor_to, ipiece_to): (&mut isize, &mut isize),
        (icolor, ipiece): (&mut isize, &mut isize),
    ) -> bool {
        match Piece::get_piece_mut(piece_pos, self) {
            Some(piece) => {
                piece.position.row = to.row;
                piece.position.col = to.col;
            }
            None => return false,
        }

        self.squares[to.row][to.col] = (*icolor, *ipiece);
        self.squares[piece_pos.row][piece_pos.col] = (*icolor_to, *ipiece_to);

        // relink the piece if we ate one
        match Piece::get_piece_mut(piece_pos, self) {
            Some(piece) => {
                piece.position.row = to.row;
                piece.position.col = to.col;
            }
            None => (),
        }

        true
    }

    // simulate a move and test if is not put in check its king
    fn put_in_check_simulation(&mut self, piece_pos: &Position, to: &Position) -> bool {
        let (mut icolor_to, mut ipiece_to): (isize, isize) = (0, 0);
        let (mut icolor, mut ipiece): (isize, isize) = (0, 0);

        let b = self.modif_simulation(
            to,
            piece_pos,
            (&mut icolor_to, &mut ipiece_to),
            (&mut icolor, &mut ipiece),
        );

        if !b {
            panic!("put_in_check_simulation: cannot modif the board for simulate");
        }

        let piece: &Piece = match Piece::get_piece(to, self) {
            Some(piece) => piece,
            None => panic!("put_in_check_simulation: cannot get the piece in the cell"),
        };

        // verification about the validity
        let is_check: bool = self.is_king_in_check(piece.color);

        let b = self.get_back_simulation(
            piece_pos,
            to,
            (&mut icolor_to, &mut ipiece_to),
            (&mut icolor, &mut ipiece),
        );

        if !b {
            panic!("put_in_check_simulation: cannot get back the board for simulate");
        }

        is_check
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
        let king: &Piece = &self.pieces[color as usize][15];
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
        println!("king: {:?}", king_position);
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

            if self.squares[king.position.row][col] != EMPTY_CELL
                || self.is_attacked(&pos, king.color)
            {
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
        // get new position
        let new_king_position: Position = if rook_position.col < king_position.col {
            // Grand roque
            Position::new(king_position.row, king_position.col - 2)
        } else {
            // Petit roque
            Position::new(king_position.row, king_position.col + 2)
        };
        let new_rook_position: Position = if rook_position.col < king_position.col {
            // Grand roque
            Position::new(rook_position.row, rook_position.col + 3)
        } else {
            // Petit roque
            Position::new(rook_position.row, rook_position.col - 2)
        };

        // move king and rook
        let mut is_move = self.update_position(king_position, &new_king_position);
        is_move = is_move && self.update_position(rook_position, &new_rook_position);
    }

    pub fn is_game_over(&self) -> bool {
        self.is_checkmate(Color::White)
            || self.is_checkmate(Color::Black)
            || self.is_pat(Color::White)
            || self.is_pat(Color::Black)
    }

    pub fn undo_move(&mut self) {
        // Safeguard: Ensure history is not empty

        // Undo moves until the turn changes
        while let Some((from, to, ptype, (x, y), eaten)) = self.history.pop() {
            // Restore the piece's position
            self.move_piece(&to, &from);
            //println!("Undo move from {:?} to {:?}", from, to);
            // Handle eaten pieces
            if eaten {
                println!("Restoring eaten piece at {:?}", from);
                self.squares[from.row][from.col] = (x as isize, y as isize);
                self.pieces[x][y].position = from; // Restore the eaten piece's position

                // Undo one more move to restore the piece that performed the capture
                continue;
            }

            // Handle pawn promotion
            if ptype == PieceType::Pawn && (to.row == 0 || to.row == BOARD_SIZE - 1) {
                println!("Undoing pawn promotion at {:?}", to);
                self.pieces[x][y].piece_type = PieceType::Pawn; // Restore the pawn
            }

            // Handle castling
            if ptype == PieceType::King && (to.col as isize - from.col as isize).abs() == 2 {
                println!("Undoing castling");
                let rook_from = if to.col > from.col {
                    Position::new(to.row, BOARD_SIZE - 1) // Kingside rook
                } else {
                    Position::new(to.row, 0) // Queenside rook
                };
                let rook_to = if to.col > from.col {
                    Position::new(to.row, to.col - 1) // Kingside rook's new position
                } else {
                    Position::new(to.row, to.col + 1) // Queenside rook's new position
                };
                self.move_piece(&rook_to, &rook_from); // Restore the rook's position
            }

            // Check if the turn has changed
            if self.turn == self.pieces[x][y].color.opposite() {
                break;
            }
        }

        // Flip the turn
        self.turn = self.turn.opposite();
    }

    pub fn is_pawn_isolated(&self, position: &Position) -> bool {
        let col = position.col;
        let row = position.row;

        // Check adjacent files for friendly pawns
        for offset in [-1, 1] {
            if let Ok(adj_col) = (col as isize + offset).try_into() {
                for piece in self.pieces[self.turn as usize].iter() {
                    if piece.piece_type == PieceType::Pawn && piece.position.col == adj_col {
                        return false; // Found a friendly pawn on an adjacent file
                    }
                }
            }
        }

        true // No friendly pawns found on adjacent files
    }

    pub fn get_all_valid_moves(&self, color: Color) -> Vec<(Position, Position)> {
        let mut moves = Vec::new();

        // Iterate over all pieces of the given color
        for piece in self.pieces[color as usize].iter() {
            if piece.position.row != NONE && piece.position.col != NONE {
                for mv in piece.valid_moves(self) {
                    moves.push((piece.position, mv));
                }
            }
        }

        moves
    }

    pub fn is_pawn_doubled(&self, position: &Position) -> bool {
        let col = position.col;

        // Check for other friendly pawns on the same file
        for piece in self.pieces[self.turn as usize].iter() {
            if piece.piece_type == PieceType::Pawn
                && piece.position.col == col
                && piece.position != *position
            {
                return true; // Found another friendly pawn on the same file
            }
        }

        false // No other friendly pawns found on the same file
    }

    pub fn is_open_file(&self, col: usize) -> bool {
        for row in 0..BOARD_SIZE {
            let (x, y) = self.squares[row][col];
            if x != NONE as isize
                && self.pieces[x as usize][y as usize].piece_type == PieceType::Pawn
            {
                return false; // Found a pawn on the file
            }
        }

        true // No pawns found on the file
    }

    pub fn is_king_exposed(&self, position: &Position) -> bool {
        let king_row = position.row;
        let king_col = position.col;

        // Check surrounding cells for friendly pawns
        for row_offset in -1isize..=1 {
            for col_offset in -1isize..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue; // Skip the king's position
                }

                // Calculate adjacent row and column
                let adj_row = king_row as isize + row_offset;
                let adj_col = king_col as isize + col_offset;

                // Ensure the indices are within bounds
                if adj_row >= 0
                    && adj_row < BOARD_SIZE as isize
                    && adj_col >= 0
                    && adj_col < BOARD_SIZE as isize
                {
                    let adj_row = adj_row as usize;
                    let adj_col = adj_col as usize;

                    // Access the square and check for friendly pawns
                    let (x, y) = self.squares[adj_row][adj_col];
                    if x != NONE as isize
                        && self.pieces[x as usize][y as usize].piece_type == PieceType::Pawn
                        && self.pieces[x as usize][y as usize].color == self.turn
                    {
                        return false; // Found a friendly pawn protecting the king
                    }
                }
            }
        }

        true // No friendly pawns found protecting the king
    }

    pub fn is_castling_move(&self, from: &Position, to: &Position) -> bool {
        // Check if the piece is a king
        if let Some(piece) = Piece::get_piece(from, self) {
            if piece.piece_type == PieceType::King {
                // Check if the move is a castling move (king moves two squares horizontally)
                return (to.col as isize - from.col as isize).abs() == 2;
            }
        }
        false
    }

    pub fn is_capture(&self, to: &Position) -> bool {
        if let Some(piece) = Piece::get_piece(to, self) {
            // Check if the piece at the destination belongs to the opponent
            return piece.color != self.turn;
        }
        false
    }
    pub fn is_safe_move(&self, from: &Position, to: &Position) -> bool {
        // Simulate the move
        let mut temp_board = self.clone();
        temp_board.move_piece(from, to);

        // Check if the destination square is attacked after the move
        !temp_board.is_attacked(to, self.turn.opposite())
    }

    pub fn is_pawn_double_move(&self, from: &Position, to: &Position) -> bool {
        if let Some(piece) = Piece::get_piece(from, self) {
            if piece.piece_type == PieceType::Pawn {
                // Check if the pawn is moving two squares forward from its starting position
                let direction = if piece.color == Color::White { -1 } else { 1 };
                let start_row = if piece.color == Color::White { 6 } else { 1 };
                return from.row == start_row
                    && (to.row as isize - from.row as isize) == 2 * direction;
            }
        }
        false
    }
}
