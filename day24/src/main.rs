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

// Now for the problem of the 3d hailstones in part2.
// This is basically an interger programming problem. we have a series of equations
// x[i,j] + v[i,j] * t = y[j] + w[j] * t
// Which we need to solve for y and w and t
// In the end we can put all of this in one big matrix, and just do Guassian elimination
// I suppose. It is rather surprising that a solution exists at all.

// Well the above is not quite true, since the equation isn't linear. But the existence
// of a solution (in t) to the equation x[i] + v[i]*t = y + w * t puts a constraint on y
//  and w in terms of x[i] and v[i]. We need to figure out what that constraint is. One
// way to phrase it is that (x[i]-y) and (v[i]-w) are parallel, which means their cross
// product is zero. Unfortunately that's not a linear equation in (y,w), because it
// containes products of y and w.
// Nevertheless we can expand the cross product, because it is distributive
// 0 = x[i] X v[i] - x[i] X w - y X v[i] + y X w

// Yeah, that's not very useful, because it's not linear.
// But maybe we can see it as an optimization problem. The norm of this cross product
// could be a loss function. As long as its derivative is easy to compute we can just
// use gradient descent or something like that.

// There is Lagrange's identity which might help us out. It states that
// \|a\cross b\|^2 = \|a\|^2 \|b\|^2 - (a\cdot b)^2
// with in our case a = (x[i]-y) and b = (v[i]-w)
// That's not a verry pretty function to differentiate, but it's not impossible
//
// The gradient should be, w.r.t. y:
// 2(x-y)\|v-w\|^2-(v-w)
//
// w.r.t. w:
// 2(v-w)\|x-y\|^2-(x-y)

// That's non-linear but not hard to implement. To start we need a vect3 struct which
// implements: dot-product, norm^2, cross product.
// Then we need to gather these vectors from the input, define the loss function
// and get cracking. First of course confirm the gradient with finite differences.

struct Vect3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Debug for Vect3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl Vect3 {
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
