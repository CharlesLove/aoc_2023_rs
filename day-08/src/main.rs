#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

struct LocationNode {
    location: String,
    left: String,
    right: String,
}

fn main() {
    let binding = fs::read_to_string("./day-08/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let steps = 0;
    let current_location = "AAA";
    let final_destination = "ZZZ";
    let mut found = false;
    let instructions: Vec<char> = input.lines().next().unwrap().chars().collect();

    //let mut location_map = HashMap::new();

    // build mega hash map
    for line_num in 2..input.lines().count() {
        println!("{}", input.lines().nth(line_num).unwrap());
    }

    // while !found {
    //     todo!();
    // }

    steps
}
fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const TEST_LINES2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 2);
        assert_eq!(part_one(TEST_LINES2), 6);
    }
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES1), 8);
    }
}
