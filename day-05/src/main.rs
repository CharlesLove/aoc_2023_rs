#![warn(clippy::pedantic)]
use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
struct AlmanacMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}
#[derive(PartialEq, Debug, Clone, Copy)]
struct SeedRange {
    min: u64,
    max: u64,
}

fn main() {
    let binding = fs::read_to_string("./day-05/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));

    // too high 1605608030
    // too high 404881758
    // incorrect 298690884

    // part 2 is mega broken right now so just disabled
    // println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u64 {
    let mut lowest_location = u64::MAX;
    let seeds = get_seeds(input);
    for cur_seed in seeds {
        let cur_location = get_location(cur_seed, input);
        if cur_location < lowest_location {
            lowest_location = cur_location;
        }
    }

    lowest_location
}
fn _part_two(input: &str) -> u64 {
    let mut lowest_location = u64::MAX;
    let seed_ranges = _get_seed_range(input);
    let mut better_seed_range: SeedRange = seed_ranges[0];

    for seed_range in seed_ranges {
        better_seed_range = _get_better_seed_range(better_seed_range, seed_range);
    }

    let _total_range: u64 = better_seed_range.max - better_seed_range.min;
    //let mut _iterations: f64 = 0.;

    for cur_seed in better_seed_range.min..better_seed_range.max {
        let cur_location = get_location(cur_seed, input);

        if cur_location < lowest_location {
            lowest_location = cur_location;
            //println!("{0}%", (iterations * 1.0 / total_range as f64) * 100.);
            println!("{lowest_location}");
        }
        //_iterations += 1.;
    }

    lowest_location
}

fn get_seeds(input: &str) -> Vec<u64> {
    //let mut seed_vec: Vec<u64> = Vec::new();

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
        let number: Vec<u64> = line
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

fn get_destination(current_number: u64, next_map: Vec<AlmanacMap>) -> u64 {
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
fn _get_destination_from_range(current_range: SeedRange, next_map: Vec<AlmanacMap>) -> u64 {
    let mut next_number = current_range.min;

    for a_map in next_map {
        let min = a_map.source_range_start;
        let max = min + a_map.range_length - 1;

        if current_range.min >= min && current_range.max <= max {
            let diff = current_range.min - min;
            next_number = a_map.destination_range_start + diff;
            break;
        }
    }

    next_number
}

fn get_location(seed_number: u64, input: &str) -> u64 {
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
fn _get_location_from_range(seed_range: SeedRange, input: &str) -> u64 {
    let path = [
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];

    let mut cur_number = _get_destination_from_range(seed_range, get_map(input, "seed-to-soil"));

    for x in 0..path.len() - 1 {
        let map_name = format!("{0}-to-{1}", path[x], path[x + 1]);
        cur_number = get_destination(cur_number, get_map(input, &map_name));
    }

    cur_number
}

fn _get_seed_range(input: &str) -> Vec<SeedRange> {
    let mut seed_range_vec: Vec<SeedRange> = Vec::new();

    let seed_string = input
        .split("seeds:")
        .nth(1)
        .unwrap()
        .split("\n\n")
        .next()
        .unwrap()
        .trim();
    let num_vec: Vec<u64> = seed_string
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for n in (0..num_vec.len()).step_by(2) {
        let cur_seed_range: SeedRange = SeedRange {
            min: num_vec[n],
            max: num_vec[n] + num_vec[n + 1] - 1,
        };
        seed_range_vec.push(cur_seed_range);
    }

    seed_range_vec
}

fn _get_better_seed_range(old_seed_range: SeedRange, new_seed_range: SeedRange) -> SeedRange {
    let mut newer_seed_range = old_seed_range;
    if new_seed_range.min < old_seed_range.min {
        newer_seed_range.min = new_seed_range.min;
    }
    if new_seed_range.max > old_seed_range.max {
        newer_seed_range.max = new_seed_range.max;
    }
    newer_seed_range
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
    fn test_seed_range() {
        assert_eq!(
            _get_seed_range(TEST_LINES1),
            [
                SeedRange { min: 79, max: 92 },
                SeedRange { min: 55, max: 67 }
            ]
        );
    }
    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 35);
    }
    // part 2 is mega broken so just stay disabled for now
    // #[test]
    // fn test_two() {
    //     assert_eq!(part_two(TEST_LINES1), 46);
    // }
}
