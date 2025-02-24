use crate::game_lib::game::Game;
use std::io;
use std::process::Command;


#[test]
fn t_game() {
    let mut game = Game::init();

    game.board.print_board();
    /* 
    let mut finish_game: Result<bool, &'static str> = Ok(true);

    while finish_game == Ok(true) {

        let mut move_user = String::new();

        println!("Au {:?} de jouer", game.board.turn);
        println!("Entrez votre movement: <position de depart>-><position d'arrivee>");

        io::stdin()
                .read_line(&mut move_user)
                .expect("Échec de la lecture de l'entrée");

        finish_game = game.make_move_algebraic(&move_user[0..6]);
        
        Command::new("clear").status().expect("Ca veut pas clear");
        game.board.print_board();
    } */

    
    // Effectuer un mouvement
    if game.make_move_algebraic("e2->e4").is_ok() {
        game.board.print_board();
    }
    println!("turn:{:?}", game.board.turn);
    if game.make_move_algebraic("d7->d5").is_ok() {
        game.board.print_board();
    }
    println!("turn:{:?}", game.board.turn);
    if game.make_move_algebraic("e4->d5").is_ok() {
        game.board.print_board();
    }
    println!("turn:{:?}", game.board.turn);
    if game.make_move_algebraic("d8->d5").is_ok() {
        game.board.print_board();
    }
    
}
