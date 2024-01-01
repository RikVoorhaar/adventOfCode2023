use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Condition {
    Broken,
    Unknown,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Group {
    entries: Vec<Condition>,
}

impl Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for &c in self.entries.iter() {
            s.push(match c {
                Condition::Broken => '#',
                Condition::Unknown => '?',
            })
        }

        write!(f, "'{}'", s)
    }
}

impl Group {
    fn first_broken(&self) -> Option<usize> {
        self.entries.iter().position(|&c| c == Condition::Broken)
    }

    fn is_wildcard(&self) -> bool {
        self.entries.iter().all(|&c| c == Condition::Unknown)
    }

    fn from_string(s: &str) -> Self {
        let mut entries: Vec<Condition> = Vec::new();
        for char in s.chars() {
            let condition = match char {
                '#' => Condition::Broken,
                '?' => Condition::Unknown,
                _ => panic!("Unknown char '{}'", char),
            };
            entries.push(condition)
        }

        Group { entries }
    }

    fn consume(&self, num: usize) -> Vec<Group> {
        let mut new_groups: Vec<Group> = Vec::new();
        let num_entries = self.entries.len();
        if num_entries < num {
            return new_groups;
        }

        let mut max_start = num_entries - num;
        if let Some(first) = self.first_broken() {
            if first < max_start {
                max_start = first
            }
        }
        for start in 0..max_start + 1 {
            if let Some(Condition::Broken) = self.entries.get(start + num) {
                continue;
            }
            if start + num == num_entries {
                new_groups.push(Group {
                    entries: Vec::new(),
                })
            } else {
                new_groups.push(Group {
                    entries: self.entries[start + num + 1..].to_vec(),
                });
            }
        }

        new_groups
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Record {
    groups: Vec<Group>,
    numbers: Vec<usize>,
}

impl Record {
    fn all_wildcard(&self) -> bool {
        self.groups.iter().all(|group| group.is_wildcard())
    }

    fn consume(&mut self) -> Vec<Record> {
        let group = self.groups.pop();
        if group.is_none() {
            return Vec::new();
        }
        let group = group.unwrap();
        let num = self.numbers.pop().unwrap();
        let mut new_records: Vec<Record> = Vec::new();

        if group.is_wildcard() {
            let mut new_numbers = self.numbers.clone();
            new_numbers.push(num);
            new_records.push(Record {
                groups: self.groups.clone(),
                numbers: new_numbers,
            });
        }

        if num > group.entries.len() {
            return new_records;
        }
        let new_groups = group.consume(num);
        for group in new_groups {
            let mut groups = self.groups.clone();
            if !group.entries.is_empty() {
                groups.push(group);
            }
            new_records.push(Record {
                groups,
                numbers: self.numbers.clone(),
            })
        }

        new_records
    }
}

fn find_num_records_recursive(record: &Record, cache: &mut HashMap<Record, usize>) -> usize {
    if let Some(&num) = cache.get(record) {
        return num;
    }

    let mut num = 0;

    let new_records = record.clone().consume();
    for record in new_records {
        if record.numbers.is_empty() {
            if record.all_wildcard() {
                num += 1
            }
        } else {
            num += find_num_records_recursive(&record, cache);
        }
    }

    cache.insert(record.clone(), num);

    num
}

fn parse_line(line: &str) -> Record {
    let repeat = 5;
    let mut split = line.split(' ');
    let condition_part = split.next().unwrap();
    let condition_unfolded = std::iter::repeat(condition_part)
        .take(repeat)
        .collect::<Vec<&str>>()
        .join("?");
    println!("condition_unfolded {}", condition_unfolded);
    let groups_strings = condition_unfolded.split('.');
    let groups = groups_strings
        .map(Group::from_string)
        .filter(|group| !group.entries.is_empty())
        .rev()
        .collect::<Vec<Group>>();

    let mut num_vector: Vec<usize> = Vec::new();
    for num in split.next().unwrap().split(',').rev() {
        num_vector.push(num.parse::<usize>().unwrap())
    }
    println!("{} | {:?} | {:?}", line, groups, num_vector);

    Record {
        groups,
        numbers: num_vector.repeat(repeat),
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day12/src/input.txt")?;
    let records: Vec<Record> = input.lines().map(parse_line).collect();
    let nums = records
        .iter()
        .map(|record| find_num_records_recursive(record, &mut HashMap::new()))
        .collect::<Vec<usize>>();
    println!("nums {:?}", nums);
    println!("sum {:?}", nums.iter().sum::<usize>());

    Ok(())
}
