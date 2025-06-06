use crate::board::BOARD_SIZE;

//class position is a tupple of usize corresponding to the position on the board
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Position {
    //crate a new Position
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    // convert a slice to a Position
    pub fn from_algebraic(algebraic: &str) -> Result<Position, &'static str> {
        if !algebraic.chars().nth(0).unwrap().is_ascii_lowercase()
            || !algebraic.chars().nth(1).unwrap().is_ascii_digit()
        {
            return Err("parse_move_str: invalid send string: <{move_piece}>");
        }
        let col: usize = algebraic.chars().next().unwrap() as usize - 'a' as usize;
        let row: usize = algebraic.chars().nth(1).unwrap() as usize - '0' as usize;

        if col >= BOARD_SIZE || row >= BOARD_SIZE {
            return Err("parse_move_str: invalid send string: <{move_piece}>");
        }

        Ok(Position { row, col })
    }

    // Convert a Position to a String
    pub fn to_algebraic(self) -> String {
        format!("{}{}", ('a' as usize + self.col) as u8 as char, self.row)
    }
    pub fn is_center(&self) -> bool {
        (self.row == 3 || self.row == 4) && (self.col == 3 || self.col == 4)
    }
}
