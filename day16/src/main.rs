use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct TileInfo {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Mirror {
    Slash,
    BackSlash,
    Vertical,
    Horizontal,
}

fn find_next_tile_pos(tile: &TileInfo, size_x: usize, size_y: usize) -> Option<(usize, usize)> {
    let (x, y) = match tile.direction {
        Direction::Right => (tile.x + 1, tile.y),
        Direction::Left => {
            if tile.x == 0 {
                return None;
            }
            (tile.x - 1, tile.y)
        }
        Direction::Up => {
            if tile.y == 0 {
                return None;
            }
            (tile.x, tile.y - 1)
        }
        Direction::Down => (tile.x, tile.y + 1),
    };
    if x >= size_x || y >= size_y {
        return None;
    }
    Some((x, y))
}

fn parse_tile(
    tile: TileInfo,
    mirrors: &HashMap<(usize, usize), Mirror>,
    size_x: usize,
    size_y: usize,
    is_first: bool,
) -> Option<Vec<TileInfo>> {
    // println!("parsing Tile: {:?}", tile);

    let (x, y) = if is_first {
        (tile.x, tile.y)
    } else {
        find_next_tile_pos(&tile, size_x, size_y)?
    };

    // println!("Next position: ({}, {})", x, y);
    let mut new_directions: Vec<Direction> = Vec::new();

    if let Some(mirror) = mirrors.get(&(x, y)) {
        match mirror {
            Mirror::Vertical => match tile.direction {
                Direction::Right | Direction::Left => {
                    new_directions.append(&mut vec![Direction::Up, Direction::Down])
                }
                _ => new_directions.push(tile.direction),
            },
            Mirror::Horizontal => match &tile.direction {
                Direction::Up | Direction::Down => {
                    new_directions.append(&mut vec![Direction::Right, Direction::Left])
                }
                _ => new_directions.push(tile.direction),
            },
            Mirror::Slash => match &tile.direction {
                Direction::Right => new_directions.push(Direction::Up),
                Direction::Left => new_directions.push(Direction::Down),
                Direction::Up => new_directions.push(Direction::Right),
                Direction::Down => new_directions.push(Direction::Left),
            },
            Mirror::BackSlash => match &tile.direction {
                Direction::Right => new_directions.push(Direction::Down),
                Direction::Left => new_directions.push(Direction::Up),
                Direction::Up => new_directions.push(Direction::Left),
                Direction::Down => new_directions.push(Direction::Right),
            },
        }
    } else {
        new_directions.push(tile.direction);
    }

    Some(
        new_directions
            .iter()
            .map(|&direction| TileInfo { x, y, direction })
            .collect(),
    )
}

fn num_energized(
    start_tile: TileInfo,
    mirrors: &HashMap<(usize, usize), Mirror>,
    size_x: usize,
    size_y: usize,
) -> usize {
    let mut queue: Vec<TileInfo> = vec![start_tile.clone()];

    let mut found_squares: HashSet<(usize, usize)> = HashSet::new();
    found_squares.insert((start_tile.x, start_tile.y));

    let mut found_tiles: HashSet<TileInfo> = HashSet::new();
    found_tiles.insert(start_tile.clone());

    let mut is_first = true;
    while let Some(tile) = queue.pop() {
        if let Some(new_tiles) = parse_tile(tile, mirrors, size_x, size_y, is_first) {
            // println!("New tiles: {:?}", new_tiles);
            for new_tile in new_tiles {
                found_squares.insert((new_tile.x, new_tile.y));
                if !found_tiles.insert(new_tile.clone()) && !is_first {
                    continue;
                }
                queue.push(new_tile);
            }
            is_first = false;
        }
    }
    println!("({}, {}): {}", start_tile.x, start_tile.y, found_squares.len());
    

    found_squares.len()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day16/src/input.txt")?;
    let mut mirrors: HashMap<(usize, usize), Mirror> = HashMap::new();
    let mut size_x = 0;
    let mut size_y = 0;
    for (y, line) in input.lines().enumerate() {
        size_y = y;
        size_x = line.len();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '/' => {
                    mirrors.insert((x, y), Mirror::Slash);
                }
                '\\' => {
                    mirrors.insert((x, y), Mirror::BackSlash);
                }
                '|' => {
                    mirrors.insert((x, y), Mirror::Vertical);
                }
                '-' => {
                    mirrors.insert((x, y), Mirror::Horizontal);
                }
                _ => {}
            }
        }
    }
    size_y += 1;

    let mut edge_tiles: Vec<TileInfo> = Vec::new();



    // Left edge
    edge_tiles.append(
        &mut (0..size_y)
            .map(|y| TileInfo {
                x: 0,
                y,
                direction: Direction::Right,
            })
            .collect(),
    );

    // Right edge
    edge_tiles.append(
        &mut (0..size_y)
            .map(|y| TileInfo {
                x: size_x - 1,
                y,
                direction: Direction::Left,
            })
            .collect(),
    );

    // Top edge
    edge_tiles.append(
        &mut (0..size_x)
            .map(|x| TileInfo {
                x,
                y: 0,
                direction: Direction::Down,
            })
            .collect(),
    );

    // Bottom edge
    edge_tiles.append(
        &mut (0..size_x)
            .map(|x| TileInfo {
                x,
                y: size_y - 1,
                direction: Direction::Up,
            })
            .collect(),
    );
    println!("Edge tiles: {:?}", edge_tiles);


    let biggest_tile = edge_tiles
        .iter()
        .map(|tile| num_energized(tile.clone(), &mirrors, size_x, size_y))
        .max();
    println!("Energy: {:?}", biggest_tile);

    Ok(())
}
