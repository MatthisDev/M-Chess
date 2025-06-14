use super::board::Board;
use super::color::Color;
use super::position::Position;

pub trait Move {
    fn is_move(&self, to: &Position, board: &Board) -> bool;
    fn get_moves(&self, to: &Position, board: &Board) -> Vec<Position>;
}

pub struct Piece {}
pub enum PieceType {}
