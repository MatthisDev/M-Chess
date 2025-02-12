use crate::components::board::Board;

const EMPTY_PIECE: &str = "   ";
const PIECE_NAME: [&str; 7] = [
    EMPTY_PIECE,
    "King",
    "Queen",
    "Rook",
    "Bishop",
    "Knight",
    "Pawn",
];
const PIECEVALUE: [u32; 7] = [0, 0, 9, 5, 3, 3, 1];

const OVERFLOW_TBL: [i8; 120] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3,
    4, 5, 6, 7, -1, -1, 8, 9, 10, 11, 12, 13, 14, 15, -1, -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, 32, 33, 34, 35, 36, 37, 38, 39, -1, -1, 40, 41, 42,
    43, 44, 45, 46, 47, -1, -1, 48, 49, 50, 51, 52, 53, 54, 55, -1, -1, 56, 57, 58, 59, 60, 61, 62,
    63, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];
const PLACEMENT_TBL: [i8; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66, 67, 68, 71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88, 91, 92, 93, 94, 95, 96, 97, 98,
];

#[derive(Clone, Debug)]
pub struct Piece {
    pub name: &'static str,
    pub color: &'static str,
    pub ennemy_color: &'static str,
    pub value: u32,
    pub moved: bool,
    pub display_name: String,
}

impl Piece {
    pub fn new(name: &'static str, color: &'static str) -> Self {
        let ennemy_color = if color == "blanc" { "noir" } else { "blanc" };
        let value = PIECEVALUE[PIECE_NAME.iter().position(|&x| x == name).unwrap_or(0)];
        let moved = false;
        let display_name = {
            if name.is_empty() {
                "".to_string()
            } else if name == "Pawn" {
                format!("i{}", color.chars().next().unwrap().to_lowercase())
            } else {
                format!(
                    "{}{}",
                    name.chars().next().unwrap().to_uppercase(),
                    color.chars().next().unwrap().to_lowercase()
                )
            }
        };

        Piece {
            name,
            color,
            ennemy_color,
            value,
            moved,
            display_name,
        }
    }

    //==============================================================================
    // Coups possibles
    //==============================================================================

    //===========================
    // Pion
    //===========================
    pub fn liste_coups_possibles_pion(&self, position: usize, board: &Board) -> Vec<usize> {
        let mut possibilities_list = Vec::new();

        if self.color == "blanc" {
            // mv de 2 si il se trouve sur la ligne 1
            if (48..=55).contains(&position) {
                let position_e = OVERFLOW_TBL[PLACEMENT_TBL[position] as usize - 20];
                if board.cases[position_e as usize].name == EMPTY_PIECE {
                    possibilities_list.push(position_e as usize);
                }
            }

            // Manger haut droite
            let position_e = OVERFLOW_TBL[PLACEMENT_TBL[position] as usize - 9];
            if position_e != -1 && board.cases[position_e as usize].color == "noir" {
                possibilities_list.push(position_e as usize);
            }

            // Manger haut gauche
            let position_e = OVERFLOW_TBL[PLACEMENT_TBL[position] as usize - 11];
            if position_e != -1 && board.cases[position_e as usize].color == "noir" {
                possibilities_list.push(position_e as usize);
            }

            // mv classique du pion
            let position_e = OVERFLOW_TBL[PLACEMENT_TBL[position] as usize - 10];
            if position_e != -1 && board.cases[position_e as usize].name == EMPTY_PIECE {
                possibilities_list.push(position_e as usize);
            }
        } else {
            // mv de 2 si il se trouve sur la ligne 6
            if (8..=15).contains(&position) {
                let position_e = OVERFLOW_TBL[PLACEMENT_TBL[position] as usize + 20];
                if board.cases[position_e as usize].name == EMPTY_PIECE {
                    possibilities_list.push(position_e as usize);
                }
            }

            // Manger bas gauche
            let position_e = OVERFLOW_TBL[PLACEMENT_TBL[position] as usize + 9];
            if position_e != -1 && board.cases[position_e as usize].color == "blanc" {
                possibilities_list.push(position_e as usize);
            }

            // Manger bas droite
            let position_e = OVERFLOW_TBL[PLACEMENT_TBL[position] as usize + 11];
            if position_e != -1 && board.cases[position_e as usize].color == "blanc" {
                possibilities_list.push(position_e as usize);
            }

            // mv classique du pion
            let position_e = OVERFLOW_TBL[PLACEMENT_TBL[position] as usize + 10];
            if position_e != -1 && board.cases[position_e as usize].name == EMPTY_PIECE {
                possibilities_list.push(position_e as usize);
            }
        }

        possibilities_list
    }

