use anyhow::Result;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    position: Position,
    direction: Direction,
    straight_steps: usize,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct StateScore {
    state: State,
    score: usize,
    heuristic_score: usize,
}

impl PartialOrd for StateScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Some(self.heuristic_score.cmp(&other.heuristic_score))
        Some(other.heuristic_score.cmp(&self.heuristic_score))
    }
}

impl Ord for StateScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.heuristic_score.cmp(&other.heuristic_score)
        other.heuristic_score.cmp(&self.heuristic_score)
    }
}

fn step_state(state: &State, size_x: usize, size_y: usize) -> Option<State> {
    let new_pos = match state.direction {
        Direction::Right => Position {
            x: state.position.x + 1,
            y: state.position.y,
        },
        Direction::Left => Position {
            x: state.position.x - 1,
            y: state.position.y,
        },
        Direction::Up => Position {
            x: state.position.x,
            y: state.position.y - 1,
        },
        Direction::Down => Position {
            x: state.position.x,
            y: state.position.y + 1,
        },
    };
    if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= size_x as i32 || new_pos.y >= size_y as i32 {
        return None;
    }

    Some(State {
        position: new_pos,
        direction: state.direction,
        straight_steps: state.straight_steps + 1,
    })
}

fn turn_state(state: &State) -> (State, State) {
    let new_dirs = match state.direction {
        Direction::Right => (Direction::Up, Direction::Down),
        Direction::Left => (Direction::Down, Direction::Up),
        Direction::Up => (Direction::Left, Direction::Right),
        Direction::Down => (Direction::Right, Direction::Left),
    };

    (
        State {
            position: state.position,
            direction: new_dirs.0,
            straight_steps: 0,
        },
        State {
            position: state.position,
            direction: new_dirs.1,
            straight_steps: 0,
        },
    )
}

fn propose_new_entries(state: State, size_x: usize, size_y: usize) -> Vec<State> {
    let mut out = Vec::new();

    if state.straight_steps < 10 {
        if let Some(new) = step_state(&state, size_x, size_y) {
            out.push(new);
        }
    }
    if state.straight_steps >= 4 {
        let (turn1, turn2) = turn_state(&state);
        if let Some(new) = step_state(&turn1, size_x, size_y) {
            out.push(new);
        }
        if let Some(new) = step_state(&turn2, size_x, size_y) {
            out.push(new);
        }
    }

    out
}

fn shortest_possible_paths(loss_map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let size_y = loss_map.len();
    let size_x = loss_map[0].len();
    let mut out = vec![vec![usize::MAX; size_x]; size_y];
    out[size_y - 1][size_x - 1] = 0;
    let mut queue: Vec<Position> = vec![Position {
        x: size_x as i32 - 1,
        y: size_y as i32 - 1,
    }];
    while let Some(pos) = queue.pop() {
        let score = out[pos.y as usize][pos.x as usize];
        if pos.y > 0 {
            let new_pos = Position {
                x: pos.x,
                y: pos.y - 1,
            };
            let new_score = score + loss_map[new_pos.y as usize][new_pos.x as usize];
            if new_score < out[new_pos.y as usize][new_pos.x as usize] {
                out[new_pos.y as usize][new_pos.x as usize] = new_score;
                queue.push(new_pos);
            }
        }
        if pos.x > 0 {
            let new_pos = Position {
                x: pos.x - 1,
                y: pos.y,
            };
            let new_score = score + loss_map[new_pos.y as usize][new_pos.x as usize];
            if new_score < out[new_pos.y as usize][new_pos.x as usize] {
                out[new_pos.y as usize][new_pos.x as usize] = new_score;
                queue.push(new_pos);
            }
        }
    }

    out
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day17/src/input.txt")?;

    let mut best_val: HashMap<State, usize> = HashMap::new();
    let mut loss_map: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let mut line_map = Vec::new();

        for ch in line.chars() {
            let val = ch.to_digit(10).unwrap() as usize;
            line_map.push(val);
        }
        loss_map.push(line_map);
    }
    let size_y = loss_map.len();
    let size_x = loss_map[0].len();

    let shortest_paths = shortest_possible_paths(&loss_map);

    let mut queue: BinaryHeap<StateScore> = BinaryHeap::from([
        StateScore {
            state: State {
                position: Position { x: 1, y: 0 },
                direction: Direction::Right,
                straight_steps: 1,
            },
            score: loss_map[0][1],
            heuristic_score: loss_map[0][1] + shortest_paths[0][1],
        },
        StateScore {
            state: State {
                position: Position { x: 0, y: 1 },
                direction: Direction::Down,
                straight_steps: 1,
            },
            score: loss_map[1][0],
            heuristic_score: loss_map[1][0] + shortest_paths[1][0],
        },
    ]);

    let mut already_seen: HashSet<StateScore> = HashSet::new();

    while let Some(state_score) = queue.pop() {
        if !already_seen.insert(state_score.clone()) {
            continue;
        }

        let state = state_score.state;
        let score = state_score.score;

        if let Some(val) = best_val.get(&state) {
            if score >= *val {
                continue;
            }
        }

        best_val.insert(state.clone(), score);

        if state.position.x == size_x as i32 - 1 && state.position.y == size_y as i32 - 1 {
            println!("Shortest path has score: {}", score);
            return Ok(());
        }

        for new_state in propose_new_entries(state, size_x, size_y) {
            let new_score =
                score + loss_map[new_state.position.y as usize][new_state.position.x as usize];
            let new_heuristic = new_score
                + shortest_paths[new_state.position.y as usize][new_state.position.x as usize];
            queue.push(StateScore {
                state: new_state,
                score: new_score,
                heuristic_score: new_heuristic,
            });
        }
    }

    Ok(())
}
