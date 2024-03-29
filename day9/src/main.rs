use anyhow::Result;
// use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn extract_line(line: &str) -> Vec<i32> {
    let mut nums = Vec::new();
    for s in line.split(' ') {
        nums.push(s.parse::<i32>().unwrap());
    }
    nums
}

fn diff(input: Vec<i32>) -> (Vec<i32>, bool) {
    let mut diff: Vec<i32> = Vec::new();
    let mut all_zero: bool = true;
    for i in 0..input.len() - 1 {
        let new_val = input[i + 1] - input[i];
        if new_val != 0 {
            all_zero = false;
        }
        diff.push(new_val);
    }
    (diff, all_zero)
}

fn make_predict(input: &[i32]) -> i32 {
    let mut all_zero = input.iter().filter(|&x| *x != 0).count() == 0;
    // let mut tower: Vec<Vec<i32>> = vec![input.to_vec()];
    let mut val = input.to_vec();
    let mut first_vals = vec![input[0]];
    while !all_zero {
        let (new_diff, new_all_zero) = diff(val);
        first_vals.push(new_diff[0]);
        val = new_diff;
        all_zero = new_all_zero;
    }
    // println!("{:?}", first_vals);
    first_vals.iter().rev().fold(0, |acc, x| x - acc)
}

fn main() -> Result<()> {
    let file = File::open("day9/src/input.txt")?;
    let reader = BufReader::new(file);
    let input_vecs: Vec<Vec<i32>> = reader.lines().map(|l| extract_line(&l.unwrap())).collect();

    let sum = input_vecs.iter().map(|x| make_predict(x)).sum::<i32>();
    println!("Sum {}", sum);

    Ok(())
}
