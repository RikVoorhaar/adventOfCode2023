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

struct Brick {
    pos1: Pos3,
    pos2: Pos3,
}

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
}


fn main() -> Result<()> {
    let input = std::fs::read_to_string("day22/src/example.txt")?;
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

    let mut supports: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
    for (i, brick) in bricks.iter().enumerate() {
        for j in brick
            .blocks()
            .iter()
            .flat_map(|pos3| brick_map.get(&pos3.project()).unwrap())
            .filter(|&&j| j != i)
        {
            let other_brick = &bricks[*j];
            if other_brick.pos1.z > brick.pos2.z {
                supports[i].insert(*j);
                println!("Brick {} supports {}", i, j);
            }
        }
    }

    Ok(())
}

// I think we actually need to compute the final configuration, and then compute which
// brick supports which brick.  
// To compute this we can do the following. For each brick, check how much empty space
// is below it using the projection hashmap. This is the maximum z-coordinate of the
// bricks below it. (Or zero).
// Then we move the brick down by that amount.
// We repeat the procedure until all bricks are in their resting position.