use std::fs::File;
use std::io::{self, BufRead};

fn find_max_joltage(line: &str, num_digits: usize) -> Option<i64> {
    let mut cur_index: usize = 0;
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    if num_digits == 0 || num_digits > len {
        return None;
    }

    let mut digits: Vec<u32> = Vec::with_capacity(num_digits);
    for i in 0..num_digits {
        // max start index so there are enough chars left to pick remaining digits
        let remaining = num_digits - i;
        let max_start = len - remaining;
        let mut max_digit: u32 = 0;
        let mut max_index = cur_index;

        for j in cur_index..=max_start {
            let digit = chars[j].to_digit(10)?; // return None if non-digit
            if digit > max_digit {
                max_digit = digit;
                max_index = j;
                if max_digit == 9 {
                    break;
                }
            }
        }

        digits.push(max_digit);
        cur_index = max_index + 1;
    }

    // build number using i64 with checked ops to avoid overflow
    let mut value: i64 = 0;
    for d in digits {
        value = value.checked_mul(10)?
                     .checked_add(d as i64)?;
    }
    Some(value)
}

fn main() -> io::Result<()> {
    let file = File::open("day3.txt")?;
    let reader = io::BufReader::new(file);

    let mut nums: Vec<i64> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        match find_max_joltage(&line.trim(), 12) {
            Some(n) => nums.push(n),
            None => eprintln!("Skipping invalid/too-short line: {:?}", line),
        }
    }

    println!("Numbers: {:?}", nums);
    println!("Sum: {}", nums.iter().sum::<i64>());

    Ok(())
}


