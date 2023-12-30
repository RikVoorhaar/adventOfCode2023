use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

fn distance(galaxy1: (usize, usize), galaxy2: (usize, usize)) -> i64 {
    let x = (galaxy1.0 as i64 - galaxy2.0 as i64).abs();
    let y = (galaxy1.1 as i64 - galaxy2.1 as i64).abs();

    x + y
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day11/src/input.txt")?;
    let mut occopied_rows: HashSet<usize> = HashSet::new();
    let mut occopied_cols: HashSet<usize> = HashSet::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut galaxy_positions: Vec<(usize, usize)> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                occopied_cols.insert(x);
                occopied_rows.insert(y);
                galaxy_positions.push((x, y))
            }
        }
    }
    let expansion_coeff = 1_000_000;
    let empty_cols: Vec<usize> = (0..width)
        .map(|x| (!occopied_cols.contains(&x) as usize) * (expansion_coeff - 1))
        .scan(0, |state, s| {
            *state += s;
            Some(*state)
        })
        .collect();
    let empty_rows: Vec<usize> = (0..height)
        .map(|y| (!occopied_rows.contains(&y) as usize) * (expansion_coeff - 1))
        .scan(0, |state, s| {
            *state += s;
            Some(*state)
        })
        .collect();
    println!("Empty cols {:?}", empty_cols);
    println!("Empty rows {:?}", empty_rows);
    println!("Galaxy before moving: {:?}", galaxy_positions);
    galaxy_positions = galaxy_positions
        .iter_mut()
        .map(|(x, y)| (*x + empty_cols[*x], *y + empty_rows[*y]))
        .collect();
    println!("Galaxy after moving: {:?}", galaxy_positions);

    let mut sum = 0;
    for (i, &galaxy1) in galaxy_positions.iter().enumerate() {
        for &galaxy2 in galaxy_positions.iter().skip(i + 1) {
            sum += distance(galaxy1, galaxy2);
        }
    }
    println!("Sum: {}", sum);

    Ok(())
}
