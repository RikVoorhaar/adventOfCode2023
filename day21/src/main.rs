use anyhow::Result;
use sprs::{CsMat, CsVec, TriMat};

fn pos_to_index(x: usize, y: usize, size_x: usize) -> usize {
    y * size_x + x
}

fn index_to_pos(index: usize, size_x: usize) -> (usize, usize) {
    (index % size_x, index / size_x)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day21/src/input.txt")?;

    let size_y = input.lines().count();
    let size_x = input.lines().next().unwrap().len();

    let mut adjacency_tri = TriMat::new((size_x * size_y, size_x * size_y));
    let mut table: Vec<Vec<bool>> = vec![vec![false; size_x]; size_y];
    let mut start_pos = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '#' {
                table[y][x] = true;
            }
            if c == 'S' {
                start_pos = (x, y);
            }
        }
    }
    println!("table {:?}", table);

    for y in 0..size_y {
        for x in 0..size_x {
            if !table[y][x] {
                continue;
            }
            let pos = pos_to_index(x, y, size_x);
            if y > 0 && table[y - 1][x] {
                let pos2 = pos_to_index(x, y - 1, size_x);
                adjacency_tri.add_triplet(pos, pos2, 1);
                adjacency_tri.add_triplet(pos2, pos, 1);
            }
            if y < size_y - 1 && table[y + 1][x] {
                let pos2 = pos_to_index(x, y + 1, size_x);
                adjacency_tri.add_triplet(pos, pos2, 1);
                adjacency_tri.add_triplet(pos2, pos, 1);
            }
            if x > 0 && table[y][x - 1] {
                let pos2 = pos_to_index(x - 1, y, size_x);
                adjacency_tri.add_triplet(pos, pos2, 1);
                adjacency_tri.add_triplet(pos2, pos, 1);
            }
            if x < size_x - 1 && table[y][x + 1] {
                let pos2 = pos_to_index(x + 1, y, size_x);
                adjacency_tri.add_triplet(pos, pos2, 1);
                adjacency_tri.add_triplet(pos2, pos, 1);
            }
        }
    }
    println!("Number non-zero {}", adjacency_tri.nnz());

    let mut adjacency_pow: CsMat<_> = adjacency_tri.to_csr();
    let start_index = pos_to_index(start_pos.0, start_pos.1, size_x);
    let mut result = CsVec::new(size_x * size_y, vec![start_index], vec![1]);
    // for i in 0..64 {
    //     result = &adjacency_pow * &result;
    //     result
    //         .iter_mut()
    //         .for_each(|(_, x)| *x = if *x > 0 { 1 } else { 0 });
    //     // println!(
    //     //     "{:?}",
    //     //     result
    //     //         .iter()
    //     //         .map(|(i, _)| index_to_pos(i, size_x))
    //     //         .collect::<Vec<_>>()
    //     // );
    //     println!("i {}, nnz {}", i, result.nnz());
    // }

    let mut pow = 1;
    for i in 0..64 {
        pow *= 2;
        adjacency_pow = &adjacency_pow * &adjacency_pow;
        adjacency_pow
            .data_mut()
            .iter_mut()
            .for_each(|x| *x = if *x > 0 { 1 } else { 0 });
        println!("i {}, pow {}", i, pow);
    }

    println!("Result: {:?}", result.nnz());
    // let pos1 = pos_to_index(4, 6, size_x);
    // let pos2 = pos_to_index(4, 5, size_x);
    // let adj = adjacency_pow.get(pos2, pos1);
    // println!("adj {} {}: {:?}", pos2, pos1, adj);
    // let adj = adjacency_pow.get(pos1, pos2);
    // println!("adj {} {}: {:?}", pos1, pos2, adj);

    Ok(())


    // For the next part:
    // We calculate how many steps it takes to get to each part of the map from the
    // starting point.
    // Then we calculate how many steps it takes to get to the same spot the next map
    // over from the starting point.
    // We then look for patterns. We should get a modular equation I suppose.
    // Then we can calculat how many copies of each spot on the map is reachable after 26501365
    // steps. Taking into account that it must be the right thing mod 2. 
}
