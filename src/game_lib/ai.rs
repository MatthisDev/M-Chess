use crate::game_lib::board::Board;
use crate::game_lib::game::*;
use crate::game_lib::position::Position;

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

struct AI {
    difficulty: Difficulty,
}

impl AI {
    fn new(difficulty: Difficulty) -> Self {
        AI { difficulty }
    }

    fn get_best_move(&self, board: &Board) -> (Position, Position) {
        (Position::new(0, 0), Position::new(1, 0))
    }

    fn evaluate_board(&self, board: &Board) -> i32 {
        0
    }
}
