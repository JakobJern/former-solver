mod former;
use crate::former::Game;
use crate::former::Form;
use std::collections::BinaryHeap;
use ahash::AHashSet;
fn main() {
    let game = Game::new();

    let mut queue: BinaryHeap<Game> = BinaryHeap::new();
    queue.push(game);

    let mut seen_grids: AHashSet<[Form; 63]> = AHashSet::new();

    let mut checking_all_potential_xs = 0;
    'outer:
    while !queue.is_empty() {
        let current_game = queue.pop().unwrap();
        for game_move in current_game.find_moves() {
            let new_game = current_game.new_game_with_move(game_move);
            if new_game.is_done() {
                new_game.print_move_list();
                break 'outer;
            }
            if !seen_grids.contains(&new_game.get_grid()) {
                seen_grids.insert(new_game.get_grid());
                queue.push(new_game);
            }
        }
        if checking_all_potential_xs < current_game.get_score() {
            checking_all_potential_xs = current_game.get_score();
            println!("Checked first potential {checking_all_potential_xs}");
        }
    }

}
