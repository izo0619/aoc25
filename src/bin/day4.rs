use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("day4.txt")?;
    let reader = io::BufReader::new(file);

    let mut input_matrix: Vec<Vec<char>> = Vec::new();
    
    
    for line in reader.lines() {
        let line = line?;
        input_matrix.push(line.chars().collect());
    }

    let mut copy_matrix: Vec<Vec<char>> = input_matrix.clone();

    let mut count = 0;
    let mut removed_none= false;

    while !removed_none {
        let mut removed_some = false;
        for i in 0..input_matrix.len() {
            for j in 0..input_matrix[i].len() {
                if input_matrix[i][j] == '@' {
                    // check 8 directions
                    let directions = [(-1, -1), (-1, 0), (-1, 1),
                                    (0, -1),          (0, 1),
                                    (1, -1), (1, 0), (1, 1)];
                    let mut num_neighbors = 0;
                    for (di, dj) in directions.iter() {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0 && ni < input_matrix.len() as isize &&
                        nj >= 0 && nj < input_matrix[i].len() as isize {
                            if input_matrix[ni as usize][nj as usize] == '@' {
                                num_neighbors += 1;
                            }
                        }
                    }
                    if num_neighbors < 4 {
                        count += 1;
                        input_matrix[i][j] = '.';
                        removed_some = true;
                    }
                    
                }
            }
        }
        if !removed_some {
            removed_none = true;
        }
    }
    println!("Count of '@' with fewer than 4 '@' neighbors: {}", count);
    // for row in copy_matrix {
    //     let line: String = row.iter().collect();
    //     println!("{}", line);
    // }
    Ok(())
}