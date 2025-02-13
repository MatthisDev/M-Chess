use std::array::from_fn;

const BOARD_SIZE: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn from_algebraic(algebraic: &str) -> Result<Self, &'static str> {
        if algebraic.len() != 2 {
            return Err("Invalid algebraic notation");
        }

        let col = algebraic.chars().next().unwrap() as usize - 'a' as usize;
        let row = 8 - (algebraic.chars().nth(1).unwrap() as usize - '0' as usize);

        if col >= BOARD_SIZE || row >= BOARD_SIZE {
            return Err("Invalid algebraic notation");
        }
        Ok(Position { row, col })
    }

    /*fn to_algebraic(&self) -> String {
        format!(
            "{}{}",
            ('a' as usize + self.col) as u8 as char,
            8 - self.row
        )
    }*/
}

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
    color: Color,
    piece_type: PieceType,
    has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece {
            color,
            piece_type,
            has_moved: false,
        }
    }

    fn valid_moves(&self, board: &Board, position: Position) -> Vec<Position> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct AttackInfo {
    attacked_by: Vec<(Position, Piece)>,
}

impl AttackInfo {
    fn new() -> Self {
        AttackInfo {
            attacked_by: Vec::new(),
        }
    }

    fn add_attacker(&mut self, position: Position, piece: Piece) {
        self.attacked_by.push((position, piece));
    }

    fn is_attacked_by(&self, color: Color) -> bool {
        self.attacked_by
            .iter()
            .any(|(_, piece)| piece.color != color)
    }

    fn clear(&mut self) {
        self.attacked_by.clear();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub squares: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    attack_info: [[AttackInfo; BOARD_SIZE]; BOARD_SIZE],
    pub turn: Color,
}

impl Board {
    pub fn init() -> Self {
        let mut squares = [[None; BOARD_SIZE]; BOARD_SIZE];
        let attack_info = from_fn(|_| from_fn(|_| AttackInfo::new()));

        // Initialiser les pions
        for col in 0..BOARD_SIZE {
            squares[1][col] = Some(Piece::new(Color::Black, PieceType::Pawn));
            squares[6][col] = Some(Piece::new(Color::White, PieceType::Pawn));
        }

        // Initialiser les tours
        squares[0][0] = Some(Piece::new(Color::Black, PieceType::Rook));
        squares[0][7] = Some(Piece::new(Color::Black, PieceType::Rook));
        squares[7][0] = Some(Piece::new(Color::White, PieceType::Rook));
        squares[7][7] = Some(Piece::new(Color::White, PieceType::Rook));

        // Initialiser les cavaliers
        squares[0][1] = Some(Piece::new(Color::Black, PieceType::Knight));
        squares[0][6] = Some(Piece::new(Color::Black, PieceType::Knight));
        squares[7][1] = Some(Piece::new(Color::White, PieceType::Knight));
        squares[7][6] = Some(Piece::new(Color::White, PieceType::Knight));

        // Initialiser les fous
        squares[0][2] = Some(Piece::new(Color::Black, PieceType::Bishop));
        squares[0][5] = Some(Piece::new(Color::Black, PieceType::Bishop));
        squares[7][2] = Some(Piece::new(Color::White, PieceType::Bishop));
        squares[7][5] = Some(Piece::new(Color::White, PieceType::Bishop));

        // Initialiser les rois et reines
        squares[0][3] = Some(Piece::new(Color::Black, PieceType::Queen));
        squares[0][4] = Some(Piece::new(Color::Black, PieceType::King));
        squares[7][3] = Some(Piece::new(Color::White, PieceType::Queen));
        squares[7][4] = Some(Piece::new(Color::White, PieceType::King));

        Board {
            squares,
            attack_info,
            turn: Color::White,
        }
    }

    pub fn print_board(&self) {
        println!("  a b c d e f g h");
        for row in 0..BOARD_SIZE {
            print!("{} ", 8 - row);
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[row][col] {
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
                } else {
                    print!(". ");
                }
            }
            println!("{}", 8 - row);
        }
        println!("  a b c d e f g h");
    }

