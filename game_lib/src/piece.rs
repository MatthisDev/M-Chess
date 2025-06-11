use serde::{Deserialize, Serialize};

// Color enum for teams
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    Black,
    White,
}
impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Color {
        match value {
            0 => Color::Black,
            _ => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {}

impl PieceType {}

//class Piece
#[derive(Debug, Clone, PartialEq)]
pub struct Piece {}

impl Piece {}
