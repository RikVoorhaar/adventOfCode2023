use anyhow::Result;
use regex::Regex;
use std::{clone, collections::HashMap};

enum Attribute {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn from_string(s: &str) -> Option<Self> {
        let regex = Regex::new(r"(\b\d+\b)").ok()?;
        let mut matches = regex.find_iter(s);
        let x = matches.next()?.as_str().parse::<i64>().ok()?;
        let m = matches.next()?.as_str().parse::<i64>().ok()?;
        let a = matches.next()?.as_str().parse::<i64>().ok()?;
        let s = matches.next()?.as_str().parse::<i64>().ok()?;

        Some(Self { x, m, a, s })
    }

    fn score(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartRange {
    fn update_range(&mut self, attribute: &Attribute, range: (i64, i64)) {
        match attribute {
            Attribute::X => self.x = range,
            Attribute::M => self.m = range,
            Attribute::A => self.a = range,
            Attribute::S => self.s = range,
        }
    }

    fn get_attribute_range(&self, attribute: &Attribute) -> (i64, i64) {
        match attribute {
            Attribute::X => self.x,
            Attribute::M => self.m,
            Attribute::A => self.a,
            Attribute::S => self.s,
        }
    }

    fn size(&self) -> i64 {
        let add = 1;
        (self.x.1 - self.x.0 + add)
            * (self.m.1 - self.m.0 + add)
            * (self.a.1 - self.a.0 + add)
            * (self.s.1 - self.s.0 + add)
    }
}

enum Inequality {
    GreaterThan,
    LessThan,
}

struct Condition {
    attribute: Attribute,
    inequality: Inequality,
    value: i64,
}

fn split_range(
    range: (i64, i64),
    inequality: &Inequality,
    val: i64,
) -> (Option<(i64, i64)>, Option<(i64, i64)>) {
    // Out is (pass, fail)
    match inequality {
        Inequality::GreaterThan => {
            if val < range.0 {
                (Some(range), None)
            } else if val >= range.1 {
                (None, Some(range))
            } else {
                (Some((val + 1, range.1)), Some((range.0, val)))
            }
        }
        Inequality::LessThan => {
            if val <= range.0 {
                (None, Some(range))
            } else if val > range.1 {
                (Some(range), None)
            } else {
                (Some((range.0, val - 1)), Some((val, range.1)))
            }
        }
    }
}

impl Condition {
    fn check_condition(&self, part: &Part) -> bool {
        match self.attribute {
            Attribute::X => match self.inequality {
                Inequality::GreaterThan => part.x > self.value,
                Inequality::LessThan => part.x < self.value,
            },
            Attribute::M => match self.inequality {
                Inequality::GreaterThan => part.m > self.value,
                Inequality::LessThan => part.m < self.value,
            },
            Attribute::A => match self.inequality {
                Inequality::GreaterThan => part.a > self.value,
                Inequality::LessThan => part.a < self.value,
            },
            Attribute::S => match self.inequality {
                Inequality::GreaterThan => part.s > self.value,
                Inequality::LessThan => part.s < self.value,
            },
        }
    }

    fn split_condition(&self, part: PartRange) -> (Option<PartRange>, Option<PartRange>) {
        // Out is (pass, fail)
        let val = part.get_attribute_range(&self.attribute);
        let (range_pass, range_fail) = split_range(val, &self.inequality, self.value);
        let part1 = if range_pass.is_some() {
            let mut new_part = part.clone();
            new_part.update_range(&self.attribute, range_pass.unwrap());
            Some(new_part)
        } else {
            None
        };
        let part2 = if range_fail.is_some() {
            let mut new_part = part;
            new_part.update_range(&self.attribute, range_fail.unwrap());
            Some(new_part)
        } else {
            None
        };
        (part1, part2)
    }

    fn from_string(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        let attribute = match chars.next()? {
            'x' => Attribute::X,
            'm' => Attribute::M,
            'a' => Attribute::A,
            's' => Attribute::S,
            _ => return None,
        };
        let inequality = match chars.next()? {
            '>' => Inequality::GreaterThan,
            '<' => Inequality::LessThan,
            _ => return None,
        };
        let rest = chars.as_str();
        let value = rest.parse::<i64>().ok()?;

        Some(Self {
            attribute,
            inequality,
            value,
        })
    }
}

struct WorkflowEntry {
    condition: Option<Condition>,
    name: String,
}

impl WorkflowEntry {
    fn from_string(s: &str) -> Option<Self> {
        if s.contains(':') {
            let mut split = s.split(':');
            let condition = Condition::from_string(split.next()?)?;
            let name = split.next()?.to_string();
            Some(Self {
                condition: Some(condition),
                name,
            })
        } else {
            let name = s.to_string();
            Some(Self {
                condition: None,
                name,
            })
        }
    }
}

struct Workflow {
    name: String,
    entries: Vec<WorkflowEntry>,
}

impl Workflow {
    fn from_string(s: &str) -> Option<Self> {
        let mut split = s.split(['{', '}']);
        let name = split.next()?.to_string();
        let entries: Vec<WorkflowEntry> = split
            .next()?
            .split(',')
            .flat_map(WorkflowEntry::from_string)
            .collect();

        Some(Self { name, entries })
    }

    fn apply(&self, part: &Part) -> &str {
        for entry in &self.entries {
            if let Some(condition) = &entry.condition {
                if condition.check_condition(part) {
                    return &entry.name;
                }
            } else {
                return &entry.name;
            }
        }
        panic!("no entry found for part");
    }

    fn apply_range(&self, partrange: PartRange) -> Vec<(String, PartRange)> {
        let mut out = Vec::new();
        let mut current_range = partrange;
        for entry in &self.entries {
            if let Some(condition) = &entry.condition {
                let (pass_range, fail_range) = condition.split_condition(current_range);
                if let Some(range) = pass_range {
                    out.push((entry.name.clone(), range));
                }
                match fail_range {
                    Some(range) => current_range = range,
                    None => break,
                }
            } else {
                out.push((entry.name.clone(), current_range));
                break;
            }
        }

        out
    }
}

fn check_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut next_workflow_name = "in";
    loop {
        let workflow = workflows.get(next_workflow_name).unwrap();
        next_workflow_name = workflow.apply(part);
        if next_workflow_name == "A" {
            return true;
        }
        if next_workflow_name == "R" {
            return false;
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day19/src/input.txt")?;

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if let Some(workflow) = Workflow::from_string(line) {
            workflows.insert(workflow.name.clone(), workflow);
        } else {
            break;
        }
    }

    let mut queue: Vec<(String, PartRange)> = vec![(
        "in".to_string(),
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    )];
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    while let Some((workflow_name, part_range)) = queue.pop() {
        let workflow = workflows.get(&workflow_name).unwrap();
        let new_ranges = workflow.apply_range(part_range);
        for range in new_ranges.into_iter() {
            match range.0.as_str() {
                "A" => accepted.push(range.1),
                "R" => rejected.push(range.1),
                _ => queue.push((range.0, range.1)),
            }
        }
    }

    println!("Accepted: {:?}", accepted);
    let sum = accepted.iter().map(PartRange::size).sum::<i64>();
    println!("sum: {}", sum);

    // let mut accepted = Vec::new();
    // let mut rejected = Vec::new();

    // let mut parts: Vec<Part> = Vec::new();
    // for line in lines.by_ref() {
    //     if let Some(part) = Part::from_string(line) {
    //         parts.push(part);
    //     } else {
    //         break;
    //     }
    // }

    // let sum: i64 = parts
    //     .iter()
    //     .filter(|part| check_part(part, &workflows))
    //     .map(Part::score)
    //     .sum();
    // println!("sum: {}", sum);

    Ok(())
}
