use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct MapEntry {
    tgt: usize,
    src: usize,
    len: usize,
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

fn _apply_map(input: usize, map: &Vec<MapEntry>) -> usize {
    for entry in map {
        if input >= entry.src && input < entry.src + entry.len {
            return input - entry.src + entry.tgt;
        }
    }

    input
}

fn apply_map_to_ranges(input: Vec<Range>, map: &MapEntry, output: &mut Vec<Range>) -> Vec<Range> {
    let mut new_input: Vec<Range> = Vec::new();
    // let range be [a,b]
    // let map be [c, d]
    // We can order all elements, and consider all cases
    // 1. [a, b, c, d] -> no overlap -> new_input is [a, b], output is nothing
    // 2. [a, c, b, d] -> overlap [c, b] -> new input is [a,c], output is [c, b]
    // 3. [a ,c, d, b] -> overlap [c, d] -> new input is [a,c]+[d,b], output is [c,d]
    // 4. [c, a, b, d] -> overlap [a, b] -> new input is [], output is [a, b]
    // 5. [c, a, d, b] -> overlap [a, d] -> new input is [d, b], output is [a, d]
    // 6. [c, d, a, b] -> no overlap -> new input is [a,b], output is nothing
    let c = map.src;
    let d = map.src + map.len;

    for range in input {
        let a = range.start;
        let b = range.end;
        println!("a={}, b={}, c={}, d={}", a, b, c, d);

        if b>=c && d >= a {
            // 2,3,4,5
            if a < c {
                // 2,3
                if b < d {
                    // 2
                    if a != c {
                        new_input.push(Range { start: a, end: c });
                    }
                    let len = b- c;
                    if len != 0 {
                        output.push(Range {
                            start: map.tgt,
                            end: map.tgt + len,
                        })
                    }
                } else {
                    // 3
                    if a != c {
                        new_input.push(Range { start: a, end: c });
                    }
                    if d != b {
                        new_input.push(Range { start: d, end: b });
                    }
                    output.push(Range {
                        start: map.tgt,
                        end: map.tgt + map.len,
                    })
                }
            } else {
                // 4, 5
                if b < d {
                    // 4
                    let len = b - a;
                    let offset = a - c;
                    output.push(Range {
                        start: map.tgt + offset,
                        end: map.tgt + offset + len,
                    })
                } else {
                    // 5
                    if d != b {
                        new_input.push(Range { start: d, end: b });
                    }
                    let offset = a - c;
                    if offset != map.len {
                        output.push(Range {
                            start: map.tgt + offset,
                            end: map.tgt + map.len,
                        })
                    }
                }
            }
        } else {
            new_input.push(range)
        }
    }

    new_input
}

fn apply_maps_to_ranges(input: Vec<Range>, maps: &Vec<MapEntry>) -> Vec<Range> {
    let mut output: Vec<Range> = Vec::new();
    let mut new_input = input;
    for entry in maps {
        new_input = apply_map_to_ranges(new_input, entry, &mut output);
        println!("entry = {:?}, new_input = {:?}, output = {:?}", entry, new_input, output);
    }
    for range in new_input {
        output.push(range)
    }

    output
}

fn extract_map_entry(line: &str, re_number: &Regex) -> Result<MapEntry> {
    let mut iter = re_number.find_iter(line);
    let destination_start: usize = iter.next().ok_or(anyhow!(""))?.as_str().parse()?;
    let source_start: usize = iter.next().ok_or(anyhow!(""))?.as_str().parse()?;
    let size: usize = iter.next().ok_or(anyhow!(""))?.as_str().parse()?;

    Ok(MapEntry {
        tgt: destination_start,
        src: source_start,
        len: size,
    })
}

fn main() -> Result<()> {
    let re_number: Regex = Regex::new(r"\d+").unwrap();

    let file = File::open("day5/src/input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap()?;

    let entries: Vec<usize> = re_number
        .find_iter(&first_line)
        .map(|x| x.as_str().parse().unwrap())
        .collect();
    let mut ranges: Vec<Range> = entries
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            end: chunk[0] + chunk[1],
        })
        .collect();
    println!("{:?}", ranges);

    let mut map_vecs: Vec<Vec<MapEntry>> = Vec::new();

    let mut current_vec: Vec<MapEntry> = Vec::new();

    for line in lines {
        let maybe_entry = extract_map_entry(&line?, &re_number);
        if let Ok(entry) = maybe_entry {
            current_vec.push(entry);
        } else if !current_vec.is_empty() {
            map_vecs.push(current_vec);
            current_vec = Vec::new();
        }
    }
    map_vecs.push(current_vec);

    for maps in &map_vecs {
        ranges = apply_maps_to_ranges(ranges, maps);
        println!("{:?}", ranges)
    }

    // for map in &maps {
    //     seeds = seeds.iter().map(|x| apply_map(*x, map)).collect();
    //     println!("{:?}; min={}", seeds, seeds.iter().min().unwrap());
    // }
    let min = ranges.iter().map(|x| x.start).min();
    println!("min = {:?}", min);

    Ok(())
}
