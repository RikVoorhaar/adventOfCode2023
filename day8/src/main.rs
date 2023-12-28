use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const START: &str = "AAA";
const END: &str = "ZZZ";

fn parse_line(line: &str) -> (String, (String, String)) {
    let mut split = line.split(" = ");
    let key = split.next().unwrap();
    let mut value_split = split.next().unwrap().split(", ");
    let value1 = &value_split.next().unwrap()[1..];
    let value2 = &value_split.next().unwrap()[..3];
    (key.to_string(), (value1.to_string(), value2.to_string()))
}

fn main() -> Result<()> {
    let file = File::open("day8/src/example2.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let instructions: &str = &lines.next().unwrap().unwrap();
    println!("{}", instructions);

    let mut network: HashMap<String, (String, String)> = HashMap::new();

    lines.next();
    for line_ in lines {
        let line = line_.unwrap();
        let (key, value) = parse_line(&line);
        network.insert(key, value);
    }
    let mut position = START;
    for (i,instr) in instructions.chars().cycle().enumerate() {
        let (left,right) = &network[position];
        // println!("{}: {}. ({}, {})", i, position, left, right);
        match instr {
            'L' => position = left,
            'R' => position = right,
            _ => (),
        }
        
        if position == END {
            println!("Found end at {}", i+1);
            break;
        }
    }

    Ok(())
}
