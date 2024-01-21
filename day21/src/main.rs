use anyhow::Result;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]

struct Pos {
    x: i64,
    y: i64,
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Pos {
    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
            Pos {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }

    fn distance(&self) -> usize {
        // Absolute distance to origin ignoring obstacles
        ((self.x - 65).abs() + (self.y - 65).abs()) as usize
    }
}

struct Garden {
    table: Vec<Vec<bool>>,
    size_x: i64,
    size_y: i64,
    start_pos: Pos,
}

impl Garden {
    fn from_string(input: &str) -> Garden {
        let size_y = input.lines().count();
        let size_x = input.lines().next().unwrap().len();

        let mut table: Vec<Vec<bool>> = vec![vec![false; size_x]; size_y];
        let mut start_pos = Pos { x: 0, y: 0 };
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '#' {
                    table[y][x] = true;
                }
                if c == 'S' {
                    start_pos = Pos {
                        x: x as i64,
                        y: y as i64,
                    };
                }
            }
        }
        Garden {
            table,
            size_x: size_x as i64,
            size_y: size_y as i64,
            start_pos,
        }
    }
}

fn find_distances_to_points<const N: usize>(
    table: &[[bool; N]; N],
    starting_point: Pos,
) -> [[usize; N]; N] {
    let mut distances = [[usize::MAX; N]; N];
    let mut current_distance = 0;
    let mut frontier = vec![starting_point];
    while !frontier.is_empty() {
        let mut new_frontier = Vec::new();
        for pos in frontier {
            if pos.x < 0 || pos.y < 0 || pos.x as usize >= N || pos.y as usize >= N {
                continue;
            }

            let x = pos.x as usize;
            let y = pos.y as usize;

            if table[y][x] && distances[y][x] == usize::MAX {
                distances[y][x] = current_distance;
                new_frontier.extend(pos.neighbors())
            }
            // else if !table[y][x] {
            //     distances[y][x] = usize::MAX;
            // }
        }
        frontier = new_frontier;
        current_distance += 1;
    }
    println!(
        "Longest distance for {:?} is {}",
        starting_point,
        current_distance - 1
    );

    distances
}

fn vec_table_to_array<const N: usize>(table: &Vec<Vec<bool>>) -> [[bool; N]; N] {
    let mut array_table = [[false; N]; N];
    for (i, row) in table.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            array_table[i][j] = *entry;
        }
    }
    array_table
}

/// Get all the tiles at a manhattan distance of d from the origin
fn get_tiles_at_distance(d: i64) -> Vec<Pos> {
    let mut out = Vec::new();

    for x in 0..=d {
        let y = d - x;
        out.push(Pos { x, y });
        match (x == 0, y == 0) {
            (false, false) => {
                out.push(Pos { x: -x, y });
                out.push(Pos { x, y: -y });
                out.push(Pos { x: -x, y: -y });
            }
            (true, false) => {
                out.push(Pos { x, y: -y });
            }
            (false, true) => {
                out.push(Pos { x: -x, y });
            }
            (true, true) => (),
        }
    }

    out
}

/// We shouldn't be using a euclidean circle; but a cirlce in the manhattan distance
/// That actually makes it easier. We just need to iterate over all the pairs of
/// points With |x|+|y| = (something). By symmetry we just pick the points with
/// positive x,y and then complete the rest.
fn count_num_lattice_points_manhattan(d: f64) -> (usize, usize, Vec<Pos>) {
    let mut num_odd = 0;
    let mut num_even = 0;
    let boundary_width: i64 = 2;
    let r = d.ceil() as i64;
    let r_inner = r - boundary_width;
    let r_outer = r + boundary_width;

    for y in -r_inner..=r_inner {
        let x_length = r_inner - y.abs();

        let total_points = 2 * x_length + 1;
        let (odd, even) = if (y + x_length) % 2 != 0 {
            ((total_points + 1) / 2, total_points / 2)
        } else {
            (total_points / 2, (total_points + 1) / 2)
        };
        num_odd += odd as usize;
        num_even += even as usize;
    }

    let mut overlap_points = Vec::new();
    for r in r_inner + 1..=r_outer {
        overlap_points.extend(get_tiles_at_distance(r))
    }

    (num_odd, num_even, overlap_points)
}
/// For a 131x131 tile, find the coordinates of the corner closest to the origin
fn closest_corner(tile_pos: Pos) -> (Pos, Pos) {
    let x = match tile_pos.x.cmp(&0) {
        Equal => 65,
        Less => 130,
        Greater => 0,
    };
    let y = match tile_pos.y.cmp(&0) {
        Equal => 65,
        Less => 130,
        Greater => 0,
    };
    (
        Pos { x, y },
        Pos {
            x: x + 131 * tile_pos.x,
            y: y + 131 * tile_pos.y,
        },
    )
}