    fn update_attack_info(&mut self) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                self.attack_info[row][col].clear();
            }
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[row][col] {
                    let valid_moves = piece.valid_moves(self, Position::new(row, col));
                    for pos in valid_moves {
                        self.attack_info[pos.row][pos.col]
                            .add_attacker(Position::new(row, col), piece);
                    }
                }
            }
        }
    }

    fn is_attacked(&self, position: Position, color: Color) -> bool {
        self.attack_info[position.row][position.col].is_attacked_by(color)
    }

    fn move_piece(&mut self, from: Position, to: Position) -> bool {
        if let Some(mut piece) = self.squares[from.row][from.col] {
            if self.is_valid_move(from, to) {
                piece.has_moved = true;
                self.squares[to.row][to.col] = Some(piece);
                self.squares[from.row][from.col] = None;
                self.turn = if self.turn == Color::White {
                    Color::Black
                } else {
                    Color::White
                };
                self.update_attack_info();
                return true;
            }
        }
        false
    }

    fn is_valid_move(&self, from: Position, to: Position) -> bool {
        if let Some(piece) = self.squares[from.row][from.col] {
            let valid_moves = piece.valid_moves(self, from);
            valid_moves.contains(&to)
        } else {
            false
        }
    }

    fn is_within_bounds(&self, position: Position) -> bool {
        position.row < BOARD_SIZE && position.col < BOARD_SIZE
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
                if let Some(piece) = self.squares[row][col] {
                    if piece.color == color && piece.piece_type == PieceType::King {
                        return Some(Position::new(row, col));
                    }
                }
            }
        }
        None
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        if !self.is_king_in_check(color) {
            return false;
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[row][col] {
                    if piece.color == color {
                        let valid_moves = piece.valid_moves(self, Position::new(row, col));
                        for mv in valid_moves {
                            if self.is_move_safe(piece, Position::new(row, col), mv) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    fn is_move_safe(&self, piece: Piece, from: Position, to: Position) -> bool {
        let mut new_board = self.clone();
        new_board.move_piece(from, to);
        !new_board.is_king_in_check(piece.color)
    }

    fn can_castle(&self, king_position: Position, rook_position: Position) -> bool {
        let king = self.squares[king_position.row][king_position.col].unwrap();
        let rook = self.squares[rook_position.row][rook_position.col].unwrap();

        // Vérifier que le roi et la tour n'ont pas bougé
        if king.has_moved || rook.has_moved {
            return false;
        }

        // Vérifier qu'il n'y a pas de pièces entre le roi et la tour
        let (min_col, max_col) = if king_position.col < rook_position.col {
            (king_position.col + 1, rook_position.col)
        } else {
            (rook_position.col + 1, king_position.col)
        };

        for col in min_col..max_col {
            if self.squares[king_position.row][col].is_some() {
                return false;
            }
        }

        // Vérifier que le roi n'est pas en échec et ne traverse pas des cases attaquées
        for col in min_col..=max_col {
            let pos = Position::new(king_position.row, col);
            if self.is_attacked(pos, king.color) {
                return false;
            }
        }

        true
    }

    fn perform_castle(&mut self, king_position: Position, rook_position: Position) {
        let new_king_position = if rook_position.col < king_position.col {
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

        // Déplacer le roi et la tour
        self.move_piece(king_position, new_king_position);
        self.move_piece(rook_position, new_rook_position);
    }
}

pub struct Game {
    pub board: Board,
    history: Vec<(Position, Position)>,
}

impl Game {
    pub fn init() -> Self {
        Game {
            board: Board::init(),
            history: Vec::new(),
        }
    }

    pub fn make_move_algebraic(&mut self, from: &str, to: &str) -> Result<bool, &'static str> {
        let from_pos = Position::from_algebraic(from)?;
        let to_pos = Position::from_algebraic(to)?;

        if let Some(piece) = self.board.squares[from_pos.row][from_pos.col] {
            if piece.piece_type == PieceType::King {
                // Vérifier si le mouvement est un roque
                let rook_positions = [
                    Position::new(from_pos.row, 0), // Tour côté dame
                    Position::new(from_pos.row, 7), // Tour côté roi
                ];

                for rook_position in rook_positions.iter() {
                    if self.board.can_castle(from_pos, *rook_position)
                        && (to_pos == Position::new(from_pos.row, from_pos.col - 2)
                            || to_pos == Position::new(from_pos.row, from_pos.col + 2))
                    {
                        self.board.perform_castle(from_pos, *rook_position);
                        self.history.push((from_pos, to_pos));
                        return Ok(true);
                    }
                }
            }
        }
        if self.board.is_valid_move(from_pos, to_pos) {
            self.board.move_piece(from_pos, to_pos);
            self.history.push((from_pos, to_pos));

            if self.board.is_king_in_check(self.board.turn) {
                self.undo_move();
                return Err("Le roi est toujours en échec après ce mouvement.");
            }
            println!("Success!");

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
            Ok(true)
        } else {
            Err("Mouvement invalide.")
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
}
