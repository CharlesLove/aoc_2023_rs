#![warn(clippy::pedantic)]
use std::fs;

/// # Panics
///
/// Will panic if file paths are incorrect
#[must_use]
pub fn get_input(is_bench: bool) -> String {
    let binding = if is_bench {
        fs::read_to_string("../{{project-name}}/inputs/input.txt").unwrap()
    } else {
        fs::read_to_string("./{{project-name}}/inputs/input.txt").unwrap()
    };
    binding.trim().to_string()
}

/// # Panics
///
/// Will panic if input is incorrect
pub fn part_one(input: &str) -> u32 {
    0
}
/// # Panics
///
/// Will panic if input is incorrect
pub fn part_two(input: &str) -> u32 {
    0
}
