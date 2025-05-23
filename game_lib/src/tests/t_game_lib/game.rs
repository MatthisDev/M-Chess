use crate::automation::ai::{Difficulty, AI};
use crate::game::Game;
use crate::piece::Color;
use crate::position::Position;
use std::io;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

// #[test]
fn t_game() { let mut game = Game::init(false);
    let ia = AI::new(Difficulty::Medium, Color::White);

    let mut finish_game: Result<bool, &'static str> = Ok(true);

    println!("Before move");
    game.board.print_board();

    while finish_game == Ok(true)
        || finish_game == Err("Invalid movement.")
        || finish_game == Err("parse_move_str: invalid send string: <{move_piece}>")
    {
        println!("Waiting 1 second for readability...");
        println!("Turn {}: {:?}", game.board.counter, game.board.turn);
        sleep(Duration::from_secs(1));

        let mut move_str = String::new();
        if game.board.turn == Color::White {
            println!("Waiting for IA to make a move...");
            let best_move = match ia.get_best_move(&game.board) {
                Some(mv) => mv,
                None => {
                    println!("No valid moves available for White");
                    break;
                }
            };
            println!("Best move found: {:?}", best_move);

            move_str = format!(
                "{}->{}",
                best_move.0.to_algebraic(),
                best_move.1.to_algebraic()
            );
        } else {
            println!("Enter your movement: <start>-><destination>");
            io::stdin()
                .read_line(&mut move_str)
                .expect("Error reading input");
            println!("Move user: {:?}", move_str);

        }

        finish_game = game.make_move_algebraic(&move_str);
        println!("Move result: {:?}", finish_game);

        game.board.print_board();
    }
}

// #[test]
fn t_game_custom() {
    let mut game = Game::init(true);

    game.board.add_piece("brd6");
    game.board.add_piece("brf6");
    game.board.add_piece("bkd7");
    game.board.add_piece("wke3");

    game.board.print_board();

    let mut finish_game: Result<bool, &'static str> = Ok(true);

    while finish_game == Ok(true)
        || finish_game == Err("Mouvement invalide.")
        || finish_game == Err("parse_move_str: invalid send string: <{move_piece}>")
    {
        let mut move_user = String::new();

        println!("Au {:?} de jouer", game.board.turn);
        println!("Entrez votre movement: <position de depart>-><position d'arrivee>");

        io::stdin()
            .read_line(&mut move_user)
            .expect("Échec de la lecture de l'entrée");

        finish_game = game.make_move_algebraic(&move_user);

        Command::new("clear").status().expect("Ca veut pas clear");
        game.board.print_board();
    }
}

// >=============== movement testing ===============<
// #[cfg(test)]
mod movements_tests {
    use super::*;
    
    #[test]
    fn pawn() {
        let mut game = Game::init(true);

        game.board.add_piece("wkf7");
        game.board.add_piece("bkf0");
        game.board.add_piece("wpc4");
        game.board.add_piece("bpd3");

        assert_eq!(game.make_move_algebraic("c4->d3"), Ok(true));
    }

    #[test]
    fn queen() {
        let mut game = Game::init(true);

        game.board.add_piece("wkf7");
        game.board.add_piece("bkf0");
        game.board.add_piece("wqb6");
        game.board.add_piece("bpf6");
        game.board.add_piece("bpf2");

        assert_eq!(game.make_move_algebraic("b6->f2"), Ok(true));
        assert_eq!(game.make_move_algebraic("f0->e0"), Ok(true));
        assert_eq!(game.make_move_algebraic("f2->f6"), Ok(true));
    }

    #[test]
    fn knight() {
        let mut game = Game::init(true);

        game.board.add_piece("wkf7");
        game.board.add_piece("bkf0");
        game.board.add_piece("wnb6");
        game.board.add_piece("bpa4");
        game.board.add_piece("bpb2");
        game.board.add_piece("bpd3");

        assert_eq!(game.make_move_algebraic("b6->a4"), Ok(true));
        assert_eq!(game.make_move_algebraic("f0->f1"), Ok(true));
        assert_eq!(game.make_move_algebraic("a4->b2"), Ok(true));
        assert_eq!(game.make_move_algebraic("f1->f0"), Ok(true));
        assert_eq!(game.make_move_algebraic("b2->d3"), Ok(true));
    }

    #[test]
    fn rook() {
        let mut game = Game::init(true);

        game.board.add_piece("wkf7");
        game.board.add_piece("bkf0");
        game.board.add_piece("brf2");
        game.board.add_piece("wpa5");

        game.board.turn = Color::Black;
        assert_eq!(game.make_move_algebraic("f2->f7"), Ok(true));
    }

