mod former;
use crate::former::Game;

fn main() {
    let mut game = Game::new();

    println!("{}", game.find_moves().len());
    game.print();
    let game_with_move = game.new_game_with_move(1);
    game_with_move.print();

}
