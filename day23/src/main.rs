use std::fmt::Debug;

use anyhow::Result;

// First thing we just have to build that graph Let's just make an enum with
// posibilities and parse based on that
// Then we just make a Vec<Vec<Tile>>.

// We then make a function that starts at a tile and then
// finds all the reachable slopes (or exit) form there.
// I guess the start and exit tile are special

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

struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType,
}

impl Tile {
    fn new(x: usize, y: usize, tile_type: TileType) -> Self {
        Self { x, y, tile_type }
    }

    fn neighbors(&self, hiking_map: &Vec<Vec<TileType>>) -> Vec<Tile> {
        let mut out = Vec::new();
        let num_rows = hiking_map.len();
        let num_cols = hiking_map[0].len();

        let x = self.x;
        let y = self.y;

        if x > 0 {
            out.push(Tile::new(x - 1, y, hiking_map[x - 1][y]));
        }

        out
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day23/src/example.txt")?;
    let mut hicking_map = input
        .lines()
        .map(|line| line.chars().map(TileType::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let num_rows = hicking_map.len();
    let num_cols = hicking_map[0].len();

    hicking_map[0][1] = TileType::Start;
    hicking_map[num_rows - 1][num_cols - 2] = TileType::Exit;
    println!("{:?}", hicking_map);

    Ok(())
}
