use lcmx::lcmx;
use std::{collections::HashMap, fs};

pub fn get_input(is_bench: bool) -> String {
    let binding = if is_bench {
        fs::read_to_string("../day-08/inputs/input.txt").unwrap()
    } else {
        fs::read_to_string("./day-08/inputs/input.txt").unwrap()
    };
    binding.trim().to_string()
}

pub fn part_one(input: &str) -> u64 {
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
pub fn part_two(input: &str) -> u64 {
    let mut steps = 0;
    let mut current_locations: Vec<String> = Vec::new();

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

    let mut destination_steps: Vec<u64> = vec![0; current_locations.len()];
    let mut i = 0;
    let mut filled_out = false;
    while !filled_out {
        steps += 1;

        for l in 0..current_locations.len() {
            let current_instruction = instructions[i];

            let next_location = &location_map[&current_locations[l]][&current_instruction];

            current_locations[l] = next_location.to_string();
            if current_locations[l].ends_with('Z') && destination_steps[l] == 0 {
                destination_steps[l] = steps;
            }
        }

        if !destination_steps.contains(&0) {
            filled_out = true;
        }

        i += 1;
        if i >= instructions.len() {
            i = 0;
        }
    }

    lcmx(&destination_steps).unwrap()
}
