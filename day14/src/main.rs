use std::ops::AddAssign;

use anyhow::Result;

#[derive(Clone, Debug)]
struct State {
    start: usize,
    num_rocks: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<State>>, usize) {
    let width = input.lines().next().unwrap().len();
    let mut out: Vec<Vec<State>> = vec![
        vec![State {
            start: 0,
            num_rocks: 0
        }];
        width
    ];
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        height += 1;
        for (col, char) in line.chars().enumerate() {
            match char {
                'O' => out[col].last_mut().unwrap().num_rocks.add_assign(1),
                '.' => {}
                '#' => out[col].push(State {
                    start: row+1,
                    num_rocks: 0,
                }),
                _ => panic!("Invalid char {}", char),
            }
        }
    }

    (out, height)
}

// The first element is worth height-start, then height-start-1 ..., height-start-num_rocks+1
// So it's the sum

fn count_weight(states: &Vec<State>, height: usize) -> usize {
    let mut sum = 0;
    for state in states {
        let val = (0..state.num_rocks)
            .map(|x| height - state.start - x)
            .sum::<usize>();
        sum += val;
        // println!("{:?} {}", state, val);
    }

    sum
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day14/src/input.txt")?;
    let (all_states, height) = parse_input(&input);
    let sum:usize = all_states.iter().map(|states| count_weight(states, height)).sum();
    // println!("{:?}", all_states);
    println!("{}", sum);


    Ok(())
}
