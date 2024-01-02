use anyhow::Result;
use std::collections::HashMap;

fn hash(state: u8, ch: char) -> u8 {
    state.wrapping_add(ch as u8).wrapping_mul(17)
}
fn hash_string(input: &str) -> usize {
    let state = 0u8;
    input.chars().fold(state, hash) as usize
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct MapEntry {
    counter: usize,
    focal_length: u8,
}

#[derive(Clone, Debug)]
struct Box {
    insertion_counter: usize,
    lenses: HashMap<String, MapEntry>,
}

struct Boxes {
    boxes: [Box; 256],
}
impl Boxes {
    fn insert(&mut self, input: &str) {
        if let Some((label, focal_length)) = input.split_once('=') {
            let hash = hash_string(label);
            let lens = Lens {
                label: label.to_string(),
                focal_length: focal_length.parse::<u8>().unwrap(),
            };
            println!("Inserting into box {} lens {:?}", hash, lens);
            self.boxes[hash].add_lens(lens);
            return;
        }

        if let Some((label, _)) = input.split_once('-') {
            let hash = hash_string(label);
            println!("Removing from box {} lens {}", hash, label);
            self.boxes[hash].remove_lens(label)
        }
    }
}

impl Box {
    fn new() -> Box {
        Box {
            insertion_counter: 0,
            lenses: HashMap::new(),
        }
    }

    fn add_lens(&mut self, lens: Lens) {
        let entry = self.lenses.remove(&lens.label).unwrap_or({
            self.insertion_counter += 1;
            MapEntry {
                counter: self.insertion_counter,
                focal_length: 0,
            }
        });
        self.lenses.insert(
            lens.label,
            MapEntry {
                counter: entry.counter,
                focal_length: lens.focal_length,
            },
        );
    }
    fn remove_lens(&mut self, label: &str) {
        self.lenses.remove(label);
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day15/src/example.txt")?;
    let mut boxes = Boxes {
        boxes: (0..256)
            .map(|_| Box::new())
            .collect::<Vec<Box>>()
            .try_into()
            .unwrap(),
    };
    for piece in input.split(',') {
        boxes.insert(piece);
    }

    Ok(())
}
