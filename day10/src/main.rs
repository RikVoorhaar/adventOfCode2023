use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Output;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    x: i32,
    y: i32,
}

impl Node {
    fn get_neighbours(&self, maze_char: char) -> Vec<Node> {
        match maze_char {
            '|' => vec![self.north(), self.south()],
            '-' => vec![self.east(), self.west()],
            'L' => vec![self.north(), self.east()],
            'J' => vec![self.north(), self.west()],
            '7' => vec![self.south(), self.west()],
            'F' => vec![self.south(), self.east()],
            '.' => Vec::new(),
            'S' => Vec::new(),
            _ => panic!("Unknown maze char: {}", maze_char),
        }
    }

    fn all_neighbors(&self) -> Vec<Node> {
        vec![self.north(), self.south(), self.east(), self.west()]
    }
    fn is_valid(&self, x_max: i32, y_max: i32) -> bool {
        self.x >= 0 && self.x < x_max && self.y >= 0 && self.y < y_max
    }

    fn north(&self) -> Node {
        Node {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn south(&self) -> Node {
        Node {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn west(&self) -> Node {
        Node {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn east(&self) -> Node {
        Node {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() -> Result<()> {
    let file = File::open("day10/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut neighbors: HashMap<Node, Vec<Node>> = HashMap::new();

    let mut start_node: Option<Node> = None;
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    let input_string = std::fs::read_to_string("day10/src/input.txt")?;
    let width = input_string.lines().next().unwrap().len();
    let height = input_string.lines().count();
    let new_line: Vec<char> = vec!['.'; width * 2];
    let mut new_input: Vec<Vec<char>> = vec![new_line; height * 2];
    for (y, line) in input_string.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            new_input[2 * y][2 * x] = char;

            match char {
                '|' => {
                    new_input[2 * y + 1][2 * x] = '|';
                    if y > 0 {
                        new_input[2 * y - 1][2 * x] = '|';
                    }
                }
                '-' => {
                    new_input[2 * y][2 * x + 1] = '-';
                    if x > 0 {
                        new_input[2 * y][2 * x - 1] = '-';
                    }
                }
                'L' => {
                    new_input[2 * y][2 * x + 1] = '-';
                    if y > 0 {
                        new_input[2 * y - 1][2 * x] = '|';
                    }
                }
                'J' => {
                    new_input[2 * y][2 * x + 1] = '-';
                    if y > 0 {
                        new_input[2 * y - 1][2 * x] = '|';
                    }
                }
                '7' => {
                    new_input[2 * y + 1][2 * x] = '|';
                    if x > 0 {
                        new_input[2 * y][2 * x - 1] = '-';
                    }
                }
                'F' => {
                    new_input[2 * y][2 * x + 1] = '-';
                    new_input[2 * y + 1][2 * x] = '|'
                }
                _ => continue,
            }
        }
    }
    for line in new_input.iter() {
        for ch in line.iter() {
            print!("{}", ch);
        }
        println!();
    }

    for (y_usize, line) in new_input.iter().enumerate() {
        let y: i32 = y_usize.try_into().unwrap();
        for (x_usize, char) in line.iter().enumerate() {
            let x: i32 = x_usize.try_into().unwrap();
            let node = Node { x, y };
            if *char == 'S' {
                start_node = Some(node);
                println!("Found start node: {:?}", start_node);
            }

            neighbors.insert(node, node.get_neighbours(*char));
        }
    }

    let mut main_loop: HashMap<Node, usize> = HashMap::new();
    main_loop.insert(start_node.unwrap(), 0);
    let mut next_layer: HashSet<Node> = HashSet::new();
    let mut current_level = 1;

    for (node, neighbors) in &neighbors {
        for neigh in neighbors {
            if *neigh == start_node.unwrap() {
                println!("Found start node in neighbors: {:?}", node);
                main_loop.insert(*node, 1);
                next_layer.insert(*node);
            }
        }
    }
    while !next_layer.is_empty() {
        current_level += 1;
        let current_layer = next_layer;
        next_layer = HashSet::new();
        for node in current_layer {
            for neighbor in &neighbors[&node] {
                if main_loop.contains_key(neighbor) {
                    continue;
                }
                main_loop.insert(*neighbor, current_level);
                next_layer.insert(*neighbor);
            }
        }
    }

    let mut outside_queue: HashSet<Node> = HashSet::new();
    let width = (2 * width - 1) as i32;
    let height = (2 * height - 1) as i32;

    for x in 0..width + 1 {
        let top_node = Node { x, y: 0 };
        if !main_loop.contains_key(&top_node) {
            outside_queue.insert(top_node);
        }
        let bottom_node = Node { x, y: height };
        if !main_loop.contains_key(&bottom_node) {
            outside_queue.insert(bottom_node);
        }
    }
    for y in 0..height + 1 {
        let left_node = Node { x: 0, y };
        if !main_loop.contains_key(&left_node) {
            outside_queue.insert(left_node);
        }
        let right_node = Node { x: width, y };
        if !main_loop.contains_key(&right_node) {
            outside_queue.insert(right_node);
        }
    }

    let mut outside_nodes: HashSet<Node> = outside_queue.clone();
    while !outside_queue.is_empty() {
        let mut new_queue = HashSet::new();
        for node in outside_queue {
            let neighbors_to_check = match main_loop.contains_key(&node) {
                false => node.all_neighbors(),
                true => neighbors[&node].clone(),
            };
            for neighbor in neighbors_to_check {
                if !neighbor.is_valid(width, height)
                    // || main_loop.contains_key(&neighbor)
                    || outside_nodes.contains(&neighbor)
                {
                    continue;
                }

                outside_nodes.insert(neighbor);
                new_queue.insert(neighbor);
            }
        }
        outside_queue = new_queue;
        println!(
            "N_outside nodes: {}, queue: {}",
            outside_nodes.len(),
            outside_queue.len()
        );
    }
    // let num_inside = (width + 1) * (height + 1) - (outside_nodes.len() as i32);
    // println!("Num inside: {}", num_inside);

    // println!("Outside nodes: {:?}", outside_nodes);
    println!("width: {}, height: {}", width, height);
    let mut num_inside = 0;
    for y in 0..height + 1 {
        for x in 0..width + 1 {
            let node = Node { x, y };
            if main_loop.contains_key(&node) {
                print!(" ");
            } else if outside_nodes.contains(&node) {
                print!("#");
            } else {
                print!(".");
                if x % 2 == 0 && y % 2 == 0 {
                    num_inside += 1;
                }
            }
        }
        println!();
    }
    println!("Num inside: {}", num_inside);

    Ok(())
}
