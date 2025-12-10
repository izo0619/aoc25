use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn main() -> io::Result<()> {
    let file = File::open("day6.txt")?;
    let reader = io::BufReader::new(file);

    let mut input_matrix: Vec<Vec<char>> = Vec::new();
    
    
    for line in reader.lines() {
        let line = line?;
        // put line in as chars
        input_matrix.push(line.chars().collect());
    }
    println!("Input matrix: {:?}", input_matrix);

    // go column by column, check last char in column for operator changes, otherwise collect numbers
    let mut solutions_per_row: Vec<i64> = Vec::new();
    let num_cols = input_matrix[0].len();
    let mut cur_operator = input_matrix.last().unwrap()[0];
    let mut cur_nums: Vec<i64> = Vec::new();
    for col in 0..num_cols {
        let mut num = String::new();
        let mut operator = input_matrix.last().unwrap()[col];
        if operator != ' ' {
            cur_operator = operator;
        }
        for row in 0..input_matrix.len()-1 {
            let c = input_matrix[row][col];
            if c.is_digit(10) {
                let n = c.to_digit(10).unwrap() as i64;
                num.push_str(&n.to_string());
            }
        }
        if !num.is_empty() {
            let n: i64 = num.parse().unwrap();
            cur_nums.push(n);
        } else{
            // if we hit a column with no number, compute current nums with current operator
            if !cur_nums.is_empty() {
                let result: i64 = if cur_operator == '*' {
                    cur_nums.iter().product()
                } else {
                    cur_nums.iter().sum()
                };
                solutions_per_row.push(result);
                cur_nums.clear();
            }
        }
        // handle last column
        if col == num_cols - 1 && !cur_nums.is_empty() {
            let result: i64 = if cur_operator == '*' {
                cur_nums.iter().product()
            } else {
                cur_nums.iter().sum()
            };
            solutions_per_row.push(result);
            cur_nums.clear();
        }
        println!("Column {}: num = {}, operator = {}", col, num, cur_operator);
    }
    println!("Solutions per row: {:?}", solutions_per_row);
    
    // // // print transposed matrix
    // // for row in transposed {
    // //     let line: String = row.join(" ");
    // //     println!("{}", line);
    // // }

    // let mut solutions_per_row = Vec::new();
    // for row in transposed {
    //     if row[0] == "*" {
    //         // take product of rest of row
    //         let product: i64 = row.iter()
    //             .skip(1)
    //             .map(|s| s.parse::<i64>().unwrap())
    //             .product();
    //         // println!("Product of row: {}", product);
    //         solutions_per_row.push(product);
    //     } else if row[0] == "+" {
    //         let sum: i64 = row.iter()
    //             .skip(1)
    //             .map(|s| s.parse::<i64>().unwrap())
    //             .sum();
    //         // println!("Sum of row: {}", sum);
    //         solutions_per_row.push(sum);
    //     }
    // }
    let final_solution: i64 = solutions_per_row.iter().sum();
    println!("Final solution: {}", final_solution);
    Ok(())
}