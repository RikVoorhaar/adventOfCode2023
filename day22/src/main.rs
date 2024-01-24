use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use anyhow::Result;

/// We need to first load all the bricks from the file, and make a struct for them Then
/// we need to give each brick an id. Then we make a hashmap mapping each (x,y)
/// coordinate that has any bricks above it to a list of brick ids
///
/// To solve the first part of the problem we just need to find the list of bricks that
/// do not have any bricks _above_ them So we need to make a simple comparison function
/// for bricks that tells whether a brick is above or below it.
///
/// Then we iterate over all the bricks, use the hashmap to find potential bricks that
/// could be aboce them. If there are none then we add one to the counter.

#[derive(Clone)]
struct Brick {
    pos1: Pos3,
    pos2: Pos3,
}

#[derive(Clone)]
struct Pos3 {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Pos2 {
    x: usize,
    y: usize,
}

impl Debug for Pos2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
impl Debug for Pos3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}~{:?}", self.pos1, self.pos2)
    }
}

impl Pos3 {
    fn from_string(s: &str) -> Self {
        let mut split_comma = s.split(',');
        let x = split_comma.next().unwrap().parse::<usize>().unwrap();
        let y = split_comma.next().unwrap().parse::<usize>().unwrap();
        let z = split_comma.next().unwrap().parse::<usize>().unwrap();
        Self { x, y, z }
    }

    fn project(&self) -> Pos2 {
        Pos2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Brick {
    fn from_string(s: &str) -> Self {
        let mut split_tilde = s.split('~');
        let pos1 = Pos3::from_string(split_tilde.next().unwrap());
        let pos2 = Pos3::from_string(split_tilde.next().unwrap());
        Self { pos1, pos2 }
    }

    fn blocks(&self) -> Vec<Pos3> {
        let mut out = Vec::new();
        for x in self.pos1.x..=self.pos2.x {
            for y in self.pos1.y..=self.pos2.y {
                for z in self.pos1.z..=self.pos2.z {
                    out.push(Pos3 { x, y, z });
                }
            }
        }
        out
    }

    fn move_down(&mut self, z: usize) {
        let height = self.pos2.z - self.pos1.z;
        self.pos1.z = z;
        self.pos2.z = z + height;
    }
}

fn find_new_z(brick: &Brick, brick_map: &HashMap<Pos2, Vec<usize>>, bricks: &Vec<Brick>) -> usize {
    brick
        .blocks()
        .iter()
        .flat_map(|pos3| brick_map.get(&pos3.project()).unwrap())
        .map(|&brick_index| bricks[brick_index].pos2.z + 1)
        .filter(|&z| z <= brick.pos1.z)
        .max()
        .unwrap_or(1)
}

fn find_num_falling_bricks(
    brick_index: usize,
    supports: &Vec<HashSet<usize>>,
    supported_by: &Vec<HashSet<usize>>,
) -> usize {
    let mut falling_bricks = HashSet::new();
    falling_bricks.insert(brick_index);

    let mut queue = vec![brick_index];
    while let Some(i) = queue.pop() {
        for j in &supports[i] {
            if supported_by[*j].iter().all(|k| falling_bricks.contains(k)) {
                falling_bricks.insert(*j);
                queue.push(*j);
            }
        }
    }
    falling_bricks.len()-1
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day22/src/input.txt")?;
    let mut bricks = Vec::new();
    let mut brick_map: HashMap<Pos2, Vec<usize>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let brick = Brick::from_string(line);
        brick
            .blocks()
            .iter()
            .map(|pos3| pos3.project())
            .for_each(|pos| brick_map.entry(pos).or_default().push(i));
        bricks.push(brick);
    }
    let mut updates = Vec::new();
    loop {
        for (i, brick) in bricks.iter().enumerate() {
            let new_z = find_new_z(brick, &brick_map, &bricks);
            if new_z != brick.pos1.z {
                updates.push((i, new_z));
            }
        }
        if updates.is_empty() {
            break;
        }

        println!("Updating {} bricks", updates.len());
        for (i, new_z) in updates.drain(..) {
            bricks[i].move_down(new_z);
        }
    }

    let mut supports: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
    let mut supported_by: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
    for (i, brick) in bricks.iter().enumerate() {
        for j in brick
            .blocks()
            .iter()
            .flat_map(|pos3| brick_map.get(&pos3.project()).unwrap())
            .filter(|&&j| j != i)
        {
            let other_brick = &bricks[*j];
            if other_brick.pos1.z == brick.pos2.z + 1 {
                supports[i].insert(*j);
                supported_by[*j].insert(i);
                // println!("Brick {} supports {}", i, j);
            }
        }
    }

    let mut num_will_fall = 0;

    // for (i, supported_inds) in supports.iter().enumerate() {
    //     let num_fall = supported_inds
    //         .iter()
    //         .filter(|&j| supported_by[*j].len() == 1)
    //         .count();
    //     println!(
    //         "Disentigrating {} will cause {} bricks to fall",
    //         i, num_fall
    //     );
    //     // println!("Brick {} can be disentigrated", i);
    //     num_will_fall += num_fall;
    // }
    for i in 0..bricks.len() {
        num_will_fall += find_num_falling_bricks(i, &supports, &supported_by);
    }

    // println!("Number of disentigratable bricks: {}", num_disentigratable);
    println!("Number of bricks that will fall: {}", num_will_fall);

    Ok(())
}

// I think we actually need to compute the final configuration, and then compute which
// brick supports which brick.
// To compute this we can do the following. For each brick, check how much empty space
// is below it using the projection hashmap. This is the maximum z-coordinate of the
// bricks below it. (Or zero).
// Then we move the brick down by that amount.
// We repeat the procedure until all bricks are in their resting position.

// For part two: we have to use the two 'supports' and 'supported_by' to recursively
// Find the bricks that will fall. And this really can be recursive; we can use a hashmap
// If a bricks falls, and it supports another brick uniquely, that one will also fall.

// No it's not recursive. But we start with a hashset of bricks that will fall. Then for
// each brick on top of that one, we check if it's supported only by bricks in the set,
// if so it also falls and is added to the set. We maintain a queue of bricks to check.
