use anyhow::Result;

#[derive(Debug)]

enum Direction {
    R,
    L,
    U,
    D,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: usize,
    // color: String,
}

impl Instruction {
    fn from_string(s: &str) -> Option<Self> {
        let mut split = s.split(' ');
        let direction = match split.next()? {
            "R" => Direction::R,
            "L" => Direction::L,
            "U" => Direction::U,
            "D" => Direction::D,
            _ => return None,
        };

        let steps = split.next()?.parse::<usize>().ok()?;

        Some(Self { direction, steps })
    }

    fn from_string_part2(s: &str) -> Option<Self> {
        let mut split = s.split(' ');
        split.next();
        split.next();
        let color = split.next()?;
        let color = color[1..color.len() - 1].to_string();
        let steps = usize::from_str_radix(&color[1..color.len() - 1], 16).ok()?;
        let direction = match color.chars().last()? {
            '0' => Direction::R,
            '1' => Direction::D,
            '2' => Direction::L,
            '3' => Direction::U,
            _ => return None,
        };

        Some(Self { direction, steps })
    }

    fn move_in_direction(&self, x: i64, y: i64) -> (i64, i64) {
        match self.direction {
            Direction::R => (x + self.steps as i64, y),
            Direction::L => (x - self.steps as i64, y),
            Direction::U => (x, y - self.steps as i64),
            Direction::D => (x, y + self.steps as i64),
        }
    }
}

fn cross_prod_2d(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0 * b.1 - a.1 * b.0
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day18/src/input.txt")?;
    let instructions = input
        .lines()
        .map(Instruction::from_string_part2)
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let mut iter = instructions.iter();
    let instruction = iter.next().unwrap();

    let mut prev_coord = instruction.move_in_direction(0, 0);

    let mut sum = instruction.steps as i64;
    for instruction in iter {
        let coord = instruction.move_in_direction(prev_coord.0, prev_coord.1);
        sum += cross_prod_2d(prev_coord, coord);
        sum += instruction.steps as i64;
        prev_coord = coord;
    }

    println!("{}", sum.abs() / 2 + 1);

    Ok(())
}
