use crate::game_lib::game::Game;

#[test]
fn t_game() {
    let mut game = Game::init();

    game.board.print_board();

    // Effectuer un mouvement
    if let Ok(_) = game.make_move_algebraic("e2->e4") {
        game.board.print_board();
    }
    println!("turn:{:?}", game.board.turn);
    if let Ok(_) = game.make_move_algebraic("d7->d5") {
        game.board.print_board();
    }
    println!("turn:{:?}", game.board.turn);
    if let Ok(_) = game.make_move_algebraic("e4->d5") {
        game.board.print_board();
    }
    println!("turn:{:?}", game.board.turn);
    if let Ok(_) = game.make_move_algebraic("d8->d5") {
        game.board.print_board();
    }
}
