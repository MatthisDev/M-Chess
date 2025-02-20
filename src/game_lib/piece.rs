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
            //position
            position,
        }
    }

    //check moves for a Piece at (x,y) depending on his type
    pub fn valid_moves(&self, board: &Board, position: Position) -> Vec<Position> {
        match self.piece_type {
            PieceType::Pawn => self.valid_moves_pawn(board, position),
            PieceType::Knight => self.valid_moves_knight(board, position),
            PieceType::Bishop => self.valid_moves_bishop(board, position),
            PieceType::Rook => self.valid_moves_rook(board, position),
            PieceType::Queen => self.valid_moves_queen(board, position),
            PieceType::King => self.valid_moves_king(board, position),
        }
    }

    //Pawn---------------------------------------------------------------------------------
    fn valid_moves_pawn(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        let direction: i32 = if self.color == Color::White { -1 } else { 1 };

        //Check Simple move forward
        let forward: Position =
            Position::new((position.row as i32 + direction) as usize, position.col);
        if board.is_within_bounds(forward) && board.squares[forward.row][forward.col] == (-1, -1) {
            moves.push(forward);

            //Check Double move forward if never moved
            if (self.color == Color::White && position.row == 6)
                || (self.color == Color::Black && position.row == 1)
            {
                let double_forward: Position =
                    Position::new((position.row as i32 + 2 * direction) as usize, position.col);
                if board.is_within_bounds(double_forward)
                    && board.squares[double_forward.row][double_forward.col] == (-1, -1)
                {
                    moves.push(double_forward);
                }
            }
        }

        // Check Diagonal capture
        for col_offset in &[-1, 1] {
            let capture: Position = Position::new(
                (position.row as i32 + direction) as usize,
                (position.col as i32 + col_offset) as usize,
            );
            if board.is_within_bounds(capture)
                && board.squares[capture.row][capture.col] != (-1, -1)
            {
                if let Some(piece) = board.pieces
                    [board.squares[capture.row][capture.col].0 as usize]
                    [board.squares[capture.row][capture.col].1 as usize]
                {
                    if piece.color != self.color {
                        moves.push(capture);
                    }
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
    fn valid_moves_knight(&self, board: &Board, position: Position) -> Vec<Position> {
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
                (position.col as i32 + col_offset) as usize,
            );
            if board.is_within_bounds(target) {
                if let Some(piece) = board.pieces[board.squares[target.row][target.col].0 as usize]
                    [board.squares[target.row][target.col].1 as usize]
                {
                    if piece.color != self.color {
                        moves.push(target);
                    }
                } else {
                    moves.push(target);
                }
            }
        }
        moves
    }
    //------------------------------------------------------------------------------------

    //Bishop------------------------------------------------------------------------------
    fn valid_moves_bishop(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        let offsets: [(i32, i32); 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

        for &(row_offset, col_offset) in &offsets {
            self.explore_direction(board, position, row_offset, col_offset, &mut moves)
        }
        moves
    }
    //-------------------------------------------------------------------------------------

    //Rook---------------------------------------------------------------------------------
    fn valid_moves_rook(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        let offsets: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

        for &(row_offset, col_offset) in &offsets {
            self.explore_direction(board, position, row_offset, col_offset, &mut moves)
        }
        moves
    }
    //--------------------------------------------------------------------------------------

    //Queen---------------------------------------------------------------------------------
    //mix of Rook and Bishop
    fn valid_moves_queen(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves: Vec<Position> = self.valid_moves_bishop(board, position);
        moves.extend(self.valid_moves_rook(board, position)); //add rook moves to  bishop moves from this position
        moves
    }
    //---------------------------------------------------------------------------------------

    //King-----------------------------------------------------------------------------------
    fn valid_moves_king(&self, board: &Board, position: Position) -> Vec<Position> {
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
            if board.is_within_bounds(target) && !board.is_attacked(target, self.color) {
                if let Some(piece) = board.pieces[board.squares[target.row][target.col].0 as usize]
                    [board.squares[target.row][target.col].1 as usize]
                {
                    if piece.color != self.color {
                        moves.push(target);
                    }
                } else {
                    moves.push(target);
                }
            }
        }

        // Vérifier les roques possibles
        //ajouter condition pour roques
        if !board.pieces[board.squares[position.row][position.col].0 as usize]
            [board.squares[position.row][position.col].1 as usize]
            .unwrap()
            .has_moved
        {
            let rook_positions: [Position; 2] = [
                Position::new(position.row, 0), // Tour côté dame
                Position::new(position.row, 7), // Tour côté roi
            ];

            for rook_position in rook_positions.iter() {
                if board.can_castle(position, *rook_position) {
                    let new_king_position = if rook_position.col < position.col {
                        Position::new(position.row, position.col - 2)
                    } else {
                        Position::new(position.row, position.col + 2)
                    };
                    moves.push(new_king_position);
                }
            }
        }

        moves
    }

    //Check move for all cases in a direction until it a move is valid
    fn explore_direction(
        &self,
        board: &Board,
        mut position: Position,
        row_offset: i32,
        col_offset: i32,
        moves: &mut Vec<Position>,
    ) {
        loop {
            position.row = (position.row as i32 + row_offset) as usize;
            position.col = (position.col as i32 + col_offset) as usize;

            if !board.is_within_bounds(position) {
                break;
            }
            if let Some(piece) = board.pieces[board.squares[position.row][position.col].0 as usize]
                [board.squares[position.row][position.col].1 as usize]
            {
                if piece.color != self.color {
                    moves.push(position);
                }
                break;
            }
            moves.push(position);
        }
    }

    pub fn get_type(self) -> PieceType {
        self.piece_type
    }
}
