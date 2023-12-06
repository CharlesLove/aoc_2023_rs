#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let binding = fs::read_to_string("./day-06/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let times: Vec<u32> = input
        .lines()
        .nth(0)
        .unwrap()
        .split("Time:")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u32> = input
        .lines()
        .nth(1)
        .unwrap()
        .split("Distance:")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let possibilities: Vec<u32> = times
        .iter()
        .zip(distances.iter())
        .map(|(x, y)| get_possibilities(*x, *y))
        .collect();

    possibilities.iter().product()
}
fn part_two(input: &str) -> u32 {
    0
}

fn get_possibilities(total_time: u32, distance: u32) -> u32 {
    let mut possibilities: u32 = 0;
    for held_time in 0..total_time {
        if distance < held_time * (total_time - held_time) {
            possibilities += 1;
        }
    }
    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 288);
    }
    // #[test]
    // fn test_two() {
    //     assert_eq!(part_two(TEST_LINES1), 8);
    // }
}
