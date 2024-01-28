#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

// const SCALE_FACTOR: f64 = 1e-36;
const SCALE_FACTOR: f64 = 1e-0;
mod vect3;
use vect3::Vect3;

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

    fn to_vect3(&self) -> (Vect3, Vect3) {
        (
            Vect3 {
                x: self.position.0 as f64,
                y: self.position.1 as f64,
                z: self.position.2 as f64,
            },
            Vect3 {
                x: self.velocity.0 as f64,
                y: self.velocity.1 as f64,
                z: self.velocity.2 as f64,
            },
        )
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
// implements: dot-product, norm^2, cross product. Then we need to gather these vectors
// from the input, define the loss function and get cracking. First of course confirm
// the gradient with finite differences.
//
// I guess it works? I havent' verified yet with finite differences, but at least it
// seems to find a descent direction. I do need to implement line search though, because
// fixed step size isn't working.

// Ok, so the problem is not well-conditioned, and the linear approach simply doesn't
// work. We can try ALS; I suspect that if we keep y or w fixed that the problem becomes
// linear.

fn loss(x: Vect3, v: Vect3, y: Vect3, w: Vect3) -> f64 {
    let cross = (x - y).cross(&(v - w));
    cross.norm2()
}

/// \|a\cross b\|^2 = \|a\|^2 \|b\|^2 - (a\cdot b)^2
// fn loss2(x: Vect3, v: Vect3, y: Vect3, w: Vect3) -> f64 {
//     let a = x - y;
//     let b = v - w;
//     let a_dot_b = a.dot(&b);
//     let a_norm2 = a.norm2();
//     let b_norm2 = b.norm2();
//     a_norm2 * b_norm2 - a_dot_b * a_dot_b
// }
fn grad(x: Vect3, v: Vect3, y: Vect3, w: Vect3) -> (Vect3, Vect3) {
    let a = x - y;
    let b = v - w;
    let a_dot_b = a.dot(&b);
    let a_norm2 = a.norm2();
    let b_norm2 = b.norm2();

    let grad_y = 2.0 * b_norm2 * a - 2.0 * a_dot_b * b;
    let grad_w = 2.0 * a_norm2 * b - 2.0 * a_dot_b * a;

    (-grad_y, -grad_w)
}

fn grad_old(x: Vect3, v: Vect3, y: Vect3, w: Vect3) -> (Vect3, Vect3) {
    // let cross = (x - y).cross(&(v - w));
    println!("v-w = {:?}", v - w);
    println!("x-y = {:?}", x - y);
    let grad_y = 2.0 * (x - y) * (v - w).norm2() - (v - w);
    let grad_w = 2.0 * (v - w) * (x - y).norm2() - (x - y);
    (grad_y, grad_w)
}

fn part1(hailstones: Vec<Hailstone2>, min_pos: i64, max_pos: i64) -> usize {
    let mut num_intersect = 0;
    for (i, hailstone) in hailstones.iter().enumerate() {
        for other in &hailstones[i + 1..] {
            if let Some((x, y)) = intersection_point(hailstone, other) {
                if x >= min_pos as f64
                    && x <= max_pos as f64
                    && y >= min_pos as f64
                    && y <= max_pos as f64
                {
                    num_intersect += 1;
                }
            }
        }
    }
    num_intersect
}

fn full_loss_grad(y: Vect3, w: Vect3, hailstones: &Vec<(Vect3, Vect3)>) -> (f64, Vect3, Vect3) {
    let mut l = 0.0;
    let mut y_grad = Vect3::zero();
    let mut w_grad = Vect3::zero();

    for &(x, v) in hailstones {
        l += loss(x, v, y, w);
        let (y_grad_, w_grad_) = grad(x, v, y, w);
        y_grad += y_grad_;
        w_grad += w_grad_;
    }

    (
        l * SCALE_FACTOR,
        y_grad * SCALE_FACTOR,
        w_grad * SCALE_FACTOR,
    )
}

fn full_loss(y: Vect3, w: Vect3, hailstones: &Vec<(Vect3, Vect3)>) -> f64 {
    let mut l = 0.0;

    for &(x, v) in hailstones {
        l += loss(x, v, y, w);
    }

    l * SCALE_FACTOR
}

fn armijo_condition(
    f_old: f64,
    f_new: f64,
    armijo_constant: f64,
    grad_dot_search: f64,
    step_size: f64,
) -> bool {
    f_new <= f_old - step_size * armijo_constant * grad_dot_search
}

fn armijo_step_size(
    loss_prev: f64,
    stepsize_prev: f64,
    y: Vect3,
    w: Vect3,
    search_y: Vect3,
    search_w: Vect3,
    grad_dot_search: f64,
    hailstones: &Vec<(Vect3, Vect3)>,
) -> f64 {
    let mut stepsize = stepsize_prev * 2.0;
    // let grad_dot_search = grad_y.dot(&grad_y_norm) + grad_w.dot(&grad_w_norm);
    // println!("search_dot_grad = {:.2e}", grad_dot_search);
    // println!("grad_y_norm = {:?}", grad_y_norm);
    // println!("grad_w_norm = {:?}", grad_w_norm);

    while !armijo_condition(
        loss_prev,
        full_loss(y - stepsize * search_y, w - stepsize * search_w, hailstones),
        1e-4,
        grad_dot_search,
        stepsize,
    ) {
        stepsize /= 1.5;
    }
    stepsize
}

/// Compute the gradient of the loss function using finite differences
fn fin_diff_grad(x: Vect3, v: Vect3, y: Vect3, w: Vect3, eps: f64) -> (Vect3, Vect3) {
    let mut y_grad_fin = Vect3::zero();
    let mut w_grad_fin = Vect3::zero();
    let l = loss(x, v, y, w);

    for i in 0..6 {
        let mut y_eps = y;
        let mut w_eps = w;
        if i < 3 {
            y_eps[i] += eps;
        } else {
            w_eps[i - 3] += eps;
        }
        let l_eps = loss(x, v, y_eps, w_eps);
        if i < 3 {
            y_grad_fin[i] = (l_eps - l) / eps;
        } else {
            w_grad_fin[i - 3] = (l_eps - l) / eps;
        }
    }

    (y_grad_fin, w_grad_fin)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day24/src/input.txt")?;
    let hailstones = input
        .lines()
        .map(|l| Hailstone3::from_str(l).to_vect3())
        .collect::<Vec<_>>();
    println!("{:?}", hailstones);

    let mut y = Vect3::zero();
    let mut w = Vect3::zero();

    // let (x, v) = hailstones[0];
    // let l = loss(x, v, y, w);
    // let (y_grad, w_grad) = grad(x, v, y, w);
    // let (y_grad2, w_grad2) = grad2(x, v, y0, w0);
    // let (y_grad_fin, w_grad_fin) = fin_diff_grad(x, v, y, w, 1e-4);
    // println!("l = {:.2e}", l);
    // println!("y_grad = {:?}", y_grad);
    // println!("y_grad2 = {:?}", y_grad2);
    // println!("y_grad_fin = {:?}", y_grad_fin);
    // println!("w_grad = {:?}", w_grad);
    // println!("w_grad2 = {:?}", w_grad2);

    // println!("w_grad_fin = {:?}", w_grad_fin);

    //

    let mut step_size_y = 1.0;
    let mut step_size_w = 1.0;
    for _ in 0..1000 {
        let (l, grad_y, _) = full_loss_grad(y, w, &hailstones);
        let search_y = grad_y * (1.0 / grad_y.norm2().sqrt());
        let grad_dot_search_y = grad_y.dot(&search_y);
        step_size_y = armijo_step_size(
            l,
            step_size_y,
            y,
            w,
            search_y,
            Vect3::zero(),
            grad_dot_search_y,
            &hailstones,
        );
        y += -step_size_y * search_y;

        let (l, _, grad_w) = full_loss_grad(y, w, &hailstones);
        let search_w = grad_w * (1.0 / grad_w.norm2().sqrt());
        let grad_dot_search_w = grad_w.dot(&search_w);

        step_size_w = armijo_step_size(
            l,
            step_size_w,
            y,
            w,
            Vect3::zero(),
            search_w,
            grad_dot_search_w,
            &hailstones,
        );
        w += -step_size_w * search_w;
        let sum = y.x + y.y + y.z;
        println!(
            "l = {:.2e}, step_size_y= {:.2e}, step_size_w= {:.2e}, y_grad_norm = {:.2e}, w_grad_norm = {:.2e}, y = {:?}, w = {:?}, sum: {}",
            l,
            step_size_y,
            step_size_w,
            grad_y.norm2(),
            grad_w.norm2(),
            y,
            w,
            sum
        );
    }

    Ok(())
}
