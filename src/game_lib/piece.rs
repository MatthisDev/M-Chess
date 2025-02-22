use crate::game_lib::board::{Board, BOARD_SIZE};
use crate::game_lib::position::Position;

use super::position;

// Color enum for teams
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
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

//class Piece
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool, //for special moves
    position: Position,
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
    
    // get the piece from a specific position
    pub fn get_piece(postion: &Position, board: &Board) -> Option<&Piece> {
        let (i, j): (isize, isize) = board.squares[position.row][position.col];
        
        // when there is no piece
        if i == -1 || j == -1 {return None;}

        return Some(&board.pieces[i][j]);
    }
    
    //check moves for a Piece at (x,y) depending on his type
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
    
    //Pawn---------------------------------------------------------------------------------
    fn valid_moves_pawn(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        let direction: i32 = if self.color == Color::White { -1 } else { 1 };

        //Check Simple move forward
        let forward: Position =
            Position::new((self.position.row as i32 + direction) as usize, self.position.col);
        
        // TODO: It's here where we upgrade the PAWN => If the move is out_of_bound - 1 => UPGRADE
        // if we are out of the board and there is nothing on the cell
        if board.is_within_bounds(&forward) && board.squares[forward.row][forward.col] == (-1, -1) {
            moves.push(forward);

            //Check Double move forward if never moved
            //If the move +1 or -1 is not possible then the move + 2
            if (self.color == Color::White && self.position.row == 6) ||
               (self.color == Color::Black && self.position.row == 1)
            {
                let double_forward: Position =
                    Position::new((self.position.row as i32 + 2 * direction) as usize, self.position.col);
                
                // If there is nothing on the cell the move is possible. (no need to check the out of board)
                if board.squares[double_forward.row][double_forward.col] == (-1, -1) {
                    moves.push(double_forward);
                }
            }
        }

        // Check Diagonal capture
        for col_offset in &[-1, 1] {
            let capture: Position = Position::new(
                (self.position.row as i32 + direction) as usize,
                (self.position.col as i32 + col_offset) as usize,
            );

            if board.is_within_bounds(&capture) &&
               board.squares[capture.row][capture.col] != (-1, -1)
            {
                // get the piece if there is
                let piece: &Piece = 
                    if let Some(piece) = Piece::get_piece(&capture, board) { piece }
                    else { continue };

                if piece.color != self.color {
                    moves.push(capture)
                }
            }
        }

        //prise en passant

        //check historique
        //check à côté
        //validation

        moves
    }
    //-------------------------------------------------------------------------------------
    //Knight-------------------------------------------------------------------------------
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
                (position.row as i32 + row_offset) as usize,
                (position.col as i32 + col_offset) as usize
            );

            if board.is_within_bounds(&target) {
                let piece: Option<&Piece> = Piece::get_piece(&target);
                
                // no piece we just put in
                if piece == None {
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

    //------------------------------------------------------------------------------------
    //Bishop------------------------------------------------------------------------------
    fn valid_moves_bishop(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        
        let offsets: [(i32, i32); 4] = [
            (-1, -1),
            (1, -1),
            (1, 1), 
            (-1, 1)
        ];
        
        // for each diag we explore the direction and add Position possible
        for &(row_offset, col_offset) in &offsets {
            self.explore_direction(board, row_offset, col_offset, &mut moves)
        }
        moves
    }
    //-------------------------------------------------------------------------------------
    //Rook---------------------------------------------------------------------------------
    fn valid_moves_rook(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        
        let offsets: [(i32, i32); 4] = [
            (-1, 0), 
            (1, 0), 
            (0, 1), 
            (0, -1)
        ];

        for &(row_offset, col_offset) in &offsets {
            self.explore_direction(board, row_offset, col_offset, &mut moves)
        }

        moves
    }
    //--------------------------------------------------------------------------------------
    //Queen---------------------------------------------------------------------------------
    //mix of Rook and Bishop
    fn valid_moves_queen(&self, board: &Board) -> Vec<Position> {
        let mut moves: Vec<Position> = self.valid_moves_bishop(board);
        moves.extend(self.valid_moves_rook(board)); //add rook moves to  bishop moves from this position
        moves
    }
    //---------------------------------------------------------------------------------------
    //King-----------------------------------------------------------------------------------
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
                (position.row as i32 + row_offset) as usize,
                (position.col as i32 + col_offset) as usize,
            );
            
            if board.is_within_bounds(&target) && !board.is_attacked(&target, self.color) {
                
                // get the piece if there is no piece just add the position
                // TODO we add the position but no check if it's push the king in check
                let piece: &Piece =
                    if let Some(piece) = Piece::get_piece(&target, &board) { piece }
                    else { 
                        moves.push(target);
                        continue 
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
            Position::new(position.row, 0), // Tour côté dame
            Position::new(position.row, 7), // Tour côté roi
        ];

        for rook_position in rook_positions.iter() {
            // check if the condition are valid for a castle
            if board.can_castle(&self.position, &rook_position) {
                
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

    //Check move for all cases in a direction until it a move is valid
    fn explore_direction(
        &self,
        board: &Board,
        row_offset: i32,
        col_offset: i32,
        moves: &mut Vec<Position>,
    ) {
        // init position the piece position
        let mut postion: Position = Position::new(self.position.row, self.position.col);

        loop {

            // the new position is the next cell
            position = Position::new(
                (position.row as i32 + row_offset) as usize,
                (position.col as i32 + col_offset) as usize
            );

            if !board.is_within_bounds(&position) {
                break;
            }

            let piece: Option<&Piece> = Piece::get_piece(position, board);

            // if no piece then we add it to the vec and continue to other position
            if piece == None {
                moves.push(position);
                continue;
            }

            let piece: &Piece = piece.unwrap();
            // if we can EAT the piece because its color is diff
            if piece.color != self.color {
                moves.push(position);
            }

            break;
        }
    }

    pub fn get_type(self) -> PieceType {
        self.piece_type
    }
}
