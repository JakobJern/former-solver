use std::vec;

static COLS: usize = 7;
static ROWS: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Form {
    None = 0,
    Blue = 1,
    Green = 2,
    Orange = 3,
    Pink = 4,
}


#[derive(Clone)]
pub struct Game {
    grid: [Form; 63],
    moves_made: u8,
    minimum_moves: u8
}

impl Game {
    pub fn new() -> Game {
        let inp = include_str!("../01-01-2025.txt");
        let mut row = 0;
        let mut grid = [Form::None; 63]; // gamesize
        for l in inp.lines() {
            let mut col = 0;
            for c in l.chars() {
                grid[row * COLS + col] = char_to_form(c);
                col += 1;
            }
            row += 1;
        }
        let mut new_game = Game {
            grid,
            moves_made: 0,
            minimum_moves: 4,
        };
        new_game.minimum_moves = new_game.minimum_moves();
        new_game
    }
    
    
    fn minimum_moves(&self) -> u8 {
        let mut blues = [false; 7];
        let mut greens = [false; 7];
        let mut oranges = [false; 7];
        let mut pinks = [false; 7];
        for col in 0..COLS {
            for row in 0..ROWS {
                match self.grid[row*COLS + col] {
                    Form::Blue => blues[col] = true,
                    Form::Green => greens[col] = true,
                    Form::Orange => oranges[col] = true,
                    Form::Pink => pinks[col] = true,
                    _ => (),
                }
            }
        }
        let mut groups = 0;
        groups += groups_of_color(&blues);
        groups += groups_of_color(&greens);
        groups += groups_of_color(&oranges);
        groups += groups_of_color(&pinks);
        groups
    }
    
    
    fn apply_gravity(&mut self) {
        for index in (0..COLS*ROWS).into_iter().rev() {
            if self.grid[index] == Form::None {
                for i in indices_above(index).into_iter() {
                    if i < 7 {
                        self.grid[i] = Form::None;
                    } else {
                        self.grid[i] = self.grid[i-7];
                    }
                }
            }
        }
    }
    
    fn find_group(&self, index: usize) -> Vec<usize> {
        let group_color = self.grid[index];
        let mut group = Vec::with_capacity(15);
        group.push(index);
        let mut group_index = 0;
        let mut pushed = vec![false; COLS * ROWS];
        while group_index < group.len() {
            for adj_i in adjacent_indices(group[group_index]) {
                if pushed[adj_i] {
                    continue;
                }
                if self.grid[adj_i] == group_color {
                    group.push(adj_i);
                    pushed[adj_i] = true;
                }
            }
            group_index += 1;
        }
        group
    }
    
    pub fn find_moves(&self) -> Vec<usize> {
        let mut seen = vec![false; COLS * ROWS];
        let mut moves = Vec::new();
        for i in 0..COLS * ROWS {
            if seen[i] || self.grid[i] == Form::None {
                continue;
            }
            let group = self.find_group(i);
            for index in group {
                seen[index] = true;
            }
            moves.push(i);
        }
        moves
    }

    pub fn new_game_with_move(&self, index: usize) -> Game {
        let mut game_with_move = self.clone();
        game_with_move.moves_made = self.moves_made + 1;
        let indices_to_delete = game_with_move.find_group(index);
        for i in indices_to_delete {
            game_with_move.grid[i] = Form::None;
        }
        game_with_move.apply_gravity();
        game_with_move.minimum_moves = game_with_move.minimum_moves();
        game_with_move
    }

    pub fn print(&self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                print!("{}", form_to_char(self.grid[COLS*row + col]));
            }
            println!();
        }
        println!();
    }

}


fn char_to_form(c: char) -> Form {
    match c {
        'b' => Form::Blue,
        'g' => Form::Green,
        'o' => Form::Orange,
        'p' => Form::Pink,
        _ => Form::None,
    }
}

fn form_to_char(form: Form) -> char {
    match form {
        Form::Blue => 'b',
        Form::Green => 'g',
        Form::Orange => 'o',
        Form::Pink => 'p',
        Form::None => ' ',
    }
}

fn adjacent_indices(i: usize) -> Vec<usize> {
    let mut adj = Vec::with_capacity(4);
    if i >= COLS {
        adj.push(i - COLS);
    }
    if i < COLS * (ROWS - 1) {
        adj.push(i + COLS);
    }
    if i % COLS != 6 {
        adj.push(i + 1);
    }
    if i % COLS != 0 {
        adj.push(i - 1);
    }
    adj
}

fn indices_above(mut i: usize) -> Vec<usize>{
    let mut above = Vec::with_capacity(8);
    above.push(i);
    while i >= 7 {
        i -= 7;
        above.push(i);
    }
    above
}

fn groups_of_color(color_in_col: &[bool; 7]) -> u8 {
    let mut prev = false;
    let mut groups = 0;
    for current in color_in_col {
        if !prev && *current {
            groups += 1;
        }
        prev = *current;
    }
    groups
}
