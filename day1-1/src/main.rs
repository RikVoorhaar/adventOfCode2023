use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn create_map() -> HashMap<&'static str, u32> {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map.insert("1", 1);
    map.insert("2", 2);
    map.insert("3", 3);
    map.insert("4", 4);
    map.insert("5", 5);
    map.insert("6", 6);
    map.insert("7", 7);
    map.insert("8", 8);
    map.insert("9", 9);

    map
}

fn regex_from_map(mapping: &HashMap<&'static str, u32>) -> String {
    let mut regex_pattern = String::new();
    for (k, _) in mapping.iter() {
        regex_pattern.push_str(k);
        regex_pattern.push('|');
    }
    regex_pattern.pop();
    regex_pattern
}

fn _process_line1(line: &str) -> Result<u32> {
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

fn process_line2(
    line: &str,
    map: &HashMap<&'static str, u32>,
    regex: &Regex,
    regex_reversed: &Regex,
) -> Result<u32> {
    let mut iter = regex.find_iter(line);
    let first = iter.next().map(|m| m.as_str());

    let mut digits = String::new();
    if let Some(ch) = first {
        let ch2 = map.get(ch).unwrap().to_string();
        digits.push_str(&ch2);
    }

    let binding = reverse_string(line);
    let mut iter = regex_reversed.find_iter(&binding);
    let last = iter.next().map(|m| m.as_str());
    if let Some(ch) = last {
        let ch_reverse = reverse_string(ch);
        let ch2 = map.get(ch_reverse.as_str()).unwrap().to_string();
        digits.push_str(&ch2);
    }

    digits.parse::<u32>().map_err(Into::into)
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() -> Result<(), io::Error> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let map = create_map();
    let regex_pattern = regex_from_map(&map);
    let regex = Regex::new(&regex_pattern).unwrap();
    let regex_reversed = Regex::new(&reverse_string(&regex_pattern)).unwrap();

    println!("regex: {}", regex);
    println!("regex reversed: {}", regex_reversed);

    let mut sum = 0;
    for line in reader.lines() {
        sum += process_line2(&line?, &map, &regex, &regex_reversed).unwrap_or(0);
    }
    println!("sum: {}", sum);

    Ok(())
}
