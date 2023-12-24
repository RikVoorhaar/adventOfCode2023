use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn extract_number(input: &str) -> Result<u32> {
    let iter = input.chars().filter(|&c| c.is_ascii_digit());
    let mut digits = String::new();
    for c in iter {
        digits.push(c);
    }
    digits.parse::<u32>().map_err(|_| anyhow!("error"))
}

fn create_map() -> HashMap<&'static str, u32> {
    let mut map = HashMap::new();
    map.insert("red", 12);
    map.insert("green", 13);
    map.insert("blue", 14);
    map
}

fn process_line(line: &str, map: &HashMap<&str, u32>) -> Result<u32> {
    let space_index = line.find(' ').ok_or_else(|| anyhow!("error"))?;
    let colon_index = line.find(':').ok_or_else(|| anyhow!("error"))?;
    let game_number = extract_number(&line[space_index + 1..colon_index])?;
    let rest = &line[colon_index + 2..];

    // println!("game_number: {}", game_number);
    // println!("rest: {}", rest);

    for part in rest.split("; ") {
        // println!("part: {}", part);
        for word in part.split(", ") {
            // println!("word: '{}'", word);
            let space_index = word.find(' ').ok_or_else(|| anyhow!("error"))?;
            // println!("space_index: {}", space_index);
            let number = extract_number(&word[..space_index])?;
            let color = &word[space_index + 1..];
            let color_max = map.get(color).ok_or_else(|| anyhow!("error"))?;
            // println!("number: {}, color: {}", number, color);
            if number > *color_max {
                // println!("number: {}, color: {}, part: {}", number, color, part);
                return Ok(0);
            }
        }
    }

    Ok(game_number)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let map = create_map();

    let mut sum = 0;

    for line in reader.lines() {
        sum += process_line(&line?, &map).unwrap_or(0);
    }
    println!("sum: {}", sum);

    Ok(())
}
