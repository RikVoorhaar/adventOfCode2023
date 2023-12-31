use anyhow::Result;
use core::num;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::SubAssign;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Condition {
    Working,
    Broken,
    Unknown,
}

#[derive(Clone)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    damaged: Vec<usize>,
}
impl Debug for ConditionRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for &c in self.conditions.iter() {
            s.push(match c {
                Condition::Working => '.',
                Condition::Broken => '#',
                Condition::Unknown => '?',
            })
        }
        write!(f, "{}", s)
    }
}

impl ConditionRecord {
    fn check_consistent(&self) -> bool {
        let mut run = 0;
        let mut num_unknown = 0;
        let mut runs = Vec::new();
        for &condition in self.conditions.iter() {
            match condition {
                Condition::Broken => run += 1,
                Condition::Working => {
                    if run > 0 {
                        runs.push(run)
                    }
                    run = 0
                }
                Condition::Unknown => {
                    num_unknown += 1;
                    if run > 0 {
                        runs.push(run)
                    }
                    run = 0
                }
            }
        }
        if run > 0 {
            runs.push(run)
        }

        if num_unknown == 0 {
            return runs == self.damaged;
        }

        // println!("{:?}, runs: {:?}, damaged: {:?}", self, runs, self.damaged);
        let mut damaged_iter = self.damaged.iter();
        // let mut num_missed = 0;
        let current_ = damaged_iter.next();
        if current_.is_none() {
            return false;
        }
        let mut current = *current_.unwrap();

        let mut run_total = 0;
        for run in runs {
            run_total += run;
            while run > current {
                // num_missed += *current.unwrap();
                let next_current = damaged_iter.next();
                if next_current.is_none() {
                    return false;
                }
                current = *next_current.unwrap();
            }
            current -= run;

            // num_missed += *current.unwrap() - run;
        }
        let damaged_total = self.damaged.iter().sum::<usize>();
        run_total <= damaged_total && damaged_total <= run_total + num_unknown
        // num_missed <= num_unknown
    }

    fn mutate_condition(&self, pos: usize, cond: Condition) -> Self {
        let mut new_conditions = self.conditions.clone();
        new_conditions[pos] = cond;
        ConditionRecord {
            conditions: new_conditions,
            damaged: self.damaged.clone(),
        }
    }

    fn next_unknown_pos(&self) -> Option<usize> {
        self.conditions
            .iter()
            .position(|&c| c == Condition::Unknown)
    }
}

fn find_num_records(record: &ConditionRecord) -> usize {
    let mut num = 0;

    let mut queue: Vec<ConditionRecord> = vec![record.clone()];
    while let Some(record) = queue.pop() {
        if let Some(pos) = record.next_unknown_pos() {
            let record1 = record.mutate_condition(pos, Condition::Broken);
            if record1.check_consistent() {
                queue.push(record1)
            } else {
            }
            let record2 = record.mutate_condition(pos, Condition::Working);
            if record2.check_consistent() {
                queue.push(record2)
            } else {
            }
        } else {
            num += 1
        }
    }

    println!("---------- {}", num);
    num
}

fn parse_line(line: &str) -> ConditionRecord {
    let mut split = line.split(' ');
    let mut condition_vector: Vec<Condition> = Vec::new();
    let condition_part = split.next().unwrap();
    let condition_unfolded = std::iter::repeat(condition_part)
        .take(5)
        .collect::<Vec<&str>>()
        .join("?");
    println!("condition_unfolded {}", condition_unfolded);

    for char in condition_unfolded.chars() {
        let condition = match char {
            '#' => Condition::Broken,
            '.' => Condition::Working,
            '?' => Condition::Unknown,
            _ => panic!("Unknown char '{}'", char),
        };
        condition_vector.push(condition)
    }
    let mut num_vector: Vec<usize> = Vec::new();
    for num in split.next().unwrap().split(',') {
        num_vector.push(num.parse::<usize>().unwrap())
    }

    ConditionRecord {
        conditions: condition_vector,
        damaged: num_vector.repeat(5),
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day12/src/example.txt")?;
    let records: Vec<ConditionRecord> = input.lines().map(parse_line).collect();
    let nums = records.iter().map(find_num_records).collect::<Vec<usize>>();
    println!("nums {:?}", nums);
    println!("sum {:?}", nums.iter().sum::<usize>());

    Ok(())
}
