use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    file: u8, // Files are the column [a-h]
    rank: u8, // Ranks are the lines [1-8]
              // in practice file and rank refer to two index then the real range value
              // is in [0-7] for both value.
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePositionError;

impl Error for ParsePositionError {}

impl Position {
    // init with integer
    pub fn init(file_: u8, rank_: u8) -> Self {
        if file_ > 7 || rank_ > 7 {
            panic!("Position::init invalid args");
        }

        Position {
            file: file_,
            rank: rank_,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let char_file: char = match char::from_u32(self.file as u32 + 'a' as u32) {
            Some(val) => val,
            None => panic!("Convertion rank -> char isn't possible"),
        };

        write!(f, "{}{}", char_file, self.rank + 1)
    }
}

impl Display for ParsePositionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Unknown chess position")
    }
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim();

        if s.len() != 2 {
            return Err(ParsePositionError);
        }

        let file: u8 = match s.chars().nth(0).unwrap() {
            c @ 'a'..='h' => c as u8 - b'a',
            _ => return Err(ParsePositionError),
        };

        let rank: u8 = match s.chars().nth(1).unwrap() {
            c @ '1'..='8' => c as u8 - b'1',
            _ => return Err(ParsePositionError),
        };

        println!("file: {file}, rank {rank}");

        Ok(Position::init(file, rank))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_init() {
        assert_eq!(Position::init(1, 1), Position { file: 1, rank: 1 });
    }

    #[test]
    #[should_panic(expected = "Position::init invalid args")]
    fn t_init_panic() {
        Position::init(200, 1);
    }

    #[test]
    fn t_to_string() {
        assert_eq!(Position::init(0, 0).to_string(), String::from("a1"));
        assert_eq!(Position::init(7, 7).to_string(), String::from("h8"));
        assert_eq!(Position::init(5, 6).to_string(), String::from("f7"));
    }

    #[test]
    fn t_from_string() {
        assert_eq!("a1".parse::<Position>(), Ok(Position::init(0, 0)));
        assert_eq!("h8".parse::<Position>(), Ok(Position::init(7, 7)));
        assert_eq!("e6".parse::<Position>(), Ok(Position::init(4, 5)));
        assert_eq!("v1".parse::<Position>(), Err(ParsePositionError));
    }
}
