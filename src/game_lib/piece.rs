use crate::game_lib::board::{Board, BOARD_SIZE, NONE, EMPTY_CELL};
use crate::game_lib::position::Position;

use super::board::NONE;
use super::position;

// Color enum for teams
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}
impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn get_value(&self) -> i32 {
        match self {
            PieceType::King => 1000,
            PieceType::Queen => 9,
            PieceType::Rook => 5,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Pawn => 1,
        }
    }
}

//class Piece
#[derive(Debug, Clone)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool, //for special moves
    pub position: Position,
}

impl Piece {
    //create a piece
    pub fn new(color: Color, piece_type: PieceType, position: Position) -> Self {
        Piece {
            color,
            piece_type,
            has_moved: false,
            position,
        }
    }

    pub fn the_string(&self) -> String {
        let mut str_piece: String = String::new();

        if self.color == Color::Black {
            str_piece.push('b');
        } else {
            str_piece.push('w');
        }

        match self.piece_type {
            PieceType::Pawn => str_piece.push('p'),
            PieceType::Knight => str_piece.push('k'),
            PieceType::Bishop => str_piece.push('b'),
            PieceType::Rook => str_piece.push('r'),
            PieceType::Queen => str_piece.push('q'),
            _ => str_piece.push('k'),
        }

        str_piece
    }