    #[test]
    fn roque() {
        let mut game = Game::init(true);

        game.board.add_piece("wke7");
        game.board.add_piece("bke0");
        game.board.add_piece("wrh7");
        game.board.add_piece("brh0");
        game.board.add_piece("bra0");
        game.board.add_piece("wpb6");


        assert_eq!(
            game.make_move_algebraic("e7->c6"),
            Err("Invalid movement.")
        );
        
        assert_eq!(game.make_move_algebraic("e7->g7"), Ok(true));
        
        assert_eq!(
            game.make_move_algebraic("e0->g0"),
            Err("Invalid movement.")
        );
        assert_eq!(game.make_move_algebraic("e0->c0"), Ok(true));
        
    }

    #[test]
    fn king() {
        let mut game = Game::init(true);

        game.board.add_piece("wkd7");
        game.board.add_piece("bkf0");
        game.board.add_piece("bpc5");

        game.board.print_board();

        assert_eq!(game.make_move_algebraic("d7->c6"), Ok(true));
    }
}
// >=============== get_list_moves ===============<
// #[cfg(test)]
mod list_moves_tests {
    use super::*;

    #[test]
    fn pawn() {
        let mut game = Game::init(false);

        assert_eq!(
            game.get_list_moves("e6".to_string()),
            Ok(vec!["e5".to_string(), "e4".to_string()])
        );
        assert_eq!(
            game.get_list_moves("e5".to_string()),
            Ok(Vec::<String>::new())
        );
    }

    #[test]
    fn king() {
        let mut game = Game::init(true);

        game.board.add_piece("brd2");
        game.board.add_piece("brf2");
        game.board.add_piece("bkd1");
        game.board.add_piece("wke5");

        assert_eq!(
            game.get_list_moves("d1".to_string()),
            Ok(vec![
                "c1".to_string(),
                "e1".to_string(),
                "d0".to_string(),
                "e2".to_string(),
                "c2".to_string(),
                "e0".to_string(),
                "c0".to_string()
            ])
        );

        assert_eq!(
            game.get_list_moves("e5".to_string()),
            Ok(vec!["e4".to_string(), "e6".to_string()])
        );
    }

    #[test]
    fn bishop() {
        let mut game = Game::init(true);

        game.board.add_piece("wkf7");
        game.board.add_piece("bkf0");
        game.board.add_piece("wpd6");
        game.board.add_piece("bpb6");
        game.board.add_piece("wpe3");
        game.board.add_piece("wbc5");

        assert_eq!(
            game.get_list_moves("c5".to_string()),
            Ok(vec![
                "b4".to_string(),
                "a3".to_string(),
                "b6".to_string(),
                "d4".to_string()
            ])
        );
    }

    #[test]
    fn protect_king() {
        let mut game = Game::init(true);

        game.board.add_piece("bkf0");
        game.board.add_piece("wkf5");
        game.board.add_piece("brf2");
        game.board.add_piece("wqd4");
        game.board.add_piece("wpa5");

        assert_eq!(
            game.get_list_moves("d4".to_string()),
            Ok(vec!["f2".to_string(), "f4".to_string()])
        );
        assert_eq!(game.get_list_moves("a3".to_string()), Ok(vec![]));
    }

    #[test]
    fn protect_king2() {
        let mut game = Game::init(true);

        game.board.add_piece("bkf0");
        game.board.add_piece("wkf6");
        game.board.add_piece("wqf5");
        game.board.add_piece("bqf3");

        assert_eq!(
            game.get_list_moves("f5".to_string()),
            Ok(vec!["f4".to_string(), "f3".to_string()])
        );
    }

    #[test]
    fn castle() {
        let mut game = Game::init(true);

        game.board.add_piece("wke7");
        game.board.add_piece("wra7");
        game.board.add_piece("wrf7");
        game.board.add_piece("wpd6");
        game.board.add_piece("wpe6");
        game.board.add_piece("wpf6");

        game.board.add_piece("bke0");
        game.board.add_piece("bpe1");

        assert_eq!(
            game.get_list_moves("e7".to_string()),
            Ok(vec!["d7".to_string(), "c7".to_string()]));
    }

}

// >=============== custom test  ===============<
// #[cfg(test)]
mod custom_test {
    use super::*;
    
    #[test]
    fn t_add_remove() {
        let mut game = Game::init(true);

        // try to add
        assert_eq!(game.board.add_piece("bpe0"), Ok(true));
        assert_eq!(game.board.add_piece("bpe0"), Ok(false)); // there is already a piece there
        assert!(game.board.add_piece("sjfd").is_err());
        // assert_eq!(

        // try to remove
        assert_eq!(game.board.remove_piece("e0"), Ok(true));
        assert_eq!(game.board.remove_piece("e1"), Ok(false)); // nothing there
        assert!(game.board.remove_piece("").is_err());
    }

