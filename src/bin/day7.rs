use std::fs::File;
use std::io::{self, BufRead};

// memoized count of ways from (row,col) to bottom
fn count_ways(
    original: &Vec<Vec<char>>,
    memo: &mut Vec<Vec<Option<i64>>>,
    row: usize,
    col: usize,
) -> i64 {
    let rows = original.len();
    let cols = original[0].len();
    if row >= rows || col >= cols {
        return 0;
    }
    if let Some(v) = memo[row][col] {
        return v;
    }
    if row == rows - 1 {
        memo[row][col] = Some(1);
        return 1;
    }

    let mut res: i64 = 0;
    if original[row + 1][col] == '^' {
        if col >= 1 {
            res += count_ways(original, memo, row + 1, col - 1);
        }
        if col + 1 < cols {
            res += count_ways(original, memo, row + 1, col + 1);
        }
    } else {
        res += count_ways(original, memo, row + 1, col);
    }

    memo[row][col] = Some(res);
    res
}

// mark reachable cells from top starts following movement rules (but only along paths that lead to bottom)
fn mark_reachable(
    original: &Vec<Vec<char>>,
    memo: &Vec<Vec<Option<i64>>>,
    reachable: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
) {
    let rows = original.len();
    let cols = original[0].len();
    if row >= rows || col >= cols {
        return;
    }
    if reachable[row][col] {
        return;
    }
    // if this cell can't reach bottom, skip
    if memo[row][col].unwrap_or(0) == 0 {
        return;
    }

    reachable[row][col] = true;
    if row == rows - 1 {
        return;
    }

    if original[row + 1][col] == '^' {
        if col >= 1 {
            mark_reachable(original, memo, reachable, row + 1, col - 1);
        }
        if col + 1 < cols {
            mark_reachable(original, memo, reachable, row + 1, col + 1);
        }
    } else {
        mark_reachable(original, memo, reachable, row + 1, col);
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day7.txt")?;
    let reader = io::BufReader::new(file);

    let mut input_matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        input_matrix.push(line.chars().collect());
    }

    let rows = input_matrix.len();
    let cols = input_matrix[0].len();

    let mut memo: Vec<Vec<Option<i64>>> = vec![vec![None; cols]; rows];

    // compute ways from top row starts
    for col in 0..cols {
        if input_matrix[0][col] == '.' {
            continue;
        }
        // fill memo for this start
        let _ = count_ways(&input_matrix, &mut memo, 0, col);
    }

    let mut num_possible_timelines: i64 = 0;
    for col in 0..cols {
        num_possible_timelines += memo[0][col].unwrap_or(0);
    }

    // mark reachable cells along successful paths and count unique splits
    let mut reachable: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    for col in 0..cols {
        if memo[0][col].unwrap_or(0) > 0 {
            mark_reachable(&input_matrix, &memo, &mut reachable, 0, col);
        }
    }

    let mut num_splits: i64 = 0;
    for r in 0..rows - 1 {
        for c in 0..cols {
            if reachable[r][c] && input_matrix[r + 1][c] == '^' {
                num_splits += 1;
            }
        }
    }

    println!("Number of splits: {}", num_splits);
    println!("Number of possible timelines: {}", num_possible_timelines);
    Ok(())
}