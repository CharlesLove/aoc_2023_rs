#![warn(clippy::pedantic)]
use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
struct AlmanacMap {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

fn main() {
    let binding = fs::read_to_string("./day-05/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("{}", input);

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    0
}
fn part_two(input: &str) -> u32 {
    0
}

fn get_seeds(input: &str) -> Vec<u32> {
    //let mut seed_vec: Vec<u32> = Vec::new();

    let seed_string = input
        .split("seeds:")
        .nth(1)
        .unwrap()
        .split("\n\n")
        .next()
        .unwrap()
        .trim();
    let seed_vec = seed_string
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    seed_vec
}

fn get_map(input: &str, map_name: &str) -> Vec<AlmanacMap> {
    let mut map_vec: Vec<AlmanacMap> = Vec::new();
    let map_name_formatted = format!("{map_name} map:");
    let map_string = input
        .split(&map_name_formatted)
        .nth(1)
        .unwrap()
        .split("\n\n")
        .next()
        .unwrap()
        .trim();

    for line in map_string.lines() {
        let number: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let almanac_map_entry: AlmanacMap = AlmanacMap {
            destination_range_start: number[0],
            source_range_start: number[1],
            range_length: number[2],
        };
        map_vec.push(almanac_map_entry);
    }

    map_vec
}

fn get_destination(current_number: u32, next_map: Vec<AlmanacMap>) -> u32 {
    let mut next_number = current_number;

    for a_map in next_map {
        let min = a_map.source_range_start;
        let max = min + a_map.range_length - 1;

        if current_number >= min && current_number <= max {
            let diff = current_number - min;
            next_number = a_map.destination_range_start + diff;
            break;
        }
    }

    next_number
}

fn get_location(seed_number: u32, input: &str) -> u32 {
    let path = [
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];

    let mut cur_number = seed_number;

    for x in 0..path.len() - 1 {
        let map_name = format!("{0}-to-{1}", path[x], path[x + 1]);
        cur_number = get_destination(cur_number, get_map(input, &map_name));
    }
    cur_number
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_seeds() {
        assert_eq!(get_seeds(TEST_LINES1), [79, 14, 55, 13]);
    }
    #[test]
    fn test_maps() {
        assert_eq!(
            get_map(TEST_LINES1, "seed-to-soil"),
            [
                AlmanacMap {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2
                },
                AlmanacMap {
                    destination_range_start: 52,
                    source_range_start: 50,
                    range_length: 48
                },
            ]
        );
        assert_eq!(
            get_map(TEST_LINES1, "humidity-to-location"),
            [
                AlmanacMap {
                    destination_range_start: 60,
                    source_range_start: 56,
                    range_length: 37
                },
                AlmanacMap {
                    destination_range_start: 56,
                    source_range_start: 93,
                    range_length: 4
                },
            ]
        );
    }
    #[test]
    fn test_destination() {
        let seed_to_soil = [
            AlmanacMap {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            },
            AlmanacMap {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            },
        ];

        assert_eq!(get_destination(79, seed_to_soil.to_vec()), 81);
        assert_eq!(get_destination(14, seed_to_soil.to_vec()), 14);
        assert_eq!(get_destination(55, seed_to_soil.to_vec()), 57);
        assert_eq!(get_destination(13, seed_to_soil.to_vec()), 13);
    }

    #[test]
    fn test_location() {
        assert_eq!(get_location(79, TEST_LINES1), 82);
        assert_eq!(get_location(14, TEST_LINES1), 43);
        assert_eq!(get_location(55, TEST_LINES1), 86);
        assert_eq!(get_location(13, TEST_LINES1), 35);
    }
    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 8);
    }
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES1), 8);
    }
}
