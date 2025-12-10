use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("day1.txt")?;
    let reader = io::BufReader::new(file);

    let mut zero_count = 0;
    let mut zero_pass = 0;
    let mut cur = 50;
    for line in reader.lines() {
        let line = line?;
        // println!("Line: {line}");
        let first = line.chars().next().unwrap();
        let rest: i32 = line[1..].parse().unwrap();
        println!("First: {first}, Rest: {rest}");

        if first == 'R' {
            let prev = cur;
            cur = (cur + rest) % 100;
            if cur < prev && cur != 0 {
                // passed zero
                zero_pass += 1;
                println!("Passed zero 1 time!");
            }
            
        } else if first == 'L' {
            let prev = cur;
            cur = ((cur - rest) % 100 + 100) % 100;
            if cur > prev && prev != 0 {
                // passed zero
                zero_pass += 1;
                println!("Passed zero 1 time!");
            }
        }


        // if rest >= 100 {
        //     let diff = rest / 100;
        //     zero_pass += diff;
        //     println!("Passed zero {diff} times!");
        //     // special case: if we land exactly on zero, we should subtract one pass
        //     if cur % 100 == 0 {
        //         zero_pass -= 1;
        //     }
        // }
        zero_pass += rest / 100;
        if cur == 0 {
            zero_count += 1;
        }
        println!("Current position: {cur}");

        // if first == 'R' {
        //     if (cur + rest) > 100 && cur != 0 {
        //         let diff =  (cur + rest) / 100;
        //         zero_pass += diff;
        //         println!("Passed zero {diff} times!");
        //         if (cur + rest) % 100 == 0 {
        //             zero_pass -= 1;
        //         }
        //     }
        //     cur += rest;
        //     cur = cur % 100;  
        // } else if first == 'L' {
        //     if (cur - rest) < 0 && cur != 0 {
        //         let diff = (rest - cur + 100) / 100;
        //         zero_pass += diff;
        //         println!("Passed zero {diff} times!");
        //     }
        //     cur -= rest;
        //     if cur < 0 {
        //         cur = (cur % 100 + 100) % 100;
        //     }
        // }

        // if cur == 0 {
        //     zero_count += 1;
        // }
        // println!("Current position: {cur}");
    }

    println!("Zero count: {}", zero_count);
    println!("Zero passes: {}", zero_pass);
    println!("Total: {}", zero_count + zero_pass);
    Ok(())
}


