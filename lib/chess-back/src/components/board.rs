use crate::components::piece::Piece;

const COORD: [&str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

#[derive(Debug, Clone)]
pub struct Board {
    pub cases: [Piece; 64],
    pub has_trait: String,
    pub en_passant: String,
    pub nb_coups: u32,
    pub history: Vec<String>,
    pub is_roque: [bool; 4],
    pub is_check: (bool, bool),
}

impl Board {
    pub fn init() -> Board {
        Board {
            cases: [
                //A
                Piece::new("Rook", "black"),
                Piece::new("Knight", "black"),
                Piece::new("Bishop", "black"),
                Piece::new("Queen", "black"),
                Piece::new("King", "black"),
                Piece::new("Bishop", "black"),
                Piece::new("Knight", "black"),
                Piece::new("Rook", "black"),
                //B
                Piece::new("Pawn", "black"),
                Piece::new("Pawn", "black"),
                Piece::new("Pawn", "black"),
                Piece::new("Pawn", "black"),
                Piece::new("Pawn", "black"),
                Piece::new("Pawn", "black"),
                Piece::new("Pawn", "black"),
                Piece::new("Pawn", "black"),
                //C
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                //D
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                //E
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                //F
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                Piece::default(),
                //G
                Piece::new("Pawn", "blanc"),
                Piece::new("Pawn", "blanc"),
                Piece::new("Pawn", "blanc"),
                Piece::new("Pawn", "blanc"),
                Piece::new("Pawn", "blanc"),
                Piece::new("Pawn", "blanc"),
                Piece::new("Pawn", "blanc"),
                Piece::new("Pawn", "blanc"),
                //H
                Piece::new("Rook", "blanc"),
                Piece::new("Knight", "blanc"),
                Piece::new("Bishop", "blanc"),
                Piece::new("Queen", "blanc"),
                Piece::new("King", "blanc"),
                Piece::new("Bishop", "blanc"),
                Piece::new("Knight", "blanc"),
                Piece::new("Rook", "blanc"),
            ],

            has_trait: String::from("blanc"),
            en_passant: String::from(""),
            nb_coups: 0,
            history: vec![],
            is_roque: [false, false, false, false],
            is_check: (false, false),
        }
    }

    pub fn get_piece(self, index: usize) -> Piece {
        self.cases[index].clone()
    }

    pub fn coordinate() -> Vec<String> {
        let mut coo = Vec::new();
        for i in 1..=9 {
            for letter in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'].iter() {
                coo.push(format!("{}{}", letter, i));
            }
        }
        coo
    }

    //==============================================================================
    // Display
    //==============================================================================

    // display on terminal
    pub fn display(&self) {
        let letters: String = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
            .iter()
            .map(|&el| format!("  {}  ", el))
            .collect::<Vec<String>>()
            .concat();

        let interline: String = (0..8).map(|_| "---- ").collect::<Vec<&str>>().concat();

        println!("{:>60}/", "");
        println!("{}", "— -".repeat(15));
        println!("{:>60}\\", "");

        println!("    {}", letters);
        println!("    {}", interline);

        let mut num_line = 8;

        for (index_pos, piece) in self.cases.iter().enumerate() {
            //print line
            if index_pos % 8 == 0 {
                print!("{}  |", num_line);
            }

            if !piece.name.is_empty() {
                print!(" {} ", piece.display_name);
            } else {
                print!("    ")
            }

            if (index_pos + 1) % 8 == 0 {
                println!("  {}", num_line);
                println!("    {}", interline);
                num_line -= 1;
            }
        }

        println!("   {}", letters)
    }

    // Show possible actions to the player
    pub fn display_possible_actions(&self, nom_case: &str) {
        let letters: String = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
            .iter()
            .map(|&element| format!("  {}  ", element))
            .collect::<Vec<String>>()
            .concat();

        let interlines: String = (0..8).map(|_| "---- ").collect::<Vec<&str>>().concat();

        let possible_actions = self.possible_actions_list(nom_case);

        let color = self.cases[self.casename_to_index(nom_case)].color;
        let ennemy_color = if color == "blanc" { "black" } else { "blanc" };

        println!("{:>60}/", "");
        println!("{}", "— -".repeat(15));
        println!("{:>60}\\", "");

        println!("   {}", letters);
        println!("    {}", interlines);

        let mut n_line = 8;

        for (i_position, piece) in self.cases.iter().enumerate() {
            if i_position % 8 == 0 {
                print!("{}  |", n_line);
            }

            if i_position == self.casename_to_index(nom_case) {
                print!("#{}#", piece.display_name);
            } else if possible_actions.contains(&i_position) {
                if piece.color == ennemy_color {
                    print!("<{}>", piece.display_name);
                } else {
                    print!(" <> ");
                }
            } else if !piece.name.is_empty() {
                print!(" {} ", piece.display_name);
            } else {
                print!("    ");
            }

            if (i_position + 1) % 8 == 0 {
                println!("  {}", n_line);
                println!("    {}", interlines);
                n_line -= 1;
            }
        }

        println!("   {}", letters);
    }

    //==============================================================================
    // Moves
    //==============================================================================

    pub fn move_piece(&mut self, nom_case_depart: &str, nom_case_arrivee: &str) {
        let start_i = self.casename_to_index(nom_case_depart);
        let end_i = self.casename_to_index(nom_case_arrivee);

        // Black short castling
        if start_i == 4
            && end_i == 6
            && self.cases[4].name == "King"
            && self.actions_with_black_check_list(4).contains(&end_i)
        {
            self.cases[end_i] = self.cases[start_i].clone();
            self.cases[start_i] = Piece::default();
            self.cases[5] = self.cases[7].clone();
            self.cases[7] = Piece::default();
        }

        // White short castling
        if start_i == 60
            && end_i == 62
            && self.cases[60].name == "King"
            && self.actions_with_white_check_list(60).contains(&end_i)
        {
            self.cases[end_i] = self.cases[start_i].clone();
            self.cases[start_i] = Piece::default();
            self.cases[61] = self.cases[63].clone();
            self.cases[63] = Piece::default();
        }

        // Black long castling
        if start_i == 4
            && end_i == 2
            && self.cases[4].name == "King"
            && self.actions_with_black_check_list(4).contains(&end_i)
        {
            self.cases[end_i] = self.cases[start_i].clone();
            self.cases[start_i] = Piece::default();
            self.cases[3] = self.cases[0].clone();
            self.cases[0] = Piece::default();
        }

        // White long castling
        if start_i == 60
            && end_i == 58
            && self.cases[60].name == "King"
            && self.actions_with_white_check_list(60).contains(&end_i)
        {
            self.cases[end_i] = self.cases[start_i].clone();
            self.cases[start_i] = Piece::default();
            self.cases[59] = self.cases[56].clone();
            self.cases[56] = Piece::default();
        }

        if !self.cases[start_i].moved {
            self.cases[start_i].moved = true;
        }

        if self.actions_with_white_check_list(start_i).contains(&end_i)
            && self.cases[start_i].color == "blanc"
        {
            self.cases[end_i] = self.cases[start_i].clone();
            self.cases[start_i] = Piece::default();
        }

        if self.actions_with_black_check_list(start_i).contains(&end_i)
            && self.cases[start_i].color == "black"
        {
            self.cases[end_i] = self.cases[start_i].clone();
            self.cases[start_i] = Piece::default();
        }
    }

    pub fn move_piece_to_index(&mut self, starting_i: usize, ending_i: usize) {
        if !self.cases[starting_i].moved {
            self.cases[starting_i].moved = true;
        }
        if self
            .possible_actions_list(self.index_to_casename(starting_i).as_str())
            .contains(&ending_i)
        {
            self.cases[ending_i] = self.cases[starting_i].clone();
            self.cases[starting_i] = Piece::default();
        }
    }

    pub fn mandatory_move(&mut self, starting_i: usize, ending_i: usize) {
        if !self.cases[starting_i].moved {
            self.cases[starting_i].moved = true;
        }
        self.cases[ending_i] = self.cases[starting_i].clone();
        self.cases[starting_i] = Piece::default();
    }

    pub fn actions_with_white_check_list(&self, index: usize) -> Vec<usize> {
        let possible_moves_list = self.possible_moves_with_index(index);
        let mut to_remove_movelist = Vec::new();

        let mut temporary_board = self.clone();

        let previous_position: Vec<(&str, &str, bool)> = self
            .cases
            .iter()
            .map(|piece| (piece.name, piece.color, piece.moved))
            .collect();

        for mouvement in possible_moves_list.iter() {
            temporary_board.mandatory_move(index, *mouvement);
            if temporary_board.is_echec_blanc() {
                to_remove_movelist.push(*mouvement);
            }
            temporary_board.cases = previous_position
                .iter()
                .map(|(name, color, moved)| Piece {
                    name,
                    color,
                    ennemy_color: if *color == "blanc" { "black" } else { "blanc" },
                    value: 0,
                    moved: *moved,
                    display_name: format!(
                        "{}{}",
                        name.chars().next().unwrap().to_uppercase(),
                        color.chars().next().unwrap().to_lowercase()
                    ),
                })
                .collect::<Vec<Piece>>()
                .try_into()
                .unwrap();
        }

        possible_moves_list
            .into_iter()
            .filter(|index| !to_remove_movelist.contains(index))
            .collect()
    }

    pub fn actions_with_black_check_list(&self, index: usize) -> Vec<usize> {
        let possible_moves_list = self.possible_moves_with_index(index);
        let mut to_remove_movelist = Vec::new();

        let mut temporary_board = self.clone();

        let previous_position: Vec<(&str, &str, bool)> = self
            .cases
            .iter()
            .map(|piece| (piece.name, piece.color, piece.moved))
            .collect();

        for mouvement in possible_moves_list.iter() {
            temporary_board.mandatory_move(index, *mouvement);
            if temporary_board.is_echec_noir() {
                to_remove_movelist.push(*mouvement);
            }
            temporary_board.cases = previous_position
                .iter()
                .map(|(name, color, moved)| Piece {
                    name,
                    color,
                    ennemy_color: if *color == "blanc" { "black" } else { "blanc" },
                    value: 0,
                    moved: *moved,
                    display_name: format!(
                        "{}{}",
                        name.chars().next().unwrap().to_uppercase(),
                        color.chars().next().unwrap().to_lowercase()
                    ),
                })
                .collect::<Vec<Piece>>()
                .try_into()
                .unwrap();
        }

        possible_moves_list
            .into_iter()
            .filter(|index| !to_remove_movelist.contains(index))
            .collect()
    }

    pub fn checked_moves_list(&self, index: usize, color: &str) -> Vec<String> {
        if color == "black" {
            self.actions_with_black_check_list(index)
                .iter()
                .map(|&k| self.index_to_casename(k))
                .collect()
        } else {
            self.actions_with_white_check_list(index)
                .iter()
                .map(|&k| self.index_to_casename(k))
                .collect()
        }
    }

    pub fn piece_list_able_tomove_case_fmt_checked(&self, color: &str) -> Vec<String> {
        self.piece_list_able_tomove_case_fmt(color)
            .into_iter()
            .filter(|element| {
                !self
                    .checked_moves_list(self.casename_to_index(element), color)
                    .is_empty()
            })
            .collect()
    }

    pub fn possible_actions_list(&self, nom_case: &str) -> Vec<usize> {
        let index_case = self.casename_to_index(nom_case);
        self.possible_moves_with_index(index_case)
    }

    pub fn possible_moves_with_index(&self, index_case: usize) -> Vec<usize> {
        match self.cases[index_case].name {
            "" => vec![],
            "Pawn" => self.cases[index_case].liste_coups_possibles_pion(index_case, self),
            "Rook" => self.cases[index_case].liste_coups_possibles_tour(index_case, self),
            "Bishop" => self.cases[index_case].liste_coups_possibles_fou(index_case, self),
            "Knight" => self.cases[index_case].liste_coups_possibles_cavalier(index_case, self),
            "Queen" => self.cases[index_case].liste_coups_possibles_dame(index_case, self),
            "King" => self.cases[index_case].liste_coups_possibles_roi(index_case, self),
            _ => vec![],
        }
    }

    pub fn possible_move_list_case_fmt(&self, nom_case: &str) -> Vec<String> {
        self.possible_actions_list(nom_case)
            .into_iter()
            .map(|index| self.index_to_casename(index))
            .collect()
    }

    pub fn piece_list_canmove(&self, color: &str) -> Vec<usize> {
        (0..64)
            .filter(|&index| {
                self.cases[index].color == color
                    && !self
                        .possible_actions_list(self.index_to_casename(index).as_str())
                        .is_empty()
            })
            .collect()
    }

    pub fn piece_list_able_tomove_case_fmt(&self, color: &str) -> Vec<String> {
        self.piece_list_canmove(color)
            .into_iter()
            .map(|index| self.index_to_casename(index))
            .collect()
    }

    pub fn is_echec_blanc(&self) -> bool {
        let roi_position = self
            .cases
            .iter()
            .position(|piece| piece.name == "King" && piece.color == "blanc")
            .unwrap();
        self.nb_ennemy_pieces_eating_king(roi_position) != 0
    }

    pub fn is_echec_noir(&self) -> bool {
        let roi_position = self
            .cases
            .iter()
            .position(|piece| piece.name == "King" && piece.color == "black")
            .unwrap();
        self.nb_ennemy_pieces_eating_king(roi_position) != 0
    }

    pub fn is_checkmate_white(&self) -> bool {
        (0..64).all(|i| {
            self.cases[i].color != "blanc" || self.actions_with_white_check_list(i).is_empty()
        })
    }

    pub fn is_checkmate_black(&self) -> bool {
        (0..64).all(|i| {
            self.cases[i].color != "black" || self.actions_with_black_check_list(i).is_empty()
        })
    }

    pub fn color_checkmate(&self) -> Option<&str> {
        if self.is_checkmate_white() {
            Some("black")
        } else if self.is_checkmate_black() {
            Some("blanc")
        } else {
            None
        }
    }

    pub fn casename_to_index(&self, nom_case: &str) -> usize {
        let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let col = letters
            .iter()
            .position(|&c| c == nom_case.chars().next().unwrap())
            .unwrap()
            + 1;
        let lig = 8 - nom_case.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
        lig * 8 + col - 1
    }

    pub fn index_to_casename(&self, index_case: usize) -> String {
        let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        format!("{}{}", letters[index_case % 8], 8 - index_case / 8)
    }

    pub fn index_list_eating_piece(&self, index_piece: usize) -> Vec<usize> {
        let mut liste_index = Vec::new();
        let couleur_opposee = self.cases[index_piece].ennemy_color;

        for starting_i in self.piece_list_canmove(couleur_opposee) {
            for ending_i in self.possible_moves_with_index(starting_i) {
                if ending_i == index_piece {
                    liste_index.push(starting_i);
                    break;
                }
            }
        }

        liste_index
    }

    pub fn move_and_eat(&self, ending_i: usize) -> bool {
        !self.cases[ending_i].color.is_empty()
    }

    pub fn nb_ennemy_pieces_eating_king(&self, index: usize) -> usize {
        self.index_list_eating_piece(index).len()
    }

    pub fn little_black_castling_possible(&self) -> bool {
        !self.cases[7].moved
            && !self.cases[4].moved
            && self.cases[6].name.is_empty()
            && self.cases[5].name.is_empty()
    }

    pub fn long_black_castling_possible(&self) -> bool {
        !self.cases[0].moved
            && !self.cases[4].moved
            && self.cases[1].name.is_empty()
            && self.cases[2].name.is_empty()
            && self.cases[3].name.is_empty()
    }

    pub fn little_white_castling_possible(&self) -> bool {
        !self.cases[60].moved
            && !self.cases[63].moved
            && self.cases[61].name.is_empty()
            && self.cases[62].name.is_empty()
    }

    pub fn long_white_castling_possible(&self) -> bool {
        !self.cases[60].moved
            && !self.cases[56].moved
            && self.cases[59].name.is_empty()
            && self.cases[58].name.is_empty()
            && self.cases[57].name.is_empty()
    }

    pub fn point_multiplier_if_can_eat(&self, ending_i: usize) -> f64 {
        let nb_piece_pouvant_manger = self.nb_ennemy_pieces_eating_king(ending_i);

        match nb_piece_pouvant_manger {
            0 => 1.0,
            1 => 1.5,
            _ => 1.5 + 0.2 * (nb_piece_pouvant_manger as f64 - 1.0),
        }
    }
}
