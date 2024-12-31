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
    fn _state_id(&self) -> (u128, u128) {
        let mut upper_half = 0u128;
        let mut lower_half = 0u128;
        let mut quad_counter: u32 = 0;
        for row in 0..9 {
            for col in 0..7 {
                let form = &self.grid[row * COLS + col];
                if quad_counter > 31 {
                    lower_half |= form_to_u128(form) << (quad_counter - 32) * 4;
                } else {
                    upper_half |= form_to_u128(form) << quad_counter * 4;
                }
                quad_counter += 1;
            }
        }
        (upper_half, lower_half)
    }
    pub fn new() -> Game {
        let inp = include_str!("../input.txt");
        let mut row = 0;
        let mut grid = [Form::None; 63]; // gamesize
        for l in inp.lines() {
            let mut col = 0;
            for c in l.chars() {
                println!("grid at {} is {:?}", row * COLS + col, char_to_form(c));
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
            let group_color = self.grid[i];
            let mut group = Vec::with_capacity(40);
            group.push(i);
            let mut group_index = 0;
            while group_index < group.len() {
                for adj_i in get_adjacent(group[group_index]) {
                    if seen[adj_i] {
                        continue;
                    }
                    if self.grid[adj_i] == group_color {
                        seen[adj_i] = true;
                        group.push(adj_i);
                    }
                }
                group_index += 1;
            }
            moves.push(i);
        }
        moves
    }
}

fn form_to_u128(form: &Form) -> u128 {
    match form {
        &Form::None => 0,
        &Form::Blue => 1,
        &Form::Green => 2,
        &Form::Orange => 3,
        &Form::Pink => 4,
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

pub fn get_adjacent(i: usize) -> Vec<usize> {
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
