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
}

/// Find all the nodes that can be reached from a tile
/// The starting tile must be a path tile
/// TODO: This is great, but we also need to find the distance. For this I think we can
/// assume that there are no loops in any of the paths, so that the shortest and longest
/// non-intersecting paths are always the same. I hope that's the case.
fn find_nodes(input_tile: Tile, hiking_map: &[Vec<TileType>]) -> Vec<(Tile, usize)> {
    let next_tile = match input_tile.tile_type {
        TileType::Start => input_tile.down(hiking_map),
        TileType::Exit => input_tile.up(hiking_map),
        TileType::SlopeDown => input_tile.down(hiking_map),
        TileType::SlopeUp => input_tile.up(hiking_map),
        TileType::SlopeLeft => input_tile.left(hiking_map),
        TileType::SlopeRight => input_tile.right(hiking_map),
        _ => return Vec::new(),
    };
    if next_tile.is_none() {
        return Vec::new();
    }
    let next_tile = next_tile.unwrap();
    if next_tile.tile_type != TileType::Path {
        return vec![(next_tile, 1)];
    }
    let mut out = Vec::new();
    let mut checked = HashSet::new();
    checked.insert(input_tile.clone());
    checked.insert(next_tile.clone());
    let mut queue = vec![(next_tile, 1)];

    while let Some((tile, d)) = queue.pop() {
        if let Some(up) = tile.up(hiking_map) {
            if checked.insert(up.clone()) {
                match up.tile_type {
                    TileType::SlopeDown | TileType::Forest => {}
                    TileType::Path => queue.push((up, d + 1)),
                    _ => out.push((up, d + 1)),
                }
            }
        }
        if let Some(down) = tile.down(hiking_map) {
            if checked.insert(down.clone()) {
                match down.tile_type {
                    TileType::SlopeUp | TileType::Forest => {}
                    TileType::Path => queue.push((down, d + 1)),
                    _ => out.push((down, d + 1)),
                }
            }
        }
        if let Some(left) = tile.left(hiking_map) {
            if checked.insert(left.clone()) {
                match left.tile_type {
                    TileType::SlopeRight | TileType::Forest => {}
                    TileType::Path => queue.push((left, d + 1)),
                    _ => out.push((left, d + 1)),
                }
            }
        }
        if let Some(right) = tile.right(hiking_map) {
            if checked.insert(right.clone()) {
                match right.tile_type {
                    TileType::SlopeLeft | TileType::Forest => {}
                    TileType::Path => queue.push((right, d + 1)),
                    _ => out.push((right, d + 1)),
                }
            }
        }
    }

    out
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day23/src/example.txt")?;
    let mut hiking_map = input
        .lines()
        .map(|line| line.chars().map(TileType::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let num_rows = hiking_map.len();
    let num_cols = hiking_map[0].len();
    let start_tile = Tile::new(1, 0, TileType::Start);

    hiking_map[0][1] = TileType::Start;
    hiking_map[num_rows - 1][num_cols - 2] = TileType::Exit;

    // let connected_to_start = find_nodes(start_tile, &hiking_map);
    let mut queue = vec![start_tile];
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
    // Next is just finding the longest acyclic path in a directed graph. It is not a
    // directed _acyclic_ graph, so this problem isn't trivial.

    Ok(())
}
