use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn process_line(line: &str) -> Result<u32> {
    let mut iter = line.chars().filter(|&c| c.is_ascii_digit());
    let mut digits = String::new();
    let first = iter.next();
    if let Some(ch) = first {
        digits.push(ch);
    }

    let last = iter.last().or(first);
    if let Some(ch) = last {
        digits.push(ch);
    }
    digits.parse::<u32>().map_err(Into::into)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        sum += process_line(&line?).unwrap_or(0);
    }
    println!("sum: {}", sum);

    Ok(())
}
