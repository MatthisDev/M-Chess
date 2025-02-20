use crate::game_lib::board::Board;
use crate::game_lib::position::Position;
const BOARD_SIZE: usize = 8;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece {
            color,
            piece_type,
            has_moved: false,
            //position
        }
    }

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

    fn valid_moves_pawn(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves = Vec::new();
        let direction = if self.color == Color::White { -1 } else { 1 };

        // Mouvement simple vers l'avant
        let forward = Position::new((position.row as i32 + direction) as usize, position.col);
        if board.is_within_bounds(forward) && board.squares[forward.row][forward.col].is_none() {
            moves.push(forward);

            // Mouvement de deux cases si le pion est à sa position initiale
            if (self.color == Color::White && position.row == 6)
                || (self.color == Color::Black && position.row == 1)
            {
                let double_forward =
                    Position::new((position.row as i32 + 2 * direction) as usize, position.col);
                if board.is_within_bounds(double_forward)
                    && board.squares[double_forward.row][double_forward.col].is_none()
                {
                    moves.push(double_forward);
                }
            }
        }

        // Capture diagonale
        for col_offset in &[-1, 1] {
            let capture = Position::new(
                (position.row as i32 + direction) as usize,
                (position.col as i32 + col_offset) as usize,
            );
            if board.is_within_bounds(capture) && board.squares[capture.row][capture.col].is_some()
            {
                if let Some(piece) = board.squares[capture.row][capture.col] {
                    if piece.color != self.color {
                        moves.push(capture);
                    }
                }
            }
        }

        //prise en passant

        moves
    }

    fn valid_moves_knight(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves = Vec::new();
        let offsets = [
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
            let target = Position::new(
                (position.row as i32 + row_offset) as usize,
                (position.col as i32 + col_offset) as usize,
            );
            if board.is_within_bounds(target) {
                if let Some(piece) = board.squares[target.row][target.col] {
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

    fn valid_moves_bishop(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves = Vec::new();
        let offsets = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

        for &(row_offset, col_offset) in &offsets {
            self.explore_direction(board, position, row_offset, col_offset, &mut moves)
        }
        moves
    }

    fn valid_moves_rook(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves = Vec::new();
        let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1)];

        for &(row_offset, col_offset) in &offsets {
            self.explore_direction(board, position, row_offset, col_offset, &mut moves)
        }
        moves
    }

    fn valid_moves_queen(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves = self.valid_moves_bishop(board, position);
        moves.extend(self.valid_moves_rook(board, position));
        moves
    }

    fn valid_moves_king(&self, board: &Board, position: Position) -> Vec<Position> {
        let mut moves = Vec::new();
        let offsets = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        for &(row_offset, col_offset) in &offsets {
            let target = Position::new(
                (position.row as i32 + row_offset) as usize,
                (position.col as i32 + col_offset) as usize,
            );
            if board.is_within_bounds(target) && !board.is_attacked(target, self.color) {
                if let Some(piece) = board.squares[target.row][target.col] {
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
        {
            let rook_positions = [
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

    // check toutes les cases de déplacement pour la tour et le fou
    //tant que la case est dans le board ou si il y a une piece qui l'empêche d'aller plus loin
    //si une piece le bloque, check si la pion peut-être mangé (couleur opposée)
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
            if let Some(piece) = board.squares[position.row][position.col] {
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
