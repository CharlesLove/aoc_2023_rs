#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let binding = fs::read_to_string("./day-06/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    0
}
fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 8);
    }
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES1), 8);
    }
}
