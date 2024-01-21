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

    fn is_garden_plot(&self, pos: Pos) -> bool {
        let (pos_x, pos_y) = self.get_mod_pos(pos);
        self.table[pos_y][pos_x]
    }

    fn get_mod_pos(&self, pos: Pos) -> (usize, usize) {
        (
            pos.x.rem_euclid(self.size_x) as usize,
            pos.y.rem_euclid(self.size_y) as usize,
        )
    }

    fn is_border(&self, pos: Pos) -> bool {
        let (pos_x, pos_y) = self.get_mod_pos(pos);
        return pos_x == 0
            || pos_x == 65
            || pos_x == 130
            || pos_y == 0
            || pos_y == 65
            || pos_y == 130;
    }

    fn closest_corners(&self, pos: Pos) -> Vec<(i64, i64)> {
        let (pos_x, pos_y) = self.get_mod_pos(pos);
        match (pos_x <= 65, pos_y <= 65) {
            (true, true) => vec![(0, 0), (0, 65), (65, 0), (65, 65)],
            (true, false) => vec![(0, 65), (0, 130), (65, 65), (65, 130)],
            (false, true) => vec![(65, 0), (65, 65), (130, 0), (130, 65)],
            (false, false) => vec![(65, 65), (65, 130), (130, 65), (130, 130)],
        }
    }

    fn map_pos_to_corner(&self, pos: Pos, corner: (i64, i64)) -> Pos {
        let (pos_x, pos_y) = self.get_mod_pos(pos);
        let pos_x_norm = pos.x - pos_x as i64;
        let pos_y_norm = pos.y - pos_y as i64;
        Pos {
            x: corner.0 + pos_x_norm,
            y: corner.1 + pos_y_norm,
        }
    }
}

fn step(
    garden: &Garden,
    step_num: usize,
    plots_reached: &mut HashMap<Pos, usize>,
    frontier: HashSet<Pos>,
    mod_reached: &mut Vec<Vec<Vec<(Pos, usize)>>>,
) -> HashSet<Pos> {
    let mut new_frontier = HashSet::new();
    for pos in frontier {
        for neighbor in pos.neighbors() {
            if !plots_reached.contains_key(&neighbor) && garden.is_garden_plot(neighbor) {
                let mod_x = neighbor.x.rem_euclid(garden.size_x);
                let mod_y = neighbor.y.rem_euclid(garden.size_y);
                let div_x = (neighbor.x - mod_x) / garden.size_x;
                let div_y = (neighbor.y - mod_y) / garden.size_y;

                mod_reached[mod_y as usize][mod_x as usize]
                    .push((Pos { x: div_x, y: div_y }, step_num));

                plots_reached.insert(neighbor, step_num);
                new_frontier.insert(neighbor);
            }
        }
    }

    new_frontier
}

