use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PulseWidth {
    Low,
    High,
}

impl Debug for PulseWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
        }
    }
}

// impl PulseWidth {
//     fn flip(&self) -> Self {
//         match self {
//             Self::Low => Self::High,
//             Self::High => Self::Low,
//         }
//     }
// }

struct Pulse {
    width: PulseWidth,
    target: String,
    source: String,
}

impl Debug for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{:?}-> {}", self.source, self.width, self.target)
    }
}

enum ModuleType {
    FlipFlop,
    Conjunction,
}

enum FlipFlipState {
    On,
    Off,
}

impl FlipFlipState {
    fn flip(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

struct FlipFlopModule {
    name: String,
    targets: Vec<String>,
    state: FlipFlipState,
}

impl FlipFlopModule {
    fn from_string(s: &str) -> Option<Self> {
        let mut split = s.split(" -> ");
        let name = split.next()?.to_string();

        let targets: Vec<String> = split.next()?.split(", ").map(|s| s.to_string()).collect();

        Some(Self {
            name,
            targets,
            state: FlipFlipState::Off,
        })
    }

    fn handle_pulse(&mut self, pulse: PulseWidth) -> Vec<Pulse> {
        match pulse {
            PulseWidth::High => Vec::new(),
            PulseWidth::Low => {
                let output = match self.state {
                    FlipFlipState::Off => PulseWidth::High,
                    FlipFlipState::On => PulseWidth::Low,
                };

                self.state = self.state.flip();
                self.targets
                    .iter()
                    .map(|t| Pulse {
                        width: output,
                        source: self.name.clone(),
                        target: t.clone(),
                    })
                    .collect()
            }
        }
    }
}
struct ConjunctionModule {
    name: String,
    targets: Vec<String>,
    state: HashMap<String, PulseWidth>,
}

impl ConjunctionModule {
    fn from_string(s: &str) -> Option<Self> {
        let mut split = s.split(" -> ");
        let name = split.next()?.to_string();

        let targets: Vec<String> = split.next()?.split(", ").map(|s| s.to_string()).collect();

        Some(Self {
            name,
            targets,
            state: HashMap::new(),
        })
    }

    fn add_source(&mut self, source: String) {
        self.state.insert(source, PulseWidth::Low);
    }

    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.state.insert(pulse.source, pulse.width);
        let all_high = !self.state.values().any(|p| *p == PulseWidth::Low);
        let output = match all_high {
            true => PulseWidth::Low,
            false => PulseWidth::High,
        };
        self.targets
            .iter()
            .map(|t| Pulse {
                width: output,
                source: self.name.clone(),
                target: t.clone(),
            })
            .collect()
    }
}

fn get_broadcaster_targets(s: &str) -> Vec<String> {
    s.split(" -> ")
        .nth(1)
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect()
}

fn simulate(
    flip_flops: &mut HashMap<String, FlipFlopModule>,
    conjunctions: &mut HashMap<String, ConjunctionModule>,
    broadcast_targets: &Vec<String>,
    cycle_num: usize,
) -> Vec<String> {
    let mut pulses: VecDeque<Pulse> = VecDeque::from(
        broadcast_targets
            .iter()
            .map(|t| Pulse {
                width: PulseWidth::Low,
                source: "broadcaster".to_string(),
                target: t.clone(),
            })
            .collect::<Vec<Pulse>>(),
    );

    let mut out = Vec::new();

    while let Some(pulse) = pulses.pop_front() {
        if pulse.target == "hb" && pulse.width == PulseWidth::High {
            println!(
                "hb input high at cycle {} from pulse {:?}",
                cycle_num, pulse
            );
            out.push(pulse.source.clone());
        }
        if flip_flops.contains_key(&pulse.target) {
            let flip_flop = flip_flops.get_mut(&pulse.target).unwrap();
            let new_pulses = flip_flop.handle_pulse(pulse.width);
            pulses.extend(new_pulses);
        } else if conjunctions.contains_key(&pulse.target) {
            let conjunction = conjunctions.get_mut(&pulse.target).unwrap();
            let new_pulses = conjunction.handle_pulse(pulse);
            pulses.extend(new_pulses);
        }
    }
    out
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day20/src/input.txt")?;
    let mut flip_flops: HashMap<String, FlipFlopModule> = HashMap::new();
    let mut conjunctions: HashMap<String, ConjunctionModule> = HashMap::new();

    let mut broadcast_targets: Vec<String> = Vec::new();

    let mut source_target_pairs: Vec<(String, String)> = Vec::new();

    for line in input.lines() {
        let first_char = line.chars().next().unwrap();
        match first_char {
            '%' => {
                let module = FlipFlopModule::from_string(&line[1..]);
                if let Some(module) = module {
                    // println!(
                    //     "flip flop. name {}, targets: {:?}",
                    //     module.name, module.targets
                    // );
                    for target in module.targets.iter() {
                        source_target_pairs.push((module.name.clone(), target.clone()));
                    }
                    flip_flops.insert(module.name.clone(), module);
                }
            }
            '&' => {
                let module = ConjunctionModule::from_string(&line[1..]);
                if let Some(module) = module {
                    // println!(
                    //     "conjunction. name {}, targets: {:?}",
                    //     module.name, module.targets
                    // );
                    for target in module.targets.iter() {
                        source_target_pairs.push((module.name.clone(), target.clone()));
                    }
                    conjunctions.insert(module.name.clone(), module);
                }
            }
            _ => {
                broadcast_targets.extend(get_broadcaster_targets(line));
            }
        }
    }
    println!("broadcast_targets: {:?}", broadcast_targets);

    for (source, target) in source_target_pairs {
        if let Some(conjunction) = conjunctions.get_mut(&target) {
            conjunction.add_source(source);
        }
    }
    let mut high_inputs: HashMap<String, usize> = HashMap::new();
    for i in 0.. {
        let hb_high_inputs = simulate(&mut flip_flops, &mut conjunctions, &broadcast_targets, i);
        for input in hb_high_inputs {
            high_inputs.insert(input.clone(), i+1);
        }
        if high_inputs.len() == 4 {
            break;
        }
    }
    println!("product: {}", high_inputs.values().product::<usize>());

    Ok(())
}
