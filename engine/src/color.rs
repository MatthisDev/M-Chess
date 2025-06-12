use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// Color enum for teams
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorError;

impl Error for ParseColorError {}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

// Convert color
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Color::White => write!(f, "w"),
            Color::Black => write!(f, "b"),
        }
    }
}

impl Display for ParseColorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Unknown color")
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(ParseColorError),
        }
    }
}

// Convert u8 to color
impl From<u8> for Color {
    fn from(value: u8) -> Color {
        match value {
            0 => Color::Black,
            _ => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_opposite() {
        assert_eq!(Color::Black.opposite(), Color::White);
        assert_eq!(Color::White.opposite(), Color::Black);
    }

    #[test]
    fn t_to_string() {
        assert_eq!(Color::Black.to_string(), "b".to_string());
        assert_eq!(Color::White.to_string(), "w".to_string());
    }

    #[test]
    fn t_from_string() {
        assert_eq!("b".parse::<Color>(), Ok(Color::Black));
        assert_eq!("w".parse::<Color>(), Ok(Color::White));
        assert_eq!("r".parse::<Color>(), Err(ParseColorError));
    }

    fn t_from_u8() {}
}
