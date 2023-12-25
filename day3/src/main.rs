use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn _process_line(
    line_prev: Option<&str>,
    line_now: &str,
    line_next: Option<&str>,
    re_number: &Regex,
    re_symbol: &Regex,
) -> Result<u32> {
    let mut sum = 0;

    for cap in re_number.find_iter(line_now) {
        let num = cap.as_str().parse::<u32>()?;
        let start = cap.start().saturating_sub(1);
        let end = std::cmp::min(cap.end() + 1, line_now.len());
        if let Some(line_prev) = line_prev {
            if re_symbol.is_match(&line_prev[start..end]) {
                sum += num;
            }
        }
        if re_symbol.is_match(&line_now[start..end]) {
            sum += num;
        }

        if let Some(line_next) = line_next {
            if re_symbol.is_match(&line_next[start..end]) {
                sum += num;
            }
        }
    }
    Ok(sum)
}
fn record_gear(
    line_number: usize,
    line_start: usize,
    num: u32,
    line_slice: &str,
    gear_map: &mut HashMap<(usize, usize), Vec<u32>>,
) {
    if let Some(index) = line_slice.find('*') {
        let vec = gear_map
            .entry((line_number, line_start + index))
            .or_default();
        vec.push(num);
        println!("({},{}): {:?}", line_number, line_start + index, vec);
    }
}
fn process_line(
    line_prev: Option<&str>,
    line_now: &str,
    line_next: Option<&str>,
    re_number: &Regex,
    line_number: usize,
    gear_map: &mut HashMap<(usize, usize), Vec<u32>>,
) -> Result<()> {
    for cap in re_number.find_iter(line_now) {
        let num = cap.as_str().parse::<u32>()?;
        let start = cap.start().saturating_sub(1);
        let end = std::cmp::min(cap.end() + 1, line_now.len());
        if let Some(line_prev) = line_prev {
            record_gear(
                line_number - 1,
                start,
                num,
                &line_prev[start..end],
                gear_map,
            );
        }
        record_gear(line_number, start, num, &line_now[start..end], gear_map);

        if let Some(line_next) = line_next {
            record_gear(
                line_number + 1,
                start,
                num,
                &line_next[start..end],
                gear_map,
            );
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let re_number: Regex = Regex::new(r"\d+").unwrap();
    // let re_symbol: Regex = Regex::new(r"[^\w\s\d.]").unwrap();

    let file = File::open("day3/src/input.txt")?;
    let reader = BufReader::new(file);

    let mut line_prev: Option<String> = None;
    let mut line_now: Option<String> = None;
    let gear_map = &mut HashMap::new();

    let mut num_lines = 0;
    for (line_number, line_result) in reader.lines().enumerate() {
        let line_next = line_result?;

        if let Some(ref line_now_str) = line_now {
            process_line(
                line_prev.as_deref(),
                line_now_str,
                Some(&line_next),
                &re_number,
                line_number,
                gear_map,
            )?
        }

        line_prev = line_now.take();
        line_now = Some(line_next);
        num_lines = line_number;
    }

    process_line(
        line_prev.as_deref(),
        &line_now.unwrap(),
        None,
        &re_number,
        num_lines + 1,
        gear_map,
    )?;

    let mut sum = 0;
    for nums in gear_map.clone().into_values() {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