fn count_reachable(plots_reached: &HashMap<Pos, usize>, step_num: usize) -> usize {
    let mut reachable = 0;
    let mod_val = step_num % 2;

    for (_, step) in plots_reached.iter() {
        if *step <= step_num && step % 2 == mod_val {
            reachable += 1;
        }
    }
    reachable
}
fn compute_num_reachable(
    tot_num_steps: usize,
    garden: &Garden,
    corner_to_distances: &HashMap<(i64, i64), [[usize; 131]; 131]>,
) {
    let mut mod_reached: Vec<Vec<Vec<(Pos, usize)>>> =
        vec![vec![vec![]; garden.size_x as usize]; garden.size_y as usize];
    let mut plots_reached: HashMap<Pos, usize> = HashMap::new();
    plots_reached.insert(garden.start_pos, 0);
    let mut frontier: HashSet<Pos> = HashSet::new();
    frontier.insert(garden.start_pos);
    let mut num_reached_even = 1;
    let mut num_reached_odd = 0;
    for step_num in 1..tot_num_steps + 1 {
        frontier = step(
            garden,
            step_num,
            &mut plots_reached,
            frontier,
            &mut mod_reached,
        );
        // let num_wrong = frontier
        //     .iter()
        //     .map(|&pos| compute_distance(pos, garden, corner_to_distances))
        //     .filter(|&x| x != step_num)
        //     .count();
        match step_num % 2 {
            0 => num_reached_even += frontier.len(),
            1 => num_reached_odd += frontier.len(),
            _ => unreachable!(),
        }
        let num_reached = match step_num % 2 {
            0 => num_reached_even,
            1 => num_reached_odd,
            _ => unreachable!(),
        };
        println!(
            "Step: {}, frontier: {}, reachable: {}",
            step_num,
            frontier.len(),
            // num_wrong,
            num_reached
        );
    }
    println!(
        "Plots reached: {}",
        count_reachable(&plots_reached, tot_num_steps)
    );
}
fn print_quadrant(quadrant: &Vec<Vec<bool>>) {
    for row in quadrant {
        for entry in row {
            if *entry {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
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

fn compute_distance(
    pos: Pos,
    garden: &Garden,
    corner_to_distances: &HashMap<(i64, i64), [[usize; 131]; 131]>,
) -> usize {
    if garden.is_border(pos) {
        return pos.distance();
    }

    let closest_corners = garden.closest_corners(pos);
    let mut min_dist = usize::MAX;
    for corner in closest_corners {
        let corner_dist = garden.map_pos_to_corner(pos, corner).distance();
        let (pos_x, pos_y) = garden.get_mod_pos(pos);
        let dist = corner_to_distances[&corner][pos_y][pos_x] + corner_dist;

        if dist < min_dist {
            min_dist = dist;
        }
    }

    min_dist
}

// TODO: This is just wrong; because it assumes that all points are reachable I suppose.
// We can just use the existing distance map from (65,65) and count the number of even
// and odd distances...
fn num_odd_even_plots<const N: usize>(table: &[[bool; N]; N]) -> (usize, usize) {
    let mut num_odd = 0;
    let mut num_even = 0;
    for (i, row) in table.iter().enumerate() {
        for (j, &is_plot) in row.iter().enumerate() {
            if is_plot {
                let pos = Pos {
                    x: (i as i64 - 65),
                    y: (j as i64 - 65),
                };

                if pos.distance() % 2 == 0 {
                    num_even += 1;
                } else {
                    num_odd += 1;
                }
            }
        }
    }
    (num_odd, num_even)
}

fn count_num_lattice_points(d: f64) -> (usize, usize, Vec<Pos>) {
    let mut num_odd = 0;
    let mut num_even = 0;
    let boundary_width: f64 = 4.0;
    let boundary_width_int = boundary_width.ceil() as i64;
    let r = (d + boundary_width).ceil() as i64;
    let r2 = ((d) * (d)).floor() as i64;
    let inner_r2 = ((d - boundary_width).max(0.0) * (d - boundary_width).max(0.0)).floor() as i64;
    let outer_r2 = ((d + boundary_width) * (d + boundary_width)).floor() as i64;

    let mut overlap_points = Vec::new();
    print!("Including: ");
    for y in -r..=r {
        print!(" | ");
        let y2 = y * y;
        let x_length = ((inner_r2 - y2) as f64).sqrt().floor() as i64;
        for x in x_length + 1..=x_length + boundary_width_int {
            let dist2 = x * x + y2;
            // println!("x,y:{},{}, dist2: {}. inner_r2 {}, outer_r2 {}",x, y, dist2, inner_r2, outer_r2);
            if dist2 >= inner_r2 && dist2 < outer_r2 {
                print!("({}, {}), ", x, y);
                overlap_points.push(Pos { x, y });
                if x != 0 {
                    overlap_points.push(Pos { x: -x, y });
                    print!("({}, {}), ", -x, y);
                }
            }
        }

        if y2 > inner_r2 {
            let x = x_length;
            print!("({}, {}), ", x, y);
            overlap_points.push(Pos { x, y });
            if x > 0 {
                print!("({}, {}), ", -x, y);
                overlap_points.push(Pos { x: -x, y });
            }
            continue;
        }

        // for x in -x_length..=x_length {
        //     print!("({}, {}), ", x, y);
        // }
        let total_points = 2 * x_length + 1;
        let (odd, even) = if (y + x_length) % 2 != 0 {
            ((total_points + 1) / 2, total_points / 2)
        } else {
            (total_points / 2, (total_points + 1) / 2)
        };

        num_odd += odd as usize;
        num_even += even as usize;
    }
    println!();

    (num_odd, num_even, overlap_points)
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
    for r in r_inner+1..=r_outer {
        overlap_points.extend(get_tiles_at_distance(r))
    }


    (num_odd, num_even, overlap_points)
}
fn count_num_lattice_points_dumb(d: f64) -> (usize, usize, Vec<Pos>) {
    let mut num_odd = 0;
    let mut num_even = 0;
    let boundary_width = 10.0;

    let r = (d + boundary_width).ceil() as i64;
    let r2 = ((d - boundary_width) * (d - boundary_width)).floor() as i64;
    let outer_r2 = ((d + boundary_width) * (d + boundary_width)).floor() as i64;
    let mut overlap_points = Vec::new();
    for y in -r..=r {
        for x in -r..=r {
            // println!(
            //     "x: {}, y: {}, r: {}, r2: {}, outer_r2: {}",
            //     x, y, r, r2, outer_r2
            // );
            let dist = x * x + y * y;
            if dist <= r2 && d > boundary_width {
                if (x + y) % 2 == 0 {
                    num_even += 1;
                } else {
                    num_odd += 1;
                }
            } else if dist <= outer_r2 {
                overlap_points.push(Pos { x, y });
            }
        }
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
    // println!("corner: {:?}", corner);
    let map = corner_to_distances[&(corner.x, corner.y)];
    let res = map
        .iter()
        .flatten()
        .zip(table.iter().flatten())
        .filter(|(&dist, &is_plot)| {
            is_plot && dist <= distance_left && dist % 2 == distance_left % 2
        })
        .count();
    // println!(
    //     "corner: {:?}, distance_left: {}, res: {}",
    //     corner, distance_left, res
    // );
    res
}
fn print_reachable(
    corner: Pos,
    distance_left: usize,
    corner_to_distances: &HashMap<(i64, i64), [[usize; 131]; 131]>,
    table: &[[bool; 131]; 131],
) {
    let map = corner_to_distances[&(corner.x, corner.y)];
    print!("Corner: {:?}, distnace_left: {}", corner, distance_left);
    for (y, row) in map.iter().enumerate() {
        for (x, &dist) in row.iter().enumerate() {
            if table[y][x] && dist <= distance_left && dist % 2 == distance_left % 2 {
                print!("({}, {})", x, y);
            }
        }
    }
    println!();
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

    // compute_num_reachable(2000, &garden, &corner_to_distances);

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
    // let num_steps = 2000;

    let radius_tiles = ((num_steps as f64) / 131.0).max(0.0);

    let mut num_reachable = 0;

    let (num_odd_tiles, num_even_tiles, boundary_points) =
        count_num_lattice_points_manhattan(radius_tiles - 0.5);
    // println!("Boundary points: {:?}", boundary_points);
    println!(
        "smart. r: {}, odd/even/tot: {}, {}, {}. Boundary length: {}", //; dumb: {}, {}, {}.",
        radius_tiles,
        num_odd_tiles,
        num_even_tiles,
        num_odd_tiles + num_even_tiles,
        boundary_points.len(),
    );
    // let (num_odd_tiles, num_even_tiles, boundary_points) =
    //     count_num_lattice_points_dumb(radius_tiles - 0.5);
    // println!(
    //     "dumb. r: {}, odd/even/tot: {}, {}, {}. Boundary length: {}", //; dumb: {}, {}, {}.",
    //     radius_tiles,
    //     num_odd_tiles,
    //     num_even_tiles,
    //     num_odd_tiles + num_even_tiles,
    //     boundary_points.len(),
    // );

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
        let tile_distnace = tile.x.abs() + tile.y.abs();
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

        // println!(
        //     "tile: {:?}, distance: {}, num: {}, distance_remaining: {}",
        //     tile, tile_distnace, num_reachable_from_tile, distance_remaining
        // );
    }
    println!("Num steps: {}, Num reachable: {}", num_steps, num_reachable);
    // print_reachable(Pos { x: 65, y: 65 }, 0, &corner_to_distances, &table_mat);
    // print_reachable(Pos { x: 65, y: 65 }, 1, &corner_to_distances, &table_mat);
    // print_reachable(Pos { x: 65, y: 65 }, 2, &corner_to_distances, &table_mat);

    Ok(())
    // We're almost there. We just need to figure out why I need to set the
    // boundary_width so high. It really should be 1.0, or 2.0 TOPS. Yet, I'm not
    // getting the right answers otherwise. Why is that? Am I not counting the odd/even
    // squares in the right way? Am I missing out on entire tiles? Like what is the
    // problem. We can start out by again printing the number of tiles we found, but add
    // a distance from start (in terms of entire tiles) to the print. We should also
    // look for the correct answers how many tiles we actually 'skip' counting.
}
