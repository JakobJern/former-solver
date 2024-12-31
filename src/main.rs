mod former;
use crate::former::Game;

fn main() {
    let mut game = Game::new();

    println!("{}", game.find_moves().len());
    game.print();
    game.apply_move(1);
    game.print();

}
