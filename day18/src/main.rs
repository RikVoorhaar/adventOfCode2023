use anyhow::Result;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]

enum Direction {
    R,
    L,
    U,
    D,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: usize,
    color: String,
}

impl Instruction {
    fn from_string(s: &str) -> Option<Self> {
        let mut split = s.split(' ');
        let direction = match split.next()? {
            "R" => Direction::R,
            "L" => Direction::L,
            "U" => Direction::U,
            "D" => Direction::D,
            _ => return None,
        };

        let steps = split.next()?.parse::<usize>().ok()?;
        let color = split.next()?;
        let color = color[1..color.len() - 1].to_string();

        Some(Self {
            direction,
            steps,
            color,
        })
    }

    fn move_in_direction(&self, x: i32, y: i32) -> (i32, i32) {
        match self.direction {
            Direction::R => (x + self.steps as i32, y),
            Direction::L => (x - self.steps as i32, y),
            Direction::U => (x, y - self.steps as i32),
            Direction::D => (x, y + self.steps as i32),
        }
    }

    fn modify_map(&self, map: &mut Vec<Vec<Terrain>>, x: i32, y: i32) -> (i32, i32) {
        let mut x = x;
        let mut y = y;
        for _ in 0..self.steps {
            (x, y) = match self.direction {
                Direction::R => (x + 1, y),
                Direction::L => (x - 1, y),
                Direction::U => (x, y - 1),
                Direction::D => (x, y + 1),
            };
            map[y as usize][x as usize] = Terrain::Trench;
        }

        (x, y)
    }
}

fn find_size_from_instructions(instructions: &Vec<Instruction>) -> (usize, usize, usize, usize) {
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut x = 0;
    let mut y = 0;
    for instruction in instructions.iter() {
        (x, y) = instruction.move_in_direction(x, y);

        if x > max_x {
            max_x = x
        }
        if x < min_x {
            min_x = x
        }

        if y > max_y {
            max_y = y
        }
        if y < min_y {
            min_y = y
        }
    }

    (
        -min_x as usize,
        (max_x - min_x + 1) as usize,
        -min_y as usize,
        (max_y - min_y + 1) as usize,
    )
}

#[derive(Clone, Copy)]
enum Terrain {
    Trench,
    Ground,
}

impl Debug for Terrain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Trench => write!(f, "#"),
            Terrain::Ground => write!(f, "."),
        }
    }
}

fn count_line(line: Vec<Terrain>) -> usize {
    // The problem with this is that it doesn't always correctly count the inside.
    // For example If we have the pattern '..^v..' then the last two dots are actually outisde, not inside
    // We thus need to modify the algorithm to give us the direction of each trench,
    // Or at least if it is an up-down or left-right trench. 
    let mut inside = false;
    let mut last_was_trench = false;
    let mut sum = 0;
    for terrain in line.iter() {
        sum += match terrain {
            Terrain::Trench => {
                if !last_was_trench {
                    inside = !inside
                };
                last_was_trench = true;
                1
            }
            Terrain::Ground => {
                last_was_trench = false;
                if inside {
                    1
                } else {
                    0
                }
            }
        }
    }

    sum
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day18/src/input.txt")?;
    let instructions = input
        .lines()
        .map(Instruction::from_string)
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let (start_x, size_x, start_y, size_y) = find_size_from_instructions(&instructions);
    println!("{} {} {} {}", start_x, size_x, start_y, size_y);

    let mut map = vec![vec![Terrain::Ground; size_x]; size_y];

    let mut x = start_x as i32;
    let mut y = start_y as i32;

    for instruction in instructions.iter() {
        (x, y) = instruction.modify_map(&mut map, x, y);
    }

    // println!("{:?}", map);

    let mut sum = 0;
    for line in map.iter() {
        sum += count_line(line.clone());
    }
    println!("{}", sum);

    Ok(())
}
