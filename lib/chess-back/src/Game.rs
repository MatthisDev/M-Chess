const BOARD_SIZE: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn from_algebric(algebric: &str) -> Result<Self, &'static str> {
        if algebric.len() != 2 {
            return Err("Invalid algebric notation");
        }

        let col = algebric.chars().next().unwrap() as usize - 'a' as usize;
        let row = 8 - (algebric.chars().nth(1).unwrap() as usize - '0' as usize);

        if col >= BOARD_SIZE || row >= BOARD_SIZE {
            return Err("Invalid algebric notation");
        }
        Ok(Position { row, col })
    }

    fn too_algebric(&self) -> String {
        format!(
            "{}{}",
            ('a' as usize + self.col) as u8 as char,
            8 - self.row
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Piece {
    color: Color,
    piece_type: PieceType,
}
impl Piece {
    fn new(color: Color, piece_type: PieceType) -> Self {
        Piece { color, piece_type }
    }

    fn valid_move(&self, board: &Board, position: Position) -> Vec<Position> {
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

        //Mouvement simple vers l'avant
        let forward = Position::new((position.row as i32 + direction) as usize, position.col);
        if board.is_within_bounds(forward) && board.square[forward.row][forward.col].is_none() {
            moves.push(forward);

            //Mouvement de 2 cases si le pion est à sa position initiale
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
        //capture diagonale
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
                //Check si il y a  un pion
                if let Some(piece) = board.squares[target.row][target.col] {
                    if piece.color != self.color {
                        moves.push(target);
                    }
                }
                //Si None
                else {
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
        let mut moves1 = self.valid_moves_bishop(board, position);
        let mut moves2 = (self.valid_moves_rook(board, position));
        moves1.append(&mut moves2); //ajoute les mvmts dans le 1er vec
        moves1
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
                //Check si il y a  un pion
                if let Some(piece) = board.squares[target.row][target.col] {
                    if piece.color != self.color {
                        moves.push(target);
                    }
                }
                //Si None
                else {
                    moves.push(target);
                }
            }
        }
        moves
    }

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
                    moves.push(position)
                }
                break;
            }
            moves.push(position);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Board {
    squares: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    turn: Color,
}

impl Board {
    fn new() -> Self {
        let mut squares = [[None; BOARD_SIZE]; BOARD_SIZE];

        //Initialise les soldats
        for col in 0..BOARD_SIZE {
            square[1][col] = Some(Piece::new(Color::Black, PieceType::Pawn));
            square[6][col] = Some(Piece::new(Color::White, PieceType::Pawn));
        }
        // Initailisation des tours
        square[0][0] = Some(Piece::new(Color::Black, PieceType::Rook));
        square[0][7] = Some(Piece::new(Color::Black, PieceType::Rook));
        square[7][0] = Some(Piece::new(Color::White, PieceType::Rook));
        square[7][7] = Some(Piece::new(Color::White, PieceType::Rook));

        //Initialisation des cavaliers
        square[0][1] = Some(Piece::new(Color::Black, PieceType::Knight));
        square[0][6] = Some(Piece::new(Color::Black, PieceType::Knight));
        square[7][1] = Some(Piece::new(Color::White, PieceType::Knight));
        square[7][6] = Some(Piece::new(Color::White, PieceType::Knight));

        //Initialisation des fous
        square[0][2] = Some(Piece::new(Color::Black, PieceType::Bishop));
        square[0][5] = Some(Piece::new(Color::Black, PieceType::Bishop));
        square[7][2] = Some(Piece::new(Color::White, PieceType::Bishop));
        square[7][5] = Some(Piece::new(Color::White, PieceType::Bishop));

        //Initialisation des rois et reines
        square[0][3] = Some(Piece::new(Color::Black, PieceType::Queen));
        square[0][4] = Some(Piece::new(Color::Black, PieceType::King));

        square[7][3] = Some(Piece::new(Color::Black, PieceType::Queen));
        square[7][4] = Some(Piece::new(Color::Black, PieceType::King));
    }

    fn move_piece(&mut self, from: position, to: Position) -> bool {
        if self.is_valid_move(from, to) {
            self.squares[to.row][to.col] = self.squares[from.row][from.col];
            self.squares[from.row][from.col] = None;

            self.turn = if self.turn == Color::White {
                Color::Black
            } else {
                Color::White
            };
            true
        } else {
            false
        }
    }

    fn is_valid_move(&self, from: Position, to: Position) -> bool {
        if let Some(piece) = self.squares[from.row][from.col] {
            let valid_moves = piece.is_valid_move(self, from);
            valid_moves.contains(&to)
        } else {
            false
        }
    }

    fn is_within_bounds(&self, position: Position) -> bool {
        positon.row < BOARD_SIZE
            && position.row >= 0
            && position.col < BOARD_SIZE
            && position.col >= 0
    }

    fn is_attacked(&self, position: Position, color: Color) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[raw][col] {
                    if piece.color != color {
                        let valid_moves = piece.valid_moves(self, Position::new(raw, col));
                        if valid_move.contains(&position) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn is_king_in_check(&self, color: Color) -> bool {
        let king_position = self.find_king(color);
        if let Some(position) = king_position {
            self.is_attacked(position, color)
        } else {
            false
        }
    }

    fn find_king(&self, color: Color) -> Option<Position> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[row][col] {}
            }
        }
    }
}

struct Game {
    board: Board,
    history: Vec<(Position, Position)>,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::new(),
            history: Vec::new(),
        }
    }

    fn make_move(&mut self, from: Position, to: Position) -> bool {
        if self.board.move_piece(from, to) {
            self.history.push((from, to));
            true
        } else {
            false
        }
    }

    fn undo_move(&mut self) {
        if let Some((from, to)) = self.history.pop() {
            self.board.move_piece(to, from);
            self.board.turn = if self.board.turn == Color::White {
                Color::Black
            } else {
                Color::White
            };
        }
    }

    fn is_game_over(&self) -> bool {
        false
    }
}
