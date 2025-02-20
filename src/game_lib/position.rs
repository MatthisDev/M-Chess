const BOARD_SIZE: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    pub fn from_algebraic(algebraic: &str) -> Position {
        let col = algebraic.chars().next().unwrap() as usize - 'a' as usize;
        let row = 8 - (algebraic.chars().nth(1).unwrap() as usize - '0' as usize);

        if col >= BOARD_SIZE || row >= BOARD_SIZE {
            panic!("Invalid algebraic notation");
        }
        Position { row, col }
    }

    pub fn to_algebraic(&self) -> String {
        format!(
            "{}{}",
            ('a' as usize + self.col) as u8 as char,
            8 - self.row
        )
    }
}
