use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = File::open("day6/src/input.txt")?;
    let reader = BufReader::new(file);

    let re_number = Regex::new(r"(\d+)").unwrap();
    let mut lines = reader.lines();
    let times: Vec<u32>;
    let distances: Vec<u32>;
    if let Some(line) = lines.next() {
        times = re_number
            .find_iter(&line?)
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();
    } else {
        return Err(anyhow!("Empty line"));
    }
    if let Some(line) = lines.next() {
        distances = re_number
            .find_iter(&line?)
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();
    } else {
        return Err(anyhow!("Empty line"));
    }

    println!("{:?}", times);
    println!("{:?}", distances);

    let output: u32 = times
        .iter()
        .zip(distances.iter())
        .map(|(race_time, distance)| compute_c_range(*race_time, *distance))
        .product();
    println!("Output: {}", output);

    

    Ok(())
}

// We have variables race_time, charge_time
// The distance is given by charge_time*(race_time-charge_time) = c(r-c)
// We need to find the range of values such that  cr-c^2 > d
// According to wolfram alpha we get
// 1/2 (r-sqrt(r^2-4d) < c < 1/2 (sqrt(r^2-4d) + r)

fn compute_c_range(race_time: u32, distance: u32) -> u32 {
    let r = race_time as f32;
    let d = distance as f32;
    let discriminant = (r.powi(2) - 4.0 * d).sqrt();
    let mut lower = (r - discriminant) / 2.0;
    if lower.ceil() == lower {
        lower += 1.0
    }
    let lower_int = lower.ceil() as u32;
    let mut upper = (r + discriminant) / 2.0;
    if upper == upper.floor() {
        upper -= 1.0
    }
    let upper_int = upper.floor() as u32;
    println!("lower: {}, upper: {}", lower_int, upper_int);

    upper_int - lower_int + 1
}
