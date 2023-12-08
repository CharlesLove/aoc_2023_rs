#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

fn main() {
    let binding = fs::read_to_string("./day-08/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let mut steps = 0;
    let mut current_location = "AAA";
    let final_destination = "ZZZ";
    let mut found = false;
    let instructions: Vec<char> = input.lines().next().unwrap().chars().collect();

    let mut location_map: HashMap<String, HashMap<char, String>> = HashMap::new();

    // build mega hash map
    for line_num in 2..input.lines().count() {
        let current_line = input.lines().nth(line_num).unwrap();

        let two_halves: Vec<&str> = current_line.split(" = ").collect();

        let location_half = two_halves[0].to_string();

        let destinations: Vec<String> = two_halves[1]
            .to_string()
            .replace(['(', ')'], "")
            .split(", ")
            .map(str::to_string)
            .collect();

        let mut destination_map: HashMap<char, String> = HashMap::new();

        destination_map.insert('L', destinations[0].to_string());
        destination_map.insert('R', destinations[1].to_string());

        location_map.insert(location_half, destination_map);
    }

    let mut i = 0;
    while !found {
        steps += 1;

        let current_instruction = instructions[i];

        let next_location = &location_map[current_location][&current_instruction];

        current_location = next_location;
        if current_location == final_destination {
            found = true;
        }

        i += 1;
        if i >= instructions.len() {
            i = 0;
        }
    }

    steps
}
fn part_two(input: &str) -> u32 {
    let mut steps = 0;
    //let mut current_location = "AAA";
    let mut current_locations: Vec<String> = Vec::new();
    let final_destination = "ZZZ";
    let mut found = false;
    let instructions: Vec<char> = input.lines().next().unwrap().chars().collect();

    let mut location_map: HashMap<String, HashMap<char, String>> = HashMap::new();

    // build mega hash map
    for line_num in 2..input.lines().count() {
        let current_line = input.lines().nth(line_num).unwrap();

        let two_halves: Vec<&str> = current_line.split(" = ").collect();

        let location_half = two_halves[0].to_string();
        if location_half.ends_with('A') {
            current_locations.push(location_half.clone());
        }

        let destinations: Vec<String> = two_halves[1]
            .to_string()
            .replace(['(', ')'], "")
            .split(", ")
            .map(str::to_string)
            .collect();

        let mut destination_map: HashMap<char, String> = HashMap::new();

        destination_map.insert('L', destinations[0].to_string());
        destination_map.insert('R', destinations[1].to_string());

        location_map.insert(location_half, destination_map);
    }

    let mut i = 0;
    let mut finds = 0;
    while finds != current_locations.len() {
        finds = 0;
        steps += 1;

        for l in 0..current_locations.len() {
            let current_instruction = instructions[i];

            let next_location = &location_map[&current_locations[l]][&current_instruction];

            current_locations[l] = next_location.to_string();
            if current_locations[l].ends_with('Z') {
                finds += 1;
            } else {
                break;
            }
        }
        i += 1;
        if i >= instructions.len() {
            i = 0;
        }
    }

    steps
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
        assert_eq!(part_two(TEST_LINES2), 6);
    }
}
