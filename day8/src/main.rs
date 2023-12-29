use anyhow::Result;
use num::integer::lcm;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: &str) -> (String, (String, String)) {
    let mut split = line.split(" = ");
    let key = split.next().unwrap();
    let mut value_split = split.next().unwrap().split(", ");
    let value1 = &value_split.next().unwrap()[1..];
    let value2 = &value_split.next().unwrap()[..3];
    (key.to_string(), (value1.to_string(), value2.to_string()))
}

fn determine_path_length(
    network: &HashMap<String, (String, String)>,
    instructions: &str,
    start_position: &str,
) -> usize {
    let mut position = start_position;
    let mut z_positions = Vec::new();
    let mut seen = HashMap::new();
    let num_instructions = instructions.len();

    let mut loop_count = 0;

    for (i, instr) in instructions.chars().cycle().enumerate() {
        let i_pos = i % num_instructions;
        let (left, right) = &network[position];
        position = match instr {
            'L' => left,
            'R' => right,
            _ => position,
        };
        if seen.contains_key(&(position, i_pos)) {
            let prev_i = seen[&(position, i_pos)];
            let cycle_length = i - prev_i;
            return cycle_length;
            let cycle_pos = i % cycle_length;

            let z_rel: Vec<usize> = z_positions.iter().map(|s| s % cycle_length).collect();
            println!(
                "Loop detected at, i_pos={},  i={}, cycle_length={}, cycle_pose={},prev_i={}: {:?}",
                i_pos, i, cycle_length, cycle_pos, prev_i, z_rel
            );
            seen = HashMap::new();

            loop_count += 1;
        }
        if loop_count > 5 {
            break;
        }
        seen.insert((position, i_pos), i);

        if position.ends_with('Z') {
            z_positions.push(i + 1);
        }
    }
    0
}

fn main() -> Result<()> {
    let file = File::open("day8/src/input.txt")?;
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
    let start_positions: Vec<String> = network
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .collect();
    println!("{:?}", start_positions);
    let path_lenghts = start_positions
        .iter()
        .map(|s| determine_path_length(&network, instructions, s) as i64)
        .collect::<Vec<i64>>();
    println!("{:?}", path_lenghts);
    let insr_len = instructions.len() as i64;
    let lcm = path_lenghts.iter().fold(insr_len, |acc, x| lcm(acc, *x/insr_len));
    println!("{}", lcm);

    Ok(())
}

// It turns out that the Z-s always occur at exactly `n*cycle_length` steps. So we just
// need the gcd of the cycle lengths
