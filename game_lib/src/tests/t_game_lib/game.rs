use crate::automation::ai::{Difficulty, AI};
use crate::game::Game;
use crate::position::Position;
use crate::piece::Color;
use std::io;
use std::process::Command;


// #[test]
fn t_game() {
    let mut game = Game::init(false);

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
#[test]
fn t_eat_pawn() {
    let mut game = Game::init(true);

    game.board.add_piece("wkf1");
    game.board.add_piece("bkf8");
    game.board.add_piece("wpc4");
    game.board.add_piece("bpd5");

    // assert_eq!(game.get_list_moves("c4".to_string()), 
               //Ok(vec!["c5".to_string(), "d5".to_string()]));
    
    assert_eq!(game.make_move_algebraic("c4->d5"), Ok(true));

}

#[test]
fn t_eat_queen() {
    let mut game = Game::init(true);

    game.board.add_piece("wkf1");
    game.board.add_piece("bkf8");
    game.board.add_piece("wqb2");
    game.board.add_piece("bpf2");
    game.board.add_piece("bpf6");

    assert_eq!(game.make_move_algebraic("b2->f6"), Ok(true));
    assert_eq!(game.make_move_algebraic("f8->e8"), Ok(true));
    assert_eq!(game.make_move_algebraic("f6->f2"), Ok(true));

}

#[test]
fn t_eat_knight() {

    let mut game = Game::init(true);

    game.board.add_piece("wkf1");
    game.board.add_piece("bkf8");
    game.board.add_piece("wnb2");
    game.board.add_piece("bpa4");
    game.board.add_piece("bpb6");
    game.board.add_piece("bpd5");
    
    assert_eq!(game.make_move_algebraic("b2->a4"), Ok(true));
    assert_eq!(game.make_move_algebraic("f8->f7"), Ok(true));
    assert_eq!(game.make_move_algebraic("a4->b6"), Ok(true));
    assert_eq!(game.make_move_algebraic("f7->f8"), Ok(true));
    assert_eq!(game.make_move_algebraic("b6->d5"), Ok(true));

}

#[test] 
fn t_eat_rook() {

    let mut game = Game::init(true);

    game.board.add_piece("wkf1");
    game.board.add_piece("bkf8");
    game.board.add_piece("brf6");
    game.board.add_piece("wpa3");
    
    game.board.turn = Color::Black;
    assert_eq!(game.make_move_algebraic("f6->f1"), Ok(true));

}

// >=============== get_list_moves ===============<

#[test]
fn t_get_list_moves_pawn() {
    let mut game = Game::init(false);


    assert_eq!(game.get_list_moves("e2".to_string()), 
               Ok(vec!["e3".to_string(), "e4".to_string()]));
    assert_eq!(game.get_list_moves("e3".to_string()),
               Ok(Vec::<String>::new()));

}

#[test]
fn t_get_list_moves_king() {

    let mut game = Game::init(true);

    game.board.add_piece("brd6");
    game.board.add_piece("brf6");
    game.board.add_piece("bkd7");
    game.board.add_piece("wke3");

    assert_eq!(game.get_list_moves("d7".to_string()), Ok(vec!["c7".to_string(),
            "e7".to_string(), "d8".to_string(), "e6".to_string(), 
            "c6".to_string(), "e8".to_string(), "c8".to_string()]));
    
    assert_eq!(game.get_list_moves("e3".to_string()),
            Ok(vec!["e4".to_string(), "e2".to_string()]));
}


#[test]
fn t_get_list_moves_bishop() {
    let mut game = Game::init(true);
    
    game.board.add_piece("wkf1");
    game.board.add_piece("bkf8");
    game.board.add_piece("wpd2");
    game.board.add_piece("bpb2");
    game.board.add_piece("wpe5");
    game.board.add_piece("wbc3");

    assert_eq!(game.get_list_moves("c3".to_string()), Ok(vec![
                "b4".to_string(), "a5".to_string(), 
                "b2".to_string(), "d4".to_string()
    ]));

}

#[test]
fn t_get_list_moves_protect_king() {
    
    let mut game = Game::init(true);

    
    game.board.add_piece("bkf8");
    game.board.add_piece("wkf3");
    game.board.add_piece("brf6");
    game.board.add_piece("wqd4");
    game.board.add_piece("wpa3");

    assert_eq!(game.get_list_moves("d4".to_string()), Ok(vec![
               "f6".to_string(), "f4".to_string()]));
    assert_eq!(game.get_list_moves("a3".to_string()), Ok(vec![]));

}


// >=============== custom test  ===============<

#[test]
fn t_game_custom_add_remove() {
    let mut game = Game::init(true);

    // try to add
    assert_eq!(game.board.add_piece("bpe1"), Ok(true));
    assert_eq!(game.board.add_piece("bpe1"), Ok(false)); // there is already a piece there
    assert_eq!(game.board.add_piece("sjfd").is_err(), true);
    // assert_eq!(

    // try to remove
    assert_eq!(game.board.remove_piece("e1"), Ok(true));
    assert_eq!(game.board.remove_piece("e2"), Ok(false)); // nothing there
    assert_eq!(game.board.remove_piece("").is_err(), true);
}

// #[test]
fn t_game_get_board() {
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

// >=============== Position ===============<
#[test]
fn t_position_from_algebraic() {
    assert_eq!(Position::from_algebraic("a1"),Ok(Position{row: 7, col: 0}));
    assert_eq!(Position::from_algebraic("a3"),Ok(Position{row: 5, col: 0}));
}

// >=============== AI  ===============<
/*
// #[test]
fn t_create_ai() {
    let mut game = Game::init(false);

    let ai = AI::new(Difficulty::Easy, Color::White);
}
// #[test]
fn t_game_ai() {
    let mut game = Game::init(false);

    let wai = AI::new(Difficulty::Hard, Color::White);
    let bai = AI::new(Difficulty::Hard, Color::Black);
    let mut finish_game: Result<bool, &'static str> = Ok(true);

    println!("Before move");
    game.board.print_board();

    while finish_game == Ok(true)
        || finish_game == Err("Mouvement invalide.")
        || finish_game == Err("parse_move_str: invalid send string: <{move_piece}>")
    {
        println!("Au {:?} de jouer", game.board.turn);
        let mut move_str = String::new();
        if game.board.turn == Color::White {
            let best_move = wai.get_best_move(&game.board);
            println!("Best move: {:?}", best_move);

            move_str = format!(
                "{}->{}",
                best_move.0.to_algebraic(),
                best_move.1.to_algebraic()
            );
        } else {
            let best_move = bai.get_best_move(&game.board);
            println!("Best move: {:?}", best_move);

            move_str = format!(
                "{}->{}",
                best_move.0.to_algebraic(),
                best_move.1.to_algebraic()
            );
        }

        finish_game = game.make_move_algebraic(&move_str);

        Command::new("clear").status().expect("Ca veut pas clear");
        game.board.print_board();
    }

    println!("Game finished: {:?}", finish_game);
}

*/