fn num_reachable_from(
    corner: Pos,
    distance_left: usize,
    corner_to_distances: &HashMap<(i64, i64), [[usize; 131]; 131]>,
    table: &[[bool; 131]; 131],
) -> usize {
    let map = corner_to_distances[&(corner.x, corner.y)];
    let res = map
        .iter()
        .flatten()
        .zip(table.iter().flatten())
        .filter(|(&dist, &is_plot)| {
            is_plot && dist <= distance_left && dist % 2 == distance_left % 2
        })
        .count();
    res
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day21/src/input.txt")?;

    let garden = Garden::from_string(&input);

    let table_mat = vec_table_to_array::<131>(&garden.table);
    let corners = [
        (0, 0),
        (0, 65),
        (0, 130),
        (65, 0),
        (65, 65),
        (65, 130),
        (130, 0),
        (130, 65),
        (130, 130),
    ];
    let corner_to_distances = corners
        .iter()
        .map(|&corner| {
            (
                corner,
                find_distances_to_points(
                    &table_mat,
                    Pos {
                        x: corner.0,
                        y: corner.1,
                    },
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut num_false_col = vec![0; garden.size_y as usize];
    for (i, row) in garden.table.iter().enumerate() {
        let mut num_false = 0;
        for (j, entry) in row.iter().enumerate() {
            if !entry {
                num_false += 1;
                num_false_col[j] += 1
            }
        }
        if num_false == 0 {
            println!("row i={} has no rocks", i);
        }
    }
    for (j, &num_false) in num_false_col.iter().enumerate() {
        if num_false == 0 {
            println!("col j={} has no rocks", j);
        }
    }
    println!("Start pos {:?}", garden.start_pos);

    // let (num_plots_in_odd, num_plots_in_even) = num_odd_even_plots(&table_mat);
    let num_plots_in_odd =
        num_reachable_from(Pos { x: 65, y: 65 }, 133, &corner_to_distances, &table_mat);
    let num_plots_in_even =
        num_reachable_from(Pos { x: 65, y: 65 }, 132, &corner_to_distances, &table_mat);
    println!(
        "Num odd: {}, num even: {}",
        num_plots_in_odd, num_plots_in_even
    );

    let mut distance_cache: HashMap<(Pos, usize), usize> = HashMap::new();
    let num_steps = 26501365;

    let radius_tiles = ((num_steps as f64) / 131.0).max(0.0);

    let mut num_reachable = 0;

    let (num_odd_tiles, num_even_tiles, boundary_points) =
        count_num_lattice_points_manhattan(radius_tiles - 0.5);

    if num_steps % 2 == 0 {
        num_reachable += num_even_tiles * num_plots_in_even + num_odd_tiles * num_plots_in_odd;
    } else {
        num_reachable += num_odd_tiles * num_plots_in_even + num_even_tiles * num_plots_in_odd;
    }
    for tile in boundary_points {
        let (corner_mod, corner) = closest_corner(tile);
        let d = corner.distance();
        if d > num_steps {
            continue;
        }
        let distance_remaining = num_steps - d;
        let num_reachable_from_tile = *distance_cache
            .entry((corner_mod, distance_remaining))
            .or_insert_with(|| {
                num_reachable_from(
                    corner_mod,
                    distance_remaining,
                    &corner_to_distances,
                    &table_mat,
                )
            });
        num_reachable += num_reachable_from_tile;
    }
    println!("Num steps: {}, Num reachable: {}", num_steps, num_reachable);

    Ok(())
}
