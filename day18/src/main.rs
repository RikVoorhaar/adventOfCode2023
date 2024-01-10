use anyhow::Result;
use std::{
    collections::HashSet,
    fmt::{Debug, Formatter},
};

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
            match self.direction {
                Direction::L | Direction::R => {
                    map[y as usize][x as usize] = Terrain::TrenchHorizontal;
                }
                Direction::U | Direction::D => {
                    map[y as usize][x as usize] = Terrain::TrenchVertical;
                }
            }
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Terrain {
    TrenchVertical,
    TrenchHorizontal,
    Ground,
}

impl Debug for Terrain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::TrenchVertical => write!(f, "|"),
            Terrain::TrenchHorizontal => write!(f, "-"),
            Terrain::Ground => write!(f, "."),
        }
    }
}

fn terrain_vec_to_string(terrain: &Vec<Terrain>) -> String {
    let mut s = String::new();
    for t in terrain.iter() {
        s.push_str(&format!("{:?}", t));
    }
    s
}

fn count_line(line: Vec<Terrain>) -> usize {
    // The problem with this is that it doesn't always correctly count the inside.
    // For example If we have the pattern '..^v..' then the last two dots are actually outisde, not inside
    // We thus need to modify the algorithm to give us the direction of each trench,
    // Or at least if it is an up-down or left-right trench.
    let mut inside = false;
    let mut last_was_h_trench = false;
    let mut last_was_v_trench = false;
    let mut sum = 0;
    for terrain in line.iter() {
        sum += match terrain {
            Terrain::TrenchHorizontal => {
                if !last_was_h_trench && !last_was_v_trench {
                    inside = !inside
                };
                last_was_h_trench = true;
                last_was_v_trench = false;
                1
            }
            Terrain::TrenchVertical => {
                if last_was_v_trench || !last_was_h_trench {
                    inside = !inside;
                }
                last_was_h_trench = false;
                last_was_v_trench = true;
                1
            }

            Terrain::Ground => {
                last_was_h_trench = false;
                last_was_v_trench = false;
                if inside {
                    1
                } else {
                    0
                }
            }
        }
    }
    println!("sum: {}, {}", sum, terrain_vec_to_string(&line));

    sum
}

fn count_outside(map: &Vec<Vec<Terrain>>, size_x: usize, size_y: usize) -> usize {
    let mut queue = Vec::new();
    let mut seen = HashSet::new();
    for i in 0..size_x {
        if map[0][i] == Terrain::Ground {
            queue.push((i, 0));
            seen.insert((i, 0));
        }
        if map[size_y - 1][i] == Terrain::Ground {
            queue.push((i, size_y - 1));
            seen.insert((i, size_y - 1));
        }
    }
    for i in 1..size_y-1 {
        if map[i][0] == Terrain::Ground {
            queue.push((0, i));
            seen.insert((0, i));
        }
        if map[i][size_x - 1] == Terrain::Ground {
            queue.push((size_x - 1, i));
            seen.insert((size_x - 1, i));
        }
    }

    let mut sum = 0;
    while let Some((x, y)) = queue.pop() {
        sum += 1;
        if x > 0 && map[y][x-1] == Terrain::Ground && seen.insert((x - 1, y)) {
            queue.push((x - 1, y));
        }
        if x < size_x - 1 && map[y][x+1] == Terrain::Ground && seen.insert((x + 1, y)) {
            queue.push((x + 1, y));
        }
        if y > 0 && map[y-1][x] == Terrain::Ground && seen.insert((x, y - 1)) {
            queue.push((x, y - 1));
        }
        if y < size_y - 1 && map[y+1][x] == Terrain::Ground && seen.insert((x, y + 1)) {
            queue.push((x, y + 1));
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

    // let size_x = size_x+1;
    let mut map = vec![vec![Terrain::Ground; size_x]; size_y];

    let mut x = start_x as i32;
    let mut y = start_y as i32;

    for instruction in instructions.iter() {
        (x, y) = instruction.modify_map(&mut map, x, y);
    }

    let mut sum = 0;
    for line in map.iter() {
        sum += count_line(line.clone());
    }
    println!("{}", sum);
    let num_outside = count_outside(&map, size_x, size_y);
    let mut num_hedge = 0;
    for line in map.iter() {
        for terrain in line.iter() {
            if *terrain != Terrain::Ground {
                num_hedge += 1;
            }
        }
    }
    let num_inside = size_x*size_y - num_outside;
    println!("{} {} {}", num_outside, num_inside, num_hedge);
    // println!("{}", count_outside(&map, size_x, size_y));

    Ok(())
}
