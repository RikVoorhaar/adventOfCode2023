#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use anyhow::Result;

// First thing we just have to build that graph Let's just make an enum with
// posibilities and parse based on that
// Then we just make a Vec<Vec<Tile>>.

// We then make a function that starts at a tile and then
// finds all the reachable slopes (or exit) form there.
// I guess the start and exit tile are special

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
enum TileType {
    Start,
    Exit,
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

impl TileType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::SlopeUp,
            'v' => Self::SlopeDown,
            '<' => Self::SlopeLeft,
            '>' => Self::SlopeRight,
            _ => panic!("Invalid tile"),
        }
    }
}

impl Debug for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            TileType::Start => 'S',
            TileType::Exit => 'E',
            TileType::Path => '.',
            TileType::Forest => '#',
            TileType::SlopeUp => '^',
            TileType::SlopeDown => 'v',
            TileType::SlopeLeft => '<',
            TileType::SlopeRight => '>',
        };
        write!(f, "{}", c)
    }
}

#[derive(Hash, Clone, PartialEq, Eq)]

struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{},{})", self.tile_type, self.x, self.y)
    }
}

impl Tile {
    fn new(x: usize, y: usize, tile_type: TileType) -> Self {
        Self { x, y, tile_type }
    }

    fn up(&self, hiking_map: &[Vec<TileType>]) -> Option<Tile> {
        if self.y > 0 {
            Some(Tile::new(
                self.x,
                self.y - 1,
                hiking_map[self.y - 1][self.x],
            ))
        } else {
            None
        }
    }

    fn down(&self, hiking_map: &[Vec<TileType>]) -> Option<Tile> {
        if self.y < hiking_map.len() - 1 {
            Some(Tile::new(
                self.x,
                self.y + 1,
                hiking_map[self.y + 1][self.x],
            ))
        } else {
            None
        }
    }

    fn left(&self, hiking_map: &[Vec<TileType>]) -> Option<Tile> {
        if self.x > 0 {
            Some(Tile::new(
                self.x - 1,
                self.y,
                hiking_map[self.y][self.x - 1],
            ))
        } else {
            None
        }
    }
    fn right(&self, hiking_map: &[Vec<TileType>]) -> Option<Tile> {
        if self.x < hiking_map[0].len() - 1 {
            Some(Tile::new(
                self.x + 1,
                self.y,
                hiking_map[self.y][self.x + 1],
            ))
        } else {
            None
        }
    }

    /// We need to have a second method that ignores the slope condition, because that's
    /// needed when determining whether something is a fork
    fn neighbors(&self, hiking_map: &[Vec<TileType>], check_slope: bool) -> Vec<Tile> {
        let mut out = Vec::new();
        if let Some(tile) = self.up(hiking_map) {
            match tile.tile_type {
                TileType::Forest => {}
                TileType::SlopeDown => {
                    if !check_slope {
                        out.push(tile)
                    }
                }
                _ => out.push(tile),
            }
        }
        if let Some(tile) = self.down(hiking_map) {
            match tile.tile_type {
                TileType::Forest => {}
                TileType::SlopeUp => {
                    if !check_slope {
                        out.push(tile)
                    }
                }
                _ => out.push(tile),
            }
        }
        if let Some(tile) = self.left(hiking_map) {
            match tile.tile_type {
                TileType::Forest => {}
                TileType::SlopeRight => {
                    if !check_slope {
                        out.push(tile)
                    }
                }
                _ => out.push(tile),
            }
        }
        if let Some(tile) = self.right(hiking_map) {
            match tile.tile_type {
                TileType::Forest => {}
                TileType::SlopeLeft => {
                    if !check_slope {
                        out.push(tile)
                    }
                }
                _ => out.push(tile),
            }
        }

        out
    }

    fn is_fork(&self, hiking_map: &[Vec<TileType>]) -> bool {
        self.neighbors(hiking_map, false).len() > 2
            || self.tile_type == TileType::Start
            || self.tile_type == TileType::Exit
    }
}

/// Finds distances to all forks reachable from this tile.
/// The input tile must be a fork itself
fn find_nodes(input_tile: Tile, hiking_map: &[Vec<TileType>]) -> Vec<(Tile, usize)> {
    let mut out = Vec::new();
    let mut checked = HashSet::new();
    checked.insert(input_tile.clone());

    let mut queue = input_tile
        .neighbors(hiking_map, false)
        .into_iter()
        .map(|t| (t, 1))
        .collect::<Vec<(Tile, usize)>>();

    while let Some((tile, d)) = queue.pop() {
        if !checked.insert(tile.clone()) {
            continue;
        }

        if tile.is_fork(hiking_map) {
            out.push((tile, d));
        } else {
            queue.extend(
                tile.neighbors(hiking_map, false)
                    .into_iter()
                    .map(|t| (t, d + 1)),
            )
        }
    }

    out
}


#[derive(Clone, Debug)]
struct Trail {
    head: Tile,
    visited: HashSet<Tile>,
    length: usize,
}

impl Trail {
    fn next_trails(&self, graph: &HashMap<Tile, Vec<(Tile, usize)>>) -> Vec<Self> {
        let mut out = Vec::new();
        for (tile, d) in &graph[&self.head] {
            if self.visited.contains(tile) {
                continue;
            }
            let mut new_visited = self.visited.clone();
            new_visited.insert(tile.clone());
            out.push(Self {
                head: tile.clone(),
                visited: new_visited,
                length: self.length + d,
            });
        }
        out
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day23/src/input.txt")?;
    let mut hiking_map = input
        .lines()
        .map(|line| line.chars().map(TileType::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let num_rows = hiking_map.len();
    let num_cols = hiking_map[0].len();
    let start_tile = Tile::new(1, 0, TileType::Start);

    hiking_map[0][1] = TileType::Start;
    hiking_map[num_rows - 1][num_cols - 2] = TileType::Exit;

    // Create the graph
    let mut queue = vec![start_tile.clone()];
    let mut graph = HashMap::new();
    while let Some(tile) = queue.pop() {
        if graph.contains_key(&tile) {
            continue;
        }
        let edges = find_nodes(tile.clone(), &hiking_map);
        println!("{:?} has connections {:?}", tile, edges);
        for (other_tile, _) in &edges {
            queue.push(other_tile.clone());
        }
        graph.insert(tile, edges);
    }

    // Find longest path
    let mut queue = vec![Trail {
        head: start_tile,
        visited: HashSet::new(),
        length: 0,
    }];
    let mut longest_trail = 0;
    while let Some(trail) = queue.pop() {
        if trail.head.tile_type == TileType::Exit && trail.length > longest_trail {
            println!("Found new longest trail: {:?}", trail);
            longest_trail = trail.length;
        }

        queue.extend(trail.next_trails(&graph));
    }

    Ok(())
}
