#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

fn main() {
    let binding = fs::read_to_string("./day-08/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));

    // too low 3102798021
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

    let starting_locations = current_locations.clone();
    let mut loop_lengths: Vec<u32> = vec![0; current_locations.len()];
    let mut destination_steps: Vec<u32> = vec![0; current_locations.len()];
    let mut i = 0;
    let mut filled_out = false;
    while !filled_out {
        //filled_out = 0;
        steps += 1;

        for l in 0..current_locations.len() {
            let current_instruction = instructions[i];

            let next_location = &location_map[&current_locations[l]][&current_instruction];

            current_locations[l] = next_location.to_string();
            if current_locations[l].ends_with('Z') && destination_steps[l] == 0 {
                destination_steps[l] = steps;
                //filled_out += 1;
            }
            // if current_locations[l] == starting_locations[l] && loop_lengths[l] == 0 {
            //     loop_lengths[l] = steps;
            // }

            //println!("#{l}; destination steps:{0}", destination_steps[l]);
        }

        if !destination_steps.contains(&0) {
            filled_out = true;
        }
        //println!("{0} {steps} {finds}", current_locations.len());
        i += 1;
        if i >= instructions.len() {
            i = 0;
        }
    }

    let mut product = 1;
    for steps in destination_steps {
        //println!("{steps}");
        product *= steps;
        //println!("{product}");
    }
    product
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
    const TEST_LINES3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 2);
        assert_eq!(part_one(TEST_LINES2), 6);
    }
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES3), 6);
    }
}
