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

fn _create_map() -> HashMap<&'static str, u32> {
    let mut map = HashMap::new();
    map.insert("red", 12);
    map.insert("green", 13);
    map.insert("blue", 14);
    map
}

fn _process_line1(line: &str, map: &HashMap<&str, u32>) -> Result<u32> {
    let space_index = line.find(' ').ok_or_else(|| anyhow!("error"))?;
    let colon_index = line.find(':').ok_or_else(|| anyhow!("error"))?;
    let game_number = extract_number(&line[space_index + 1..colon_index])?;
    let rest = &line[colon_index + 2..];

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
fn process_line2(line: &str) -> Result<u32> {
    // let space_index = line.find(' ').ok_or_else(|| anyhow!("error"))?;
    let colon_index = line.find(':').ok_or_else(|| anyhow!("error"))?;
    // let game_number = extract_number(&line[space_index + 1..colon_index])?;
    let rest = &line[colon_index + 2..];

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for part in rest.split("; ") {
        for word in part.split(", ") {
            let space_index = word.find(' ').ok_or_else(|| anyhow!("error"))?;
            let number = extract_number(&word[..space_index])?;
            let color = &word[space_index + 1..];
            if color == "red" && number > max_red {
                max_red = number;
            } else if color == "green" && number > max_green {
                max_green = number;
            } else if color == "blue" && number > max_blue {
                max_blue = number;
            }
        }
    }
    Ok(max_blue * max_green * max_red)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    // let map = create_map1();

    let mut sum = 0;

    for line in reader.lines() {
        let val = process_line2(&line?).unwrap_or(0);
        sum += val;
    }
    println!("sum: {}", sum);

    Ok(())
}
