mod former;
use crate::former::Game;
use crate::former::Form;
use std::collections::BinaryHeap;
use ahash::AHashSet;
fn main() {

    let game = Game::new();
    let new_game = game.new_game_with_move(6*7 + 4);
    new_game.print();
    let new_game = new_game.new_game_with_move(7*7 + 6);
    new_game.print();

    beam_search(10);

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

fn beam_search(mut beamwidth: usize) {
    let mut best_finish = 99;
    while beamwidth < 500_000 {
        let game = Game::new();
        let mut queue: Vec<Game> = Vec::new();
        queue.push(game);
        let mut applied_moves = 0;
        let mut pushed_grids: AHashSet<[Form; 63]> = AHashSet::new();
        'beam:
        while !queue.is_empty() {
            let current_game = queue.remove(0);
            if current_game.moves_made > applied_moves {
                queue.sort_by(|a, b| a.heuristic().cmp(&b.heuristic()));
                queue = queue.into_iter().take(beamwidth).collect();
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
        beamwidth *= 3;
        println!("Beamwidth now {beamwidth}")
    }


}