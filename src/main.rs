mod former;
use crate::former::get_adjacent;
use crate::former::Game;

fn main() {
    let game = Game::new();
    println!("{:?}", game.find_moves());
    println!("{:?}", get_adjacent(10));
}