    // #[test]
    fn t_get_board() {
        let mut game = Game::init(false);

        let board = game.board.get();

        for i in 0..8_usize {
            print!("|");
            for j in 0..8_usize {
                print!("{:?}|", board[i][j]);
            }
            print!("\n");
        }
    }
}

// >=============== Position ===============<
// #[cfg(test)]
mod positions_tests {
    use super::*;

    #[test]
    fn t_from_algebraic() {
        assert_eq!(
            Position::from_algebraic("a0"),
            Ok(Position { row: 0, col: 0 })
        );
        assert_eq!(
            Position::from_algebraic("a3"),
            Ok(Position { row: 3, col: 0 })
        );
    }
}

// >=============== AI  ===============<
// #[cfg(test)]
mod ia_tests{
    use super::*;
    
    //#[test]
    fn t_create() {
        let mut game = Game::init(false);

        let ai = AI::new(Difficulty::Easy, Color::White);
    }

    #[ignore = "only manual launch"]
    // #[test]
    fn t_game() {
        let mut game = Game::init(false);

        let wai = AI::new(Difficulty::Hard, Color::White);
        let bai = AI::new(Difficulty::Medium, Color::Black);
        let mut finish_game: Result<bool, &'static str> = Ok(true);

        println!("Before move");
        game.board.print_board();

        while finish_game == Ok(true)
            || finish_game == Err("Mouvement invalide.")
                || finish_game == Err("parse_move_str: invalid send string: <{move_piece}>")
                {
                    println!("Waiting for 1 second...");
                    println!("Turn {}: {:?}", game.board.counter, game.board.turn);
                    sleep(Duration::from_secs(1));

                    let mut move_str = String::new();
                    if game.board.turn == Color::White {
                        let best_move = match wai.get_best_move(&game.board) {
                            Some(mv) => mv,
                            None => {
                                println!("No valid moves available for White");
                                break;
                            }
                        };
                        println!("Best move: {:?}", best_move);

                        move_str = format!(
                            "{}->{}",
                            best_move.0.to_algebraic(),
                            best_move.1.to_algebraic()
                        );
                    } else {
                        let best_move = match bai.get_best_move(&game.board) {
                            Some(mv) => mv,
                            None => {
                                println!("No valid moves available for Black");
                                break;
                            }
                        };
                        println!("Best move: {:?}", best_move);

                        move_str = format!(
                            "{}->{}",
                            best_move.0.to_algebraic(),
                            best_move.1.to_algebraic()
                        );
                    }

                    if game.board.counter == 29 {
                        println!("Best move: ");
                    }
                    finish_game = game.make_move_algebraic(&move_str);

                    game.board.print_board();
                }

        println!("Game finished: {:?}", finish_game);
    }

    // #[test]
    fn t_game_custom() {
        let mut game = Game::init(true);
        game.board.add_piece("bke0");    
        game.board.add_piece("brh5");    
        game.board.add_piece("bpg5");
        game.board.add_piece("bqf5");    

        game.board.add_piece("wke7");
        game.board.add_piece("wpf6");
        game.board.add_piece("wrh7");

        let ia = AI::new(Difficulty::Medium, Color::White);

        let mut finish_game: Result<bool, &'static str> = Ok(true);

        println!("Before move");
        game.board.print_board();

        while finish_game == Ok(true)
            || finish_game == Err("Invalid movement.")
                || finish_game == Err("parse_move_str: invalid send string: <{move_piece}>")
                {
                    println!("Waiting 1 second for readability...");
                    println!("Turn {}: {:?}", game.board.counter, game.board.turn);
                    sleep(Duration::from_secs(1));

                    let mut move_str = String::new();
                    if game.board.turn == ia.get_color(){
                        println!("Waiting for IA to make a move...");
                        let best_move = match ia.get_best_move(&game.board) {
                            Some(mv) => mv,
                            None => {
                                println!("No valid moves available for White");
                                break;
                            }
                        };
                        println!("Best move found: {:?}", best_move);

                        move_str = format!(
                            "{}->{}",
                            best_move.0.to_algebraic(),
                            best_move.1.to_algebraic()
                        );
                    } else {
                        println!("Enter your movement: <start>-><destination>");
                        io::stdin()
                            .read_line(&mut move_str)
                            .expect("Error reading input");
                        println!("Move user: {:?}", move_str);
                        Command::new("clear").status().expect("Ca veut pas clear");
                    }

                    finish_game = game.make_move_algebraic(&move_str);
                    println!("Move result: {:?}", finish_game);

                    game.board.print_board();
                }
    }
}
