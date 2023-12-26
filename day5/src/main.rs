use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct MapEntry {
    destination_start: usize,
    source_start: usize,
    size: usize,
}

fn apply_map(input: usize, map: &Vec<MapEntry>) -> usize {
    for entry in map {
        if input >= entry.source_start && input < entry.source_start + entry.size {
            return input - entry.source_start + entry.destination_start;
        }
    }

    input
}

fn extract_map_entry(line: &str, re_number: &Regex) -> Result<MapEntry> {
    let mut iter = re_number.find_iter(line);
    let destination_start: usize = iter.next().ok_or(anyhow!(""))?.as_str().parse()?;
    let source_start: usize = iter.next().ok_or(anyhow!(""))?.as_str().parse()?;
    let size: usize = iter.next().ok_or(anyhow!(""))?.as_str().parse()?;

    Ok(MapEntry {
        destination_start,
        source_start,
        size,
    })
}

fn main() -> Result<()> {
    let re_number: Regex = Regex::new(r"\d+").unwrap();

    let file = File::open("day5/src/input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap()?;
    let mut seeds: Vec<usize> = re_number
        .find_iter(&first_line)
        .map(|x| x.as_str().parse().unwrap())
        .collect();
    println!("{:?}", seeds);

    let mut maps: Vec<Vec<MapEntry>> = Vec::new();

    let mut current_vec: Vec<MapEntry> = Vec::new();

    for line in lines {
        let maybe_entry = extract_map_entry(&line?, &re_number);
        if let Ok(entry) = maybe_entry {
            current_vec.push(entry);
        } else if !current_vec.is_empty() {
            // println!("{:?}", current_vec);
            maps.push(current_vec);
            current_vec = Vec::new();
        }
    }
    // println!("{:?}", current_vec);
    maps.push(current_vec);

    for map in &maps {
        seeds = seeds.iter().map(|x| apply_map(*x, map)).collect();
        println!("{:?}; min={}", seeds, seeds.iter().min().unwrap());
    }

    Ok(())
}