    //===========================
    // Rook
    //===========================
    pub fn liste_coups_possibles_tour(&self, position: usize, board: &Board) -> Vec<usize> {
        let moves = [-10, 10, -1, 1];
        let opposit_color_case = if self.color == "white" {
            "black"
        } else {
            "white"
        };

        let mut possibilities_list = Vec::new();

        for &mv in &moves {
            let mut multiplier = 1;
            let mut position_e = OVERFLOW_TBL[(PLACEMENT_TBL[position] as isize + mv) as usize];

            while position_e != -1 {
                if board.cases[position_e as usize].color == self.color {
                    break;
                }

                if board.cases[position_e as usize].color == opposit_color_case {
                    possibilities_list.push(position_e as usize);
                    break;
                } else if board.cases[position_e as usize].name == EMPTY_PIECE {
                    possibilities_list.push(position_e as usize);
                }

                multiplier += 1;
                position_e =
                    OVERFLOW_TBL[(PLACEMENT_TBL[position] as isize + mv * multiplier) as usize];
            }
        }
        possibilities_list
    }

    //===========================
    // Bishop
    //===========================
    pub fn liste_coups_possibles_fou(&self, position: usize, board: &Board) -> Vec<usize> {
        let moves = [-11, -9, 11, 9];
        let opposit_color_case = if self.color == "white" {
            "black"
        } else {
            "white"
        };

        let mut possibilities_list = Vec::new();

        for &mv in &moves {
            let mut multiplier = 1;
            let mut position_e = OVERFLOW_TBL[(PLACEMENT_TBL[position] as isize + mv) as usize];

            while position_e != -1 {
                if board.cases[position_e as usize].color == self.color {
                    break;
                }

                if board.cases[position_e as usize].color == opposit_color_case {
                    possibilities_list.push(position_e as usize);
                    break;
                } else if board.cases[position_e as usize].name == EMPTY_PIECE {
                    possibilities_list.push(position_e as usize);
                }

                multiplier += 1;
                position_e =
                    OVERFLOW_TBL[(PLACEMENT_TBL[position] as isize + mv * multiplier) as usize];
            }
        }
        possibilities_list
    }
    //===========================
    // Knight
    //===========================
    pub fn liste_coups_possibles_cavalier(&self, position: usize, board: &Board) -> Vec<usize> {
        let moves = [-12, -21, -19, -8, 12, 21, 19, 8];
        let opposit_color_case = if self.color == "white" {
            "black"
        } else {
            "white"
        };

        let mut possibilities_list = Vec::new();

        for &mv in &moves {
            let mut multiplier = 1;
            let mut position_e = OVERFLOW_TBL[(PLACEMENT_TBL[position] as isize + mv) as usize];

            while position_e != -1 {
                if board.cases[position_e as usize].color == self.color {
                    break;
                }

                if board.cases[position_e as usize].color == opposit_color_case {
                    possibilities_list.push(position_e as usize);
                    break;
                } else if board.cases[position_e as usize].name == EMPTY_PIECE {
                    possibilities_list.push(position_e as usize);
                }

                multiplier += 1;
                position_e =
                    OVERFLOW_TBL[(PLACEMENT_TBL[position] as isize + mv * multiplier) as usize];
            }
        }
        possibilities_list
    }
    //===========================
    // King
    //===========================
    pub fn liste_coups_possibles_roi(&self, position: usize, board: &Board) -> Vec<usize> {
        let moves = [-11, -10, -9, -1, 1, 9, 10, 11];
        let opposit_color_case = if self.color == "white" {
            "black"
        } else {
            "white"
        };

        let mut possibilities_list = Vec::new();

        for &mv in &moves {
            let mut multiplier = 1;
            let mut position_e = OVERFLOW_TBL[(PLACEMENT_TBL[position] as isize + mv) as usize];

            while position_e != -1 {
                if board.cases[position_e as usize].color == self.color {
                    break;
                }

                if board.cases[position_e as usize].color == opposit_color_case {
                    possibilities_list.push(position_e as usize);
                    break;
                } else if board.cases[position_e as usize].name == EMPTY_PIECE {
                    possibilities_list.push(position_e as usize);
                }

                multiplier += 1;
                position_e =
                    OVERFLOW_TBL[(PLACEMENT_TBL[position] as isize + mv * multiplier) as usize];
            }
        }
        possibilities_list
    }

    //===========================
    // Queeh
    //===========================
    pub fn liste_coups_possibles_dame(&self, position: usize, board: &Board) -> Vec<usize> {
        let mut fou_moves = self.clone().liste_coups_possibles_fou(position, board);
        let mut tour_moves = self.liste_coups_possibles_tour(position, board);
        fou_moves.append(&mut tour_moves);
        fou_moves
    }

    pub fn is_empty(self) -> bool {
        self.name.is_empty()
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece::new("", "")
    }
}
