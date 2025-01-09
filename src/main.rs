mod former;
use crate::former::Game;
use crate::former::Form;
use std::collections::BinaryHeap;
use ahash::AHashSet;
fn main() {

    beam_search();
    // a_star();

}

fn a_star() {
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

fn beam_search() {
    let beamwidths = [100, 10_000, 100_000, 200_000];
    let mut best_finish = 99;
    for beamwidth in beamwidths {
        println!("Beamwidth now {beamwidth}");
        let game = Game::new();
        let mut queue: Vec<Game> = Vec::new();
        queue.push(game);
        let mut applied_moves = 0;
        let mut pushed_grids: AHashSet<[Form; 63]> = AHashSet::new();
        'beam:
        while !queue.is_empty() {
            let current_game = queue.remove(0);
            if current_game.moves_made > applied_moves {
                let length = queue.len();
                queue = (queue
                    .select_nth_unstable_by(beamwidth.clamp(0, length-1), |a, b| a.heuristic().cmp(&b.heuristic()))).0
                    .to_vec();
                applied_moves = current_game.moves_made;
            }
            for game_move in current_game.find_moves() {
                let new_game = current_game.new_game_with_move(game_move);
                if new_game.is_done() && new_game.get_score() < best_finish {
                    best_finish = new_game.get_score();
                    println!("New best score of {best_finish}");
                    new_game.print_move_list();
                    break 'beam;
                }
                if !pushed_grids.contains(&new_game.get_grid()) && new_game.get_score() < best_finish {
                    pushed_grids.insert(new_game.get_grid());
                    queue.push(new_game);
                }
            }
        }
    }


}