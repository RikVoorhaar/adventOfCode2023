use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

fn main() -> Result<()> {
    let file = File::open("day10/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut neighbors: HashMap<Node, Vec<Node>> = HashMap::new();

    let mut start_node: Option<Node> = None;
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        for (x, char) in line.chars().enumerate() {
            let node = Node {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            if char == 'S' {
                start_node = Some(node);
                println!("Found start node: {:?}", start_node);
            }

            neighbors.insert(node, node.get_neighbours(char));
        }
    }
    let mut distance_map: HashMap<Node, usize> = HashMap::new();
    distance_map.insert(start_node.unwrap(), 0);
    let mut next_layer: HashSet<Node> = HashSet::new();
    let mut current_level = 1;

    for (node, neighbors) in &neighbors {
        for neigh in neighbors {
            if *neigh == start_node.unwrap() {
                println!("Found start node in neighbors: {:?}", node);
                distance_map.insert(*node, 1);
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
                if distance_map.contains_key(neighbor) {
                    continue;
                }
                distance_map.insert(*neighbor, current_level);
                next_layer.insert(*neighbor);
            }
        }
        println!("current_level: {}", current_level);
        println!("next_layer: {:?}", next_layer);
    }

    Ok(())
}
