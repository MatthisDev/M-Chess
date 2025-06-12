use serde::{Deserialize, Serialize};

use crate::board::Board;
use crate::game::*;
use crate::piece::{Color, Piece, PieceType};
use crate::position::Position;

use crate::position;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Difficulty {}

#[derive(Clone, Debug, PartialEq)]
pub struct AI {}

impl AI {}
