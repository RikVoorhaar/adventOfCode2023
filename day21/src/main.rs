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
    let mut distances = [[0usize; N]; N];
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

            if table[y][x] && distances[y][x] == 0 {
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
    let r = (d + 1.0).ceil() as i64;
    let r2 = (d * d).floor() as i64;
    let outer_r2 = ((d + 1.0) * (d + 1.0)).floor() as i64;
    let mut overlap_points = Vec::new();
    for y in -r..=r {
        let x_length = ((r2 - y * y) as f64).sqrt().floor() as i64;
        for &x in &[
            x_length - 1,
            x_length,
            x_length + 1,
            -x_length - 1,
            -x_length,
            -x_length + 1,
        ] {
            let dist2 = x * x + y * y;
            if dist2 > r2 && dist2 <= outer_r2 {
                overlap_points.push(Pos { x, y });
            }
        }

        if x_length < 0 {
            continue;
        }
        let total_points = 2 * x_length + 1;
        let (odd, even) = if (y + x_length) % 2 != 0 {
            ((total_points + 1) / 2, total_points / 2)
        } else {
            (total_points / 2, (total_points + 1) / 2)
        };

        num_odd += odd as usize;
        num_even += even as usize;
    }

    (num_odd, num_even, overlap_points)
}
fn count_num_lattice_points_dumb(d: f64) -> (usize, usize, Vec<Pos>) {
    let mut num_odd = 0;
    let mut num_even = 0;
    let r = (d + 1.0).ceil() as i64;
    let r2 = ((d - 1.0) * (d - 1.0)).floor() as i64;
    let outer_r2 = ((d + 1.0) * (d + 1.0)).floor() as i64;
    let mut overlap_points = Vec::new();
    for y in -r..=r {
        for x in -r..=r {
            println!(
                "x: {}, y: {}, r: {}, r2: {}, outer_r2: {}",
                x, y, r, r2, outer_r2
            );
            let dist = x * x + y * y;
            if dist <= r2 && d > 1.0 {
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
    println!("corner: {:?}", corner);
    let map = corner_to_distances[&(corner.x, corner.y)];
    let res = map
        .iter()
        .flatten()
        .zip(table.iter().flatten())
        .filter(|(&dist, &is_plot)| {
            is_plot && dist <= distance_left && dist % 2 == distance_left % 2
        })
        .count();
    println!(
        "corner: {:?}, distance_left: {}, res: {}",
        corner, distance_left, res
    );
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

    // compute_num_reachable(1000, &garden, &corner_to_distances);

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

    let (num_plots_in_odd, num_plots_in_even) = num_odd_even_plots(&table_mat);
    println!(
        "Num odd: {}, num even: {}",
        num_plots_in_odd, num_plots_in_even
    );

    let mut distance_cache: HashMap<(Pos, usize), usize> = HashMap::new();
    let num_steps = 2;

    let radius_tiles = ((num_steps as f64) / 131.0).max(0.0);

    let mut num_reachable = 0;

    let (num_odd_tiles, num_even_tiles, boundary_points) = count_num_lattice_points(radius_tiles);
    let (num_odd_tiles, num_even_tiles, boundary_points) =
        count_num_lattice_points_dumb(radius_tiles - 0.5);
    println!(
        "smart. r: {}, odd/even/tot: {}, {}, {}. Boundary length: {}", //; dumb: {}, {}, {}.",
        radius_tiles,
        num_odd_tiles,
        num_even_tiles,
        num_odd_tiles + num_even_tiles,
        boundary_points.len(),
    );

    if num_steps % 2 == 1 {
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
        println!(
            "Corner: {:?}, corner_mod: {:?}, distance_remaining: {}",
            corner, corner_mod, distance_remaining
        );
        num_reachable += *distance_cache
            .entry((corner_mod, distance_remaining))
            .or_insert_with(|| {
                num_reachable_from(
                    corner_mod,
                    distance_remaining,
                    &corner_to_distances,
                    &table_mat,
                )
            });
    }
    println!("Num steps: {}, Num reachable: {}", num_steps, num_reachable);

    Ok(())
    // Next up, we need to compute the cost of going one up/down based on neighbors.
    // Then use that to verify hypothesis

    // Actually an interesting observation: In both the input and example, there is a
    // border of dots around the edge. This might be only straight line of dots.
    // Thus to get anywhere, we always need to use this 'highway'
    // We can use this to compute the distance between any two points
    // We just computer the distances to all the border points for the two points, then
    // compute the distance between the border points, and take the minimum.
    // This is much faster, because the distance to each of the border points is always
    // the same, so we can just make a LOT

    // Let's first check the border hypothesis

    // No, but in some sense it's easier. There are empty rows on i=0,65,130 and empty
    // cols on the smae indices. Crucially the starting position is at (65,65). Thus to
    // get to any point we can get there either from the starting 'cross' or from the
    // border around the outside.

    // The look-up table thing is also no dice, because it would contain >1 million
    // entries per quadrant. But I suppose it is enough actually to just know the
    // distance to all the points in the quadrant from each of the four corners of the
    // quadrant. Nice thing is that we even start of at a corner!

    // So we start with making a function that starts with a point, and then creates a
    // table with distances to all the points in the table from there. Then we create a
    // list of corners (0,0), (0,65), (0,130), (65,0), (65,65), (65,130), (130,0),
    // (130,65), (130,130) each with their own LOT, this will have  135,200 entries; a
    // lot, but much less at least.

    // ----
    // Next: We have to test this function 'compute distance' and compare it to existing
    // things. Just discover points distance wise and check if the function returns
    // true.
    // (DONE)
    //
    // After that we have to check what the maximum value of the distances is, that way
    // We can for each square immediately tell if it is reached within the specified
    // number of steps or not. Probably all the ones in a specific radius are. Then we
    // just have a thin circle where we need to check. Let's estimate:
    //
    // The circle has radius 202300 squares,  which means roughly 1,271,088 squares in
    // the circle (or twice as much perhaps), then for each we need to do 131*131 look
    // ups resulting in a total of around 21,813,147,820 lookups. I'm not sure that's
    // computationally feasible, actually. But we can do a lot of caching; We just need
    // to compute the number of squares that are reachable. This is always going to be
    // the same, depending on which corner is closest to the starting point.
    //
    // Yes, largest distance is 261 for corners, 131 for middle and 196 for sides.
    // We also need to determine if a square is 'odd' or 'even', and we need to determine
    // The number of odd and even squares without rocks
    //
    // So really we just need to compute for each square, which corner is closest to starting point,
    // And then look up how many points fit within a certain range of distances.
    // We can just cache that result, and put it in a hash table.
    //
    // Of course we need to keep into account whether or not the number is odd or even!
}
