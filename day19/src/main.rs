use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

enum Attribute {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn from_string(s: &str) -> Option<Self> {
        let regex = Regex::new(r"(\b\d+\b)").ok()?;
        let mut matches = regex.find_iter(s);
        let x = matches.next()?.as_str().parse::<i32>().ok()?;
        let m = matches.next()?.as_str().parse::<i32>().ok()?;
        let a = matches.next()?.as_str().parse::<i32>().ok()?;
        let s = matches.next()?.as_str().parse::<i32>().ok()?;

        Some(Self { x, m, a, s })
    }

    fn score(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

enum Inequality {
    GreaterThan,
    LessThan,
}

struct Condition {
    attribute: Attribute,
    inequality: Inequality,
    value: i32,
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
        let value = rest.parse::<i32>().ok()?;

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

    let mut parts: Vec<Part> = Vec::new();
    for line in lines.by_ref() {
        if let Some(part) = Part::from_string(line) {
            parts.push(part);
        } else {
            break;
        }
    }

    let sum: i32 = parts
        .iter()
        .filter(|part| check_part(part, &workflows))
        .map(Part::score)
        .sum();
    println!("sum: {}", sum);

    Ok(())
}
