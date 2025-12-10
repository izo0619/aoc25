use std::fs::File;
use std::io::{self, BufRead};

fn indices_to_mask(len: usize, indices: &str) -> u32 {
    // parse a string like "(0,2,5)" into a bitmask of length `len`
    let parsed_indices: Vec<usize> = indices
        .trim_matches(|c| c == '(' || c == ')') // remove parentheses
        .split(',')                              // split at comma
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())    // convert to usize
        .collect();

    let mut mask: u32 = 0;
    for i in parsed_indices {
        if i < len {
            // interpret index i as 0..len-1 from left (MSB), convert to bit position
            let pos = (len - 1 - i) as u32;
            mask |= 1u32 << pos;
        }
    }
    mask
}

fn exhaustive_search_button_masks(
    button_masks: &Vec<u32>,
    num_presses: u32,
) -> (Vec<u32>, Vec<Vec<u32>>) {
    // generate all combinations of button presses of length num_presses
    let n = button_masks.len();
    let mut all_combinations: Vec<u32> = Vec::new();
    let mut summations: Vec<Vec<u32>> = Vec::new();

    if num_presses == 0 {
        all_combinations.push(0);
        summations.push(vec![0u32; 32]);
        return (all_combinations, summations);
    }

    if num_presses == 1 {
        for &mask in button_masks.iter() {
            // println!("mask: {}", mask);
            let mut arr = vec![0u32; 32];
            all_combinations.push(mask);
            // for each 1 bit mask, add 1 to its location in summations
            for i in 0..32 {
                // println!("i: {}", i);
                // println!("(mask << i) & (1 << 31): {}", (mask << i) & (1 << 31));
                if (mask << i) & (1 << 31) == (1 << 31) {
                    arr[i] += 1;
                }
            }
            summations.push(arr);
        }
        return (all_combinations, summations);
    }

    let (smaller_combinations, smaller_summations) = exhaustive_search_button_masks(button_masks, num_presses - 1);
    for (i, &smaller_mask) in smaller_combinations.iter().enumerate() {
        
        for &mask in button_masks.iter() {
            all_combinations.push(smaller_mask ^ mask);
            // take smaller_summations[i] and add current mask's bits
            // println!("mask: {}", mask);
            let mut arr = smaller_summations[i].clone();
            for i in 0..32 {
                // Calculate which bit of mask corresponds to this position from the left
                let shift = 32 - 1 - i;
                if (mask >> shift) & 1 == 1 {
                    arr[i] += 1;
                }
            }
            summations.push(arr);
        }
    }
    (all_combinations, summations)
}


fn find_min_button_presses(indicator_lights: &str, buttons: &Vec<&str>, joltage: &str) -> u32 {
    let mut presses = 0;

    // cast indicator_lights to be a binary number
    let mut indicator_lights_value: u32 = 0;
    let mut indicator_lights_bits: usize = 0;

    for c in indicator_lights.chars().filter(|c| *c == '#' || *c == '.') {
        indicator_lights_value <<= 1;
        if c == '#' {
            indicator_lights_value |= 1;
        }
        indicator_lights_bits += 1;
    }

    // convert buttons to binary masks
    let bits_len = indicator_lights.chars().filter(|c| *c == '#' || *c == '.').count();
    let button_masks: Vec<u32> = buttons
        .iter()
        .map(|b| indices_to_mask(bits_len, b))
        .collect();

    // starting from coefficient of 1, brute force all button combinations
    let mut num_presses: u32 = 0;
    let starting_state: u32 = 0;
    
    loop {
        println!("Trying num_presses: {}", num_presses);
        if starting_state == indicator_lights_value {
            return num_presses;
        }

        let (combinations, summations) = exhaustive_search_button_masks(&button_masks, num_presses);
        for i in 0..combinations.len() {
            let state = combinations[i];
            let summation = &summations[i];
            let summation_length = 32 - indicator_lights_bits;
            let summation_string = format!("{{{}}}",
                summation
                    .iter()
                    .skip(summation_length)         
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            println!("Summation: {:?}", summation_string);

            if (starting_state ^ state) == indicator_lights_value && joltage == summation_string {
                return num_presses;
            }

            if num_presses >= 10 {
                // prevent infinite loops
                return u32::MAX;
            }
        }
        num_presses += 1;
    }
}


fn main() -> io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = io::BufReader::new(file);

    let mut nums: Vec<u32> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let indicator_lights = parts[0];
        let buttons = parts[1..parts.len()-1].to_vec();
        let joltage = parts[parts.len()-1];

        println!("Indicator lights: {}", indicator_lights);
        println!("Buttons: {:?}", buttons);
        println!("Joltage: {}", joltage);
        let min_presses = find_min_button_presses(indicator_lights, &buttons, joltage);
        nums.push(min_presses);
        break;   
    }

    println!("Numbers: {:?}", nums);
    println!("Sum: {}", nums.iter().sum::<u32>());
    Ok(())
}


