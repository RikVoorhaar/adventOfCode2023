use anyhow::Result;

fn line_to_u8_slice(line: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for c in line.chars() {
        out.push(match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("Invalid char {}", c),
        })
    }

    out
}

fn u8_vec_to_int(input: Vec<u8>) -> u32 {
    let mut out: u32 = 0;
    let mut acc: u32 = 1;
    for x in input {
        if x != 0 {
            out += acc;
        }
        acc *= 2;
    }
    out
}

fn u8_array_to_int_vec(input: Vec<Vec<u8>>) -> Vec<u32> {
    let mut out = vec![0; input[0].len()];
    let mut acc = 1;
    for row in input {
        for (i, &x) in row.iter().enumerate() {
            if x != 0 {
                out[i] += acc;
            }
        }
        acc *= 2;
    }

    out
}

fn find_value(vec: Vec<u32>) -> Option<usize> {
    for i in 0..vec.len() - 1 {
        if vec[i] == vec[i + 1] {
            let len = std::cmp::min(i + 1, vec.len() - i - 1);
            // println!("{},{}",i+1,row.len()-i);
            let mut reversed = vec[i + 1..i + len + 1].to_vec();
            reversed.reverse();
            if reversed == vec[i + 1 - len..i + 1].to_vec() {
                return Some(i + 1);
            }
            // println!("{}, {:?}\t{:?}", i+1, row[i+1-len..i+1].to_vec(), reversed);
        }
    }

    None
}
fn equal_up_to_smudge(vec1: &Vec<u32>, vec2: &Vec<u32>) -> bool {
    let mut smudge_found: bool = false;
    for (x, y) in vec1.iter().zip(vec2.iter()) {
        if x != y {
            if smudge_found {
                return false;
            }
            let diff = match x > y {
                true => x - y,
                false => y - x,
            };
            if diff & (diff - 1) == 0 {
                smudge_found = true;
            } else {
                return false;
            }
        }
    }
    smudge_found
}

fn equal_up_power_2(x: u32, y: u32) -> bool {
    let diff = match x > y {
        true => x - y,
        false => y - x,
    };
    diff & (diff - 1) == 0
}

fn find_value_smudge(vec: Vec<u32>) -> Option<usize> {
    for i in 0..vec.len() - 1 {
        if vec[i] == vec[i + 1] || equal_up_power_2(vec[i], vec[i + 1])  {
            let len = std::cmp::min(i + 1, vec.len() - i - 1);
            // println!("{},{}",i+1,row.len()-i);
            let mut reversed = vec[i + 1..i + len + 1].to_vec();
            reversed.reverse();
            let normal = vec[i + 1 - len..i + 1].to_vec();
            if equal_up_to_smudge(&reversed, &normal) {
                println!("{} {:?} {:?}", i + 1, reversed, normal);
                return Some(i + 1);
            }
            // println!("{}, {:?}\t{:?}", i+1, row[i+1-len..i+1].to_vec(), reversed);
        }
    }

    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day13/src/input.txt")?;

    let mut arrays: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut current_array = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            println!("array: {:?}", current_array);
            arrays.push(current_array);
            current_array = Vec::new();
            continue;
        }
        current_array.push(line_to_u8_slice(line));
    }
    println!("array: {:?}", current_array);
    arrays.push(current_array);
    let mut row_values = Vec::new();
    let mut col_values = Vec::new();
    for array in arrays {
        row_values.push(
            array
                .iter()
                .map(|x| u8_vec_to_int(x.clone()))
                .collect::<Vec<u32>>(),
        );
        col_values.push(u8_array_to_int_vec(array));
    }
    println!("row_values: {:?}", row_values);
    println!("col_values: {:?}", col_values);

    let mut sum = 0;
    for (row, col) in row_values.iter().zip(col_values.iter()) {
        if let Some(num) = find_value_smudge(col.clone()) {
            sum += num
        }
        if let Some(num) = find_value_smudge(row.clone()) {
            sum += num * 100
        }
    }
    println!("sum: {}", sum);

    Ok(())
}
