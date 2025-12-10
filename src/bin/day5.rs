use std::fs::File;
use std::io::{self, BufRead};

fn check_if_in_range(num: i64, ranges: &Vec<(i64, i64)>) -> bool {
    for (start, end) in ranges {
        if num >= *start && num <= *end {
            return true;
        }
    }
    false
}

fn merge_ranges(ranges: &mut Vec<(i64, i64)>) {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(i64, i64)> = Vec::new();

    for range in ranges.iter() {
        if let Some(last) = merged.last_mut() {
            if range.0 <= last.1 {
                last.1 = last.1.max(range.1);
            } else {
                merged.push(*range);
            }
        } else {
            merged.push(*range);
        }
    }

    *ranges = merged;
}

fn main() -> io::Result<()> {
    let file = File::open("day5.txt")?;
    let reader = io::BufReader::new(file);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();
    
    let mut hit_newline = false;
    
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            hit_newline = true;
            continue;
        }

        if hit_newline {
            let id: i64 = line.trim().parse().unwrap();
            ids.push(id);
            continue;
        }
        let parts: Vec<&str> = line.trim().split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        ranges.push((start, end));
    }

    ids.retain(|&id| check_if_in_range(id, &ranges));


    // println!("Ranges: {:?}", ranges);
    // println!("IDs: {:?}", ids);

    println!("Length of valid IDs: {}", ids.len());

    // part 2
    merge_ranges(&mut ranges);
    let mut total_covered = 0;
    for (start, end) in ranges {
        total_covered += end - start + 1;
    }
    println!("Total covered IDs: {}", total_covered);
    Ok(())
    
}