    // get the piece from a specific position
    pub fn get_piece<'a>(position: &Position, board: &'a Board) -> Option<&'a Piece> {
        if position.col == NONE || position.row == NONE {
            println!("Error: position is out of bounds");
            return None;
        }

        let (i, j): (isize, isize) = board.squares[position.row][position.col];

        // when there is no piece

        if (i, j) == EMPTY_CELL{
            return None;
        }

        let (i, j): (usize, usize) = (i as usize, j as usize);

        Some(&board.pieces[i][j])
    }

    pub fn get_piece_mut<'a>(position: &Position, board: &'a mut Board) -> Option<&'a mut Piece> {
        let (i, j): (isize, isize) = board.squares[position.row][position.col];

        // when there is no piece
        if (i, j) == EMPTY_CELL {
            return None;
        }

        let (i, j): (usize, usize) = (i as usize, j as usize);

        Some(&mut board.pieces[i][j])
    }
    
    pub fn is_valid_move(&self, board: &Board, to_pos: &Position) -> bool {
        match self.piece_type {
            PieceType::Pawn => self.is_valid_move_pawn(board, to_pos),
            PieceType::Knight => self.is_valid_move_knight(board, to_pos),
            PieceType::Bishop => self.is_valid_move_bishop(board, to_pos),
            PieceType::Rook => self.is_valid_move_rook(board, to_pos),
            PieceType::Queen => self.is_valid_move_queen(board, to_pos),
            PieceType::King => self.is_valid_move_king(board, to_pos)
        }

    }
   
    // Get list of all move possible for a specific piece
    // The validity of the mo
    pub fn valid_moves(&self, board: &Board) -> Vec<Position> {
        match self.piece_type {
            PieceType::Pawn => self.valid_moves_pawn(board),
            PieceType::Knight => self.valid_moves_knight(board),
            PieceType::Bishop => self.valid_moves_bishop(board),
            PieceType::Rook => self.valid_moves_rook(board),
            PieceType::Queen => self.valid_moves_queen(board),
            PieceType::King => self.valid_moves_king(board),
        }
    }

    //Pawn-----------------------------------------------------------------
    fn is_valid_move_pawn(&self, board: &Board, to_pos: &Position) -> bool{
        let direction: i32 = if self.color == Color::White { -1 } else { 1 };

        let forward: Position = Position::new(
            (self.position.row as i32 + direction) as usize,
            self.position.col
        );

        if board.is_within_bounds(&forward) {
            if board.squares[forward.row][forward.col] == EMPTY_CELL {
                if forward == *to_pos { 
                    return true;
                }

                //Check Double move forward if never moved
                //If the move +1 or -1 is not possible then the move + 2
                if (self.color == Color::White && self.position.row == 6)
                    || (self.color == Color::Black && self.position.row == 1)
                {
                    let double_forward: Position = Position::new(
                        (self.position.row as i32 + 2 * direction) as usize,
                        self.position.col,
                    );

                    // If there is nothing on the cell the move is possible.
                    // (no need to check the out of board)
                    if board.squares[double_forward.row][double_forward.col] == EMPTY_CELL 
                       && forward == *to_pos {
                        return true;
                    }
                }
            }
        }


        for col_offset in &[-1, 1] {
            let capture: Position = Position::new(
                (self.position.row as i32 + direction) as usize,
                (self.position.col as i32 + col_offset) as usize,
            );

            if capture != *to_pos { return false; }

            if board.is_within_bounds(&capture)
                && board.squares[capture.row][capture.col] != EMPTY_CELL {
                // get the piece if there is
                let piece: &Piece =
                    match Piece::get_piece(&capture, board) {
                        Some(piece) => piece,
                        None => continue
                    };
        
                if piece.color != self.color {
                    return true;
                }
            }
        }

        //prise en passant
        if let Some((from, to, ptype, _, t)) = board.history.last() {
            if !t && *ptype == PieceType::Pawn {

                if (self.color == Color::White
                    && self.position.row == 3
                    && from.row == 1
                    && to.row == 3)
                    || (self.color == Color::Black
                        && self.position.row == 5
                        && from.row == 6
                        && to.row == 4)
                {
                    let pos: Position = Position::new(to.row - 1, to.col);
                    
                    if pos == *to_pos {
                        return true;
                    }

                }
            }
        }

        false
    }

    fn valid_moves_pawn(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        let direction: i32 = if self.color == Color::White { -1 } else { 1 };

        //Check Simple move forward
        let forward: Position = Position::new(
            (self.position.row as i32 + direction) as usize,
            self.position.col,
        );

        // TODO: It's here where we upgrade the PAWN =>
        // If the move is out_of_bound - 1 => UPGRADE
        // if we are out of the board and there is nothing on the cell
        if board.is_within_bounds(&forward) {
            if board.squares[forward.row][forward.col] == EMPTY_CELL {
                moves.push(forward);

                //Check Double move forward if never moved
                //If the move +1 or -1 is not possible then the move + 2
                if (self.color == Color::White && self.position.row == 6)
                    || (self.color == Color::Black && self.position.row == 1)
                {
                    let double_forward: Position = Position::new(
                        (self.position.row as i32 + 2 * direction) as usize,
                        self.position.col,
                    );

                    // If there is nothing on the cell the move is possible.
                    // (no need to check the out of board)
                    if board.squares[double_forward.row][double_forward.col] == EMPTY_CELL {
                        moves.push(double_forward);
                    }
                }
            }
        }
        // TODO UPGRADE HERE.

        // Check Diagonal capture
        for col_offset in &[-1, 1] {
            let capture: Position = Position::new(
                (self.position.row as i32 + direction) as usize,
                (self.position.col as i32 + col_offset) as usize,
            );

            if board.is_within_bounds(&capture)
                && board.squares[capture.row][capture.col] != EMPTY_CELL {
                // get the piece if there is
                let piece: &Piece = if let Some(piece) = Piece::get_piece(&capture, board) {
                    piece
                } else {
                    continue;
                };

                if piece.color != self.color {
                    moves.push(capture)
                }
            }
        }

        //prise en passant
        if let Some((from, to, ptype, _, t)) = board.history.last() {
            if !t && *ptype == PieceType::Pawn {
                //the pawn is supposed to be of the oposite color assuming that
                //only one pawn can be move by turn for one color
                //this is the 2 speed move of the pawn so we assume there is no piece as obstacle
                if (self.color == Color::White
                    && self.position.row == 3
                    && from.row == 1
                    && to.row == 3)
                    || (self.color == Color::Black
                        && self.position.row == 5
                        && from.row == 6
                        && to.row == 4)
                {
                    moves.push(Position::new(to.row - 1, to.col));
                }
            }
        }

        moves
    }
    //---------------------------------------------------------------------

    //Knight---------------------------------------------------------------
    fn is_valid_move_knight(&self, board: &Board, to_pos: &Position) -> bool {
        let offsets: [(i32, i32); 8] = [
            (-2, -1),
            (-2, 1),
            (2, -1),
            (2, 1),
            (-1, -2),
            (-1, 2),
            (1, 2),
            (1, -2),
        ];


        for &(row_offset, col_offset) in &offsets {
            let target: Position = Position::new(
                (self.position.row as i32 + row_offset) as usize,
                (self.position.col as i32 + col_offset) as usize,
            );
            
            if target != *to_pos {
                return false;
            }

            if board.is_within_bounds(&target) {
                let piece_option: Option<&Piece> = Piece::get_piece(&target, board);
                let piece: Option<&Piece> = piece_option;

                // no piece we just put in

                if piece.is_none(){
                    return true;
                }

                let piece: &Piece = piece.unwrap();

                // if the piece is not is our color
                if piece.color != self.color {
                    return true;
                } 
            }
        }

        false
    }

    fn valid_moves_knight(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        let offsets: [(i32, i32); 8] = [
            (-2, -1),
            (-2, 1),
            (2, -1),
            (2, 1),
            (-1, -2),
            (-1, 2),
            (1, 2),
            (1, -2),
        ];

        for &(row_offset, col_offset) in &offsets {
            let target: Position = Position::new(
                (self.position.row as i32 + row_offset) as usize,
                (self.position.col as i32 + col_offset) as usize,
            );

            if board.is_within_bounds(&target) {
                let piece_option: Option<&Piece> = Piece::get_piece(&target, board);
                let piece: Option<&Piece> = piece_option;

                // no piece we just put in
                if piece.is_none() {
                    moves.push(target);
                    continue;
                }

                let piece: &Piece = piece.unwrap();

                // if the piece is not is our color
                if piece.color != self.color {
                    moves.push(target);
                }
            }
        }
        moves
    }
    //---------------------------------------------------------------------

    //Bishop---------------------------------------------------------------
    fn is_valid_move_bishop(&self, board: &Board, to_pos: &Position) -> bool {

        let offsets: [(i32, i32); 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

        // for each diag we explore the direction and add Position possible
        for &(row_offset, col_offset) in &offsets {
            if self.find(board, row_offset, col_offset, to_pos) {
                return true;
            }
        }

        false
    }

    fn valid_moves_bishop(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();

        let offsets: [(i32, i32); 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

        // for each diag we explore the direction and add Position possible
        for &(row_offset, col_offset) in &offsets {
            self.explore_direction(board, row_offset, col_offset, &mut moves)
        }
        moves
    }
    //---------------------------------------------------------------------

    //Rook-----------------------------------------------------------------
    fn is_valid_move_rook(&self, board: &Board, to_pos: &Position) -> bool {

        let offsets: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

        for &(row_offset, col_offset) in &offsets {
            if self.find(board, row_offset, col_offset, to_pos) {
                return true;
            }
        }

        false
    }

    fn valid_moves_rook(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();

        let offsets: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

        for &(row_offset, col_offset) in &offsets {
            self.explore_direction(board, row_offset, col_offset, &mut moves)
        }

        moves
    }
    //---------------------------------------------------------------------

    //Queen----------------------------------------------------------------
    fn is_valid_move_queen(&self, board: &Board, to_pos: &Position) -> bool {
        self.is_valid_move_bishop(board, to_pos) || self.is_valid_move_rook(board, to_pos)
    }

    //mix of Rook and Bishop
    fn valid_moves_queen(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = self.valid_moves_bishop(board);
        moves.extend(self.valid_moves_rook(board)); //add rook moves to  bishop moves from this position
        moves
    }
    //---------------------------------------------------------------------

    //King-----------------------------------------------------------------
    pub fn is_in_king_hitbox(&self, cell_pos: &Position) -> bool {
        if self.piece_type != PieceType::King { return false; }

        self.position == *cell_pos || (
            (self.position.row + 1 == cell_pos.row || self.position.row == cell_pos.row + 1) &&
            (self.position.col + 1 == cell_pos.col || self.position.col == cell_pos.col + 1))
    }

    fn is_valid_move_king(&self, board: &Board, to_pos: &Position) -> bool {

        let offsets: [(i32, i32); 8] = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        //Normal moves without
        for &(row_offset, col_offset) in &offsets {
            let target: Position = Position::new(
                (self.position.row as i32 + row_offset) as usize,
                (self.position.col as i32 + col_offset) as usize,
            );

            if target != *to_pos { return false; }

            if board.is_within_bounds(&target) && !board.is_attacked(&target, self.color) {

                match Piece::get_piece(&target, board) {
                    Some(piece) if piece.color != self.color && *to_pos == target => return true,
                    None if *to_pos == target => return true,
                    _ => continue
                };
            }
        }

        false
    }

    fn valid_moves_king(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();

        let offsets: [(i32, i32); 8] = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        //Normal moves without
        for &(row_offset, col_offset) in &offsets {
            let target: Position = Position::new(
                (self.position.row as i32 + row_offset) as usize,
                (self.position.col as i32 + col_offset) as usize,
            );

            if board.is_within_bounds(&target) && !board.is_attacked(&target, self.color) {
                // get the piece if there is no piece just add the position
                let piece: &Piece = if let Some(piece) = Piece::get_piece(&target, board) {
                    piece
                } else {
                    moves.push(target);
                    continue;
                };

                if piece.color != self.color {
                    moves.push(target);
                }
            }
        }

        // Vérifier les roques possibles
        //ajouter condition pour roques

        // If the king moved we can not castle
        if self.has_moved {
            return moves;
        }

        // all position for a possible castle
        let rook_positions: [Position; 2] = [
            Position::new(self.position.row, 0), // Tour côté dame
            Position::new(self.position.row, 7), // Tour côté roi
        ];

        for rook_position in rook_positions.iter() {
            // check if the condition are valid for a castle
            if board.can_castle(&self.position, rook_position) {
                // new king position for castle
                let new_king_position = if rook_position.col < self.position.col {
                    Position::new(self.position.row, self.position.col - 2)
                } else {
                    Position::new(self.position.row, self.position.col + 2)
                };

                moves.push(new_king_position);
            }
        }

        moves
    }

    // Same as explore_direction but when find the cell it's return
    fn find(
        &self,
        board: &Board,
        row_offset: i32,
        col_offset: i32,
        to_pos: &Position
    ) -> bool {
        // init position the piece position
        let mut position: Position = Position::new(self.position.row, self.position.col);

        loop {
            // the new position is the next cell
            position = Position::new(
                (position.row as i32 + row_offset) as usize,
                (position.col as i32 + col_offset) as usize,
            );

            if !board.is_within_bounds(&position) {
                break;
            }

            match Piece::get_piece(&position, board) {
                Some(piece) if piece.color != self.color && *to_pos == position => return true,
                None if *to_pos == position => return true,
                None => continue,
                _ => break // if there is a piece of our color
            };
        }

        false
    }

    //Check move for all cases in a direction until it a move is valid
    fn explore_direction(
        &self,
        board: &Board,
        row_offset: i32,
        col_offset: i32,
        moves: &mut Vec<Position>,
    ) {
        // init position the piece position
        let mut position: Position = Position::new(self.position.row, self.position.col);

        loop {
            // the new position is the next cell
            position = Position::new(
                (position.row as i32 + row_offset) as usize,
                (position.col as i32 + col_offset) as usize,
            );

            if !board.is_within_bounds(&position) {
                break;
            }

            match Piece::get_piece(&position, board) {
                Some(piece) if piece.color != self.color => moves.push(position),
                None => moves.push(position),
                _ => break // if there is a piece of our color
            };
        }
    }

    pub fn get_type(self) -> PieceType {
        self.piece_type
    }
}
