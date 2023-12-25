use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn extract_numbers_to_set(input: &str, re_number: &Regex) -> HashSet<u32> {
    let mut numbers = HashSet::new();
    for cap in re_number.find_iter(input) {
        let num = cap.as_str().parse::<u32>().unwrap();
        numbers.insert(num);
    }

    numbers
}

fn _process_line(line: &str, re_number: &Regex) -> Result<u32> {
    let colon_index = line.find(':').unwrap();
    let pipe_index = line.find('|').unwrap();

    let winning_numbers = extract_numbers_to_set(&line[colon_index + 1..pipe_index], re_number);
    let our_numbers = extract_numbers_to_set(&line[pipe_index + 1..], re_number);
    let intersection_num = winning_numbers.intersection(&our_numbers).count() as u32;

    match intersection_num {
        0 => Ok(0),
        1 => Ok(1),
        x => Ok(2u32.pow(x - 1)),
    }
}

fn process_line(
    line: &str,
    re_number: &Regex,
    count_hashmap: &mut HashMap<usize, u32>,
) -> Result<u32> {
    let colon_index = line.find(':').unwrap();
    let pipe_index = line.find('|').unwrap();

    let winning_numbers = extract_numbers_to_set(&line[colon_index + 1..pipe_index], re_number);
    let our_numbers = extract_numbers_to_set(&line[pipe_index + 1..], re_number);
    let intersection_num = winning_numbers.intersection(&our_numbers).count() as usize;

    let card_number = re_number
        .find(&line[..colon_index])
        .ok_or(anyhow!("parsing error"))?
        .as_str()
        .parse::<usize>()
        .unwrap();

    let self_count = *count_hashmap.entry(card_number).or_insert(1u32);
    for i in card_number + 1..card_number + 1 + intersection_num {
        let entry = count_hashmap.entry(i).or_insert(1u32);
        *entry += self_count
    }

    Ok(self_count)
}

fn main() -> Result<()> {
    let re_number: Regex = Regex::new(r"\d+").unwrap();
    let mut count_hashmap = HashMap::new();

    let file = File::open("day4/src/example.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let val = process_line(&line?, &re_number, &mut count_hashmap)?;
        sum += val;
    }

    println!("sum: {}", sum);
    Ok(())
}
