#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let binding = fs::read_to_string("./day-06/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u64 {
    let times: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .split("Time:")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u64> = input
        .lines()
        .nth(1)
        .unwrap()
        .split("Distance:")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let possibilities: Vec<u64> = times
        .iter()
        .zip(distances.iter())
        .map(|(x, y)| get_possibilities(*x, *y))
        .collect();

    possibilities.iter().product()
}
fn part_two(input: &str) -> u64 {
    let total_time: u64 = input
        .lines()
        .next()
        .unwrap()
        .split("Time:")
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: u64 = input
        .lines()
        .nth(1)
        .unwrap()
        .split("Distance:")
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let possibility: u64 = get_possibilities(total_time, distance);

    possibility
}

fn get_possibilities(total_time: u64, distance: u64) -> u64 {
    let mut possibilities: u64 = 0;
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
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES1), 71503);
    }
}
