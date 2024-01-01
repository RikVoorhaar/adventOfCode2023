use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{collections::HashSet, fmt::Debug};

use anyhow::Result;

#[derive(Eq, PartialEq, Clone)]
struct Table {
    width: usize,
    height: usize,
    square_rocks: HashSet<(usize, usize)>,
    round_rocks: HashSet<(usize, usize)>,
}

impl Hash for Table {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut rocks_sorted: Vec<&(usize, usize)> = self.square_rocks.iter().collect();
        rocks_sorted.sort();
        rocks_sorted.hash(state);

        let mut rocks_sorted: Vec<&(usize, usize)> = self.round_rocks.iter().collect();
        rocks_sorted.sort();
        rocks_sorted.hash(state);
    }
}

impl Table {
    fn compute_load(&self) -> usize {
        self.round_rocks.iter().map(|(_, y)| self.height - y).sum()
    }
    fn move_up(&mut self) {
        for x in 0..self.width {
            let mut target_y = 0;
            for y in 0..self.height {
                if self.square_rocks.contains(&(x, y)) {
                    target_y = y + 1;
                }
                if self.round_rocks.remove(&(x, y)) {
                    self.round_rocks.insert((x, target_y));
                    target_y += 1;
                }
            }
        }
    }
    fn move_down(&mut self) {
        for x in 0..self.width {
            let mut target_y = self.height - 1;
            for y in (0..self.height).rev() {
                if self.square_rocks.contains(&(x, y)) {
                    target_y = y.saturating_sub(1);
                }
                if self.round_rocks.remove(&(x, y)) {
                    self.round_rocks.insert((x, target_y));
                    target_y = target_y.saturating_sub(1);
                }
            }
        }
    }
    fn move_right(&mut self) {
        for y in 0..self.height {
            let mut target_x = self.width - 1;
            for x in (0..self.width).rev() {
                if self.square_rocks.contains(&(x, y)) {
                    target_x = x.saturating_sub(1);
                }
                if self.round_rocks.remove(&(x, y)) {
                    self.round_rocks.insert((target_x, y));
                    target_x = target_x.saturating_sub(1);
                }
            }
        }
    }
    fn move_left(&mut self) {
        for y in 0..self.height {
            let mut target_x = 0;
            for x in 0..self.width {
                if self.square_rocks.contains(&(x, y)) {
                    target_x = x + 1;
                }
                if self.round_rocks.remove(&(x, y)) {
                    self.round_rocks.insert((target_x, y));
                    target_x += 1;
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.move_up();
        self.move_left();
        self.move_down();
        self.move_right();
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                if self.square_rocks.contains(&(col, row)) {
                    out.push('#');
                } else if self.round_rocks.contains(&(col, row)) {
                    out.push('O');
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

fn parse_input(input: &str) -> Table {
    let width = input.lines().next().unwrap().len();
    let mut square_rocks = HashSet::new();
    let mut round_rocks = HashSet::new();
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height += 1;
        for (x, char) in line.chars().enumerate() {
            match char {
                'O' => {
                    round_rocks.insert((x, y));
                }
                '.' => {}
                '#' => {
                    square_rocks.insert((x, y));
                }
                _ => panic!("Invalid char {}", char),
            }
        }
    }

    Table {
        width,
        height,
        square_rocks,
        round_rocks,
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day14/src/input.txt")?;
    let mut table = parse_input(&input);
    let mut found_states: HashSet<Table> = HashSet::new();
    let mut state_map: HashMap<usize, Table> = HashMap::new();
    let mut counter = 0;
    while !found_states.contains(&table) {
        found_states.insert(table.clone());
        state_map.insert(counter, table.clone());
        table.cycle();
        counter += 1
    }
    println!("Found loop at {}", counter);
    let mut loop_length = 0;
    let mut loop_start = 0;
    for i in 0..counter {
        if state_map[&i] == table {
            println!("Loop starts at {}", i);
            loop_length = counter - i;
            loop_start = i;
            break;
        }
    }
    let target_value = 1_000_000_000;
    let mod_value = (target_value -loop_start)% loop_length +loop_start;
    let table = &state_map[&mod_value];
    println!("Mod value {}", mod_value);
    println!("Load: {}", table.compute_load());

    Ok(())
}
