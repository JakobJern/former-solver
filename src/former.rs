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

fn minimum_moves() -> u8 {
    0
}

pub struct Game {
    grid: [Form; 63],
    moves_made: u8,
}

impl Game {
    pub fn new() -> Game {
        let inp = include_str!("../31-12-2024.txt");
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
        Game {
            grid,
            moves_made: 0,
        }
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
    pub fn apply_move(&mut self, index: usize) {
        self.moves_made += 1;
        let indices_to_delete = self.find_group(index);
        for i in indices_to_delete {
            self.grid[i] = Form::None;
        }
        self.apply_gravity();
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
