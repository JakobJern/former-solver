use std::vec;

static COLS: usize = 7;
static ROWS: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Form {
    None = 0,
    Blue = 1,
    Green = 2,
    Orange = 3,
    Pink = 4,
}


#[derive(Clone, PartialEq, Eq)]
pub struct Game {
    grid: [Form; 63],
    pub moves_made: u8,
    minimum_additional_moves: u8,
    move_list: Vec<u8>
}

impl Game {
    pub fn new() -> Game {
        let inp = include_str!("../04-01-2025.txt");
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
            minimum_additional_moves: 4,
            move_list: Vec::new()
        };
        new_game.minimum_additional_moves = new_game.minimum_moves();
        new_game
    }
    
    pub fn heuristic(&self) -> u8 {
        self.find_moves().len() as u8 + self.minimum_additional_moves
    }
    
    fn estimated_moves(&self) -> u8 {
        (self.find_moves().len()/4) as u8
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

    
    fn apply_gravity(&mut self, deleted_indices: &mut Vec<usize>) {
        deleted_indices.sort();
        for index in deleted_indices {
            if self.grid[*index] == Form::None {
                for i in indices_above(*index) {
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

    pub fn is_done(&self) -> bool {
        for i in ROWS*(COLS-1)..ROWS*COLS {
            if self.grid[i] != Form::None {
                return false
            }
        }
        true
    }

    pub fn new_game_with_move(&self, index: usize) -> Game {
        let mut game_with_move = self.clone();
        game_with_move.moves_made += 1;
        game_with_move.move_list.push(index as u8);
        let mut indices_to_delete = game_with_move.find_group(index);
        for &i in &indices_to_delete {
            game_with_move.grid[i] = Form::None;
        }
        game_with_move.apply_gravity(&mut indices_to_delete);
        game_with_move.minimum_additional_moves = game_with_move.minimum_moves();
        game_with_move
    }

    pub fn get_score(&self) -> u8 {
        self.moves_made + self.minimum_additional_moves
    }

    pub fn get_grid(&self) -> [Form; 63] {
        self.grid
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

    pub fn print_move_list(&self) {
        for m in &self.move_list {
            println!("Row: {}, Col: {}", m/7, m%7);
        }
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
    while i >= COLS {
        i -= COLS;
        above.push(i);
    }
    above
}

fn groups_of_color(color_in_col: &[bool; 7]) -> u8 {
    let mut prev = false;
    let mut groups = 0;
    for &current in color_in_col {
        if !prev && current {
            groups += 1;
        }
        prev = current;
    }
    groups
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let score = self.get_score();
        let other_score = other.get_score();
        if score < other_score {
            return Some(std::cmp::Ordering::Greater);
        } else if score > other_score {
            return Some(std::cmp::Ordering::Less);
        } else {
            if self.moves_made > other.moves_made {
                return Some(std::cmp::Ordering::Greater);
            }
            return Some(std::cmp::Ordering::Equal);
        }
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}