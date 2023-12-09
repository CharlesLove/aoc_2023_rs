#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

/// # Panics
///
/// Will panic if file paths are incorrect
#[must_use]
pub fn get_input(is_bench: bool) -> String {
    let binding = if is_bench {
        fs::read_to_string("../day-01/inputs/input.txt").unwrap()
    } else {
        fs::read_to_string("./day-01/inputs/input.txt").unwrap()
    };
    binding.trim().to_string()
}

pub fn add_line(line: &str) -> u32 {
    let mut digit_1: u32 = 0;
    let mut digit_2: u32 = 0;
    let mut combined_digit = String::new();

    // find first digit
    for c in line.chars() {
        if c.is_numeric() {
            digit_1 = c.to_digit(10).unwrap();
            break;
        }
    }
    // find last digit
    for c in line.chars().rev() {
        if c.is_numeric() {
            digit_2 = c.to_digit(10).unwrap();
            break;
        }
    }

    combined_digit.push_str(&digit_1.to_string());
    combined_digit.push_str(&digit_2.to_string());
    combined_digit.parse().unwrap()
}

// Simplification game plan
// 1. Iterate through length of line string
// 2. take the current iterator and create a string slice from there to end of line string
// 3. If first position is digit, store it and quit the loop
// 4. Else, see if starts with is in dictionary, if so return it and quit

pub fn add_line_corrected(line: &str) -> u32 {
    let mut digit_1: u32 = 0;
    let mut digit_2: u32 = 0;
    let mut combined_digit = String::new();

    let mut i = 0;
    while i < line.len() {
        let cur_slice = &line[i..];
        let first_char = cur_slice.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            if digit_1 == 0 {
                digit_1 = first_char.to_digit(10).unwrap();
            }
            digit_2 = first_char.to_digit(10).unwrap();
        } else {
            let cur_match = find_match(cur_slice);
            if cur_match > 0 {
                if digit_1 == 0 {
                    digit_1 = cur_match;
                }
                digit_2 = cur_match;
            }
        }
        i += 1;
    }

    combined_digit.push_str(&digit_1.to_string());
    combined_digit.push_str(&digit_2.to_string());
    combined_digit.parse().unwrap()
}

pub fn part_one(lines: &str) -> u32 {
    let mut sum = 0;
    let split_lines = lines.split('\n');
    for l in split_lines {
        sum += add_line(l);
    }
    sum
}
pub fn part_two(lines: &str) -> u32 {
    let mut sum = 0;
    let split_lines = lines.split('\n');
    for l in split_lines {
        //let line_value = add_line_corrected(&l.to_string());
        sum += add_line_corrected(l);
    }
    sum
}

pub fn find_match(checked_string: &str) -> u32 {
    let number_spellings: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (key, _value) in number_spellings.clone() {
        if checked_string.starts_with(key) {
            return number_spellings[key];
        }
    }
    0
}
