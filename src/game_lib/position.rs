use crate::game_lib::board::BOARD_SIZE;

//class position is a tupple of usize corresponding to the position on the board
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    //crate a new Position
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    // convert a slice to a Position
    pub fn from_algebraic(algebraic: &str) -> Position {
        let col: usize = algebraic.chars().next().unwrap() as usize - 'a' as usize;
        let row: usize = 8 - (algebraic.chars().nth(1).unwrap() as usize - '0' as usize);

        if col >= BOARD_SIZE || row >= BOARD_SIZE {
            panic!("Invalid algebraic notation");
        }
        Position { row, col }
    }

    //Convert a Position to a String
    pub fn to_algebraic(&self) -> String {
        format!(
            "{}{}",
            ('a' as usize + self.col) as u8 as char,
            8 - self.row
        )
    }
}
