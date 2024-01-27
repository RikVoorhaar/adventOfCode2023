#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use anyhow::Result;

struct Hailstone3 {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

impl Hailstone3 {
    fn from_str(s: &str) -> Self {
        let mut s = s.split(" @ ");
        let position = s
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<_>>();
        let next = s.next().unwrap();
        let velocity = next
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<_>>();
        Self {
            position: (position[0], position[1], position[2]),
            velocity: (velocity[0], velocity[1], velocity[2]),
        }
    }

    fn project(&self) -> Hailstone2 {
        Hailstone2 {
            position: (self.position.0, self.position.1),
            velocity: (self.velocity.0, self.velocity.1),
        }
    }
}

impl Debug for Hailstone3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @ {:?}", self.position, self.velocity)
    }
}

struct Hailstone2 {
    position: (i64, i64),
    velocity: (i64, i64),
}
impl Debug for Hailstone2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @ {:?}", self.position, self.velocity)
    }
}

fn intersection_point(stone1: &Hailstone2, stone2: &Hailstone2) -> Option<(f64, f64)> {
    let (x1, y1) = stone1.position;
    let (x2, y2) = stone2.position;
    let (vx1, vy1) = stone1.velocity;
    let (vx2, vy2) = stone2.velocity;

    let vel_cross = (vx1 * vy2 - vx2 * vy1) as f64;

    if vel_cross == 0.0 {
        // parallel
        return None;
    }

    // Intersection means that (x1+vx1*t, y1+vy1*t) == (x2+vx2*s, y2+vy2*s)
    let t = (vx2 * y1 - vx2 * y2 - vy2 * x1 + vy2 * x2) as f64 / vel_cross;
    let s = -(vx1 * y2 - vx1 * y1 - vy1 * x2 + vy1 * x1) as f64 / vel_cross;

    if t < 0.0 || s < 0.0 {
        return None;
    }

    let x = x1 as f64 + vx1 as f64 * t;
    let y = y1 as f64 + vy1 as f64 * t;

    Some((x, y))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day24/src/input.txt")?;
    let hailstones = input
        .lines()
        .map(|l| Hailstone3::from_str(l).project())
        .collect::<Vec<_>>();
    println!("{:?}", hailstones);

    // let (min_pos, max_pos) = (7.0, 27.0);
    let (min_pos, max_pos) = (200000000000000.0, 400000000000000.0);
    let mut num_intersect = 0;
    for (i, hailstone) in hailstones.iter().enumerate() {
        for other in &hailstones[i + 1..] {
            if let Some((x, y)) = intersection_point(hailstone, other) {
                if x >= min_pos && x <= max_pos && y >= min_pos && y <= max_pos {
                    num_intersect += 1;
                }
            }
        }
    }
    println!("num_intersect = {}", num_intersect);

    Ok(())
}
