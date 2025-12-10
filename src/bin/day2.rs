use std::fs;

fn check_if_repeated(s: &str, repeat_seq: &str) -> bool {
    let repeat_len = repeat_seq.len();
    let s_len = s.len();

    if s_len % repeat_len != 0 {
        return false;
    }

    for i in (0..s_len).step_by(repeat_len) {
        if &s[i..i + repeat_len] != repeat_seq {
            return false;
        }
    }

    true
}

fn main() {
    // Read entire file as a string
    let contents = fs::read_to_string("day2.txt")
        .expect("Failed to read file");

    // Remove newlines just in case
    let line = contents.trim();

    // Split by commas
    let items: Vec<&str> = line.split(',').collect();

    let mut invalid_ids: Vec<i64> = Vec::new();

    // Print each item
    for item in &items {
        // split item by '-'
        let parts: Vec<&str> = item.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        println!("Start: {start}, End: {end}");

        for num in start..=end {
            let s = num.to_string();
            let len = s.len();
            // if len % 2 != 0 {
            //     continue;
            // }

            // let mid = len / 2;
            // let first = &s[..mid];
            // let second = &s[mid..];

            // if first == second {
            //     invalid_ids.push(num);
            //     println!("{num} is invalid");
            // }
            for rep_len in 1..=len/2 {
                let repeat_seq = &s[..rep_len];
                if check_if_repeated(&s, repeat_seq) {
                    invalid_ids.push(num);
                    println!("{num} is invalid (repeats {repeat_seq})");
                    break;
                }
        }
    }
    }

    // sum of invalid ids
    let sum: i64 = invalid_ids.iter().sum();
    println!("Sum of invalid IDs: {}", sum);
}
