#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coords2D {
    x: u32,
    y: u32,
}

fn main() {
    let binding = fs::read_to_string("./day-03/inputs/input.txt").unwrap();
    let input = binding.trim();

    // 1382231 is too high
    // 550354 is too high too?
    println!("Part 1:\n{}", find_numbers_sum(input));
    println!("Part 2:\n{}", find_gear_ratio_sum(input));
}

fn get_width(input: &str) -> i32 {
    input.lines().next().unwrap().len().try_into().unwrap()
}

fn get_height(input: &str) -> i32 {
    input.lines().count().try_into().unwrap()
}

fn check_surrounding(input: &str, cur_x: i32, cur_y: i32, length: i32) -> bool {
    let mut is_summable = false;
    let mut surrounding_string = String::new();
    let mut top_bound = 0;
    if cur_y as i32 - 1 > 0 {
        top_bound = cur_y - 1;
    }
    let mut left_bound = 0;
    if cur_x as i32 - length as i32 - 1 > 0 {
        left_bound = cur_x - length - 1;
    }
    let mut right_bound = get_width(input) - 1;
    if cur_x < get_width(input) {
        right_bound = cur_x;
    }
    let mut bottom_bound = get_height(input) - 1;
    if cur_y + 1 < get_height(input) {
        bottom_bound = cur_y + 1;
    }

    for y in top_bound..=bottom_bound {
        for x in left_bound..=right_bound {
            //let cur_line = input.lines().nth(y).unwrap();
            let cur_char = input
                .lines()
                .nth(y.try_into().unwrap())
                .unwrap()
                .chars()
                .nth(x.try_into().unwrap())
                .unwrap();
            surrounding_string.push(cur_char);

            if cur_char.is_ascii_punctuation() && cur_char != '.' {
                is_summable = true;
            }
        }
        surrounding_string.push('\n');
    }

    // if is_summable {
    //     println!("line: {cur_y} char: {cur_x}");
    //     println!("{surrounding_string}");
    // }

    is_summable
}

fn find_numbers_sum(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut cur_num_string = String::new();

    let mut cur_x;
    let mut cur_y = 0;
    for line in input.lines() {
        cur_x = 0;
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                cur_num_string.push(ch);
            } else if !cur_num_string.is_empty() {
                if check_surrounding(
                    input,
                    cur_x,
                    cur_y,
                    cur_num_string.len().try_into().unwrap(),
                ) {
                    sum += &cur_num_string.parse().unwrap();
                }
                cur_num_string = String::new();
            }
            cur_x += 1;
        }
        if !cur_num_string.is_empty() {
            if check_surrounding(
                input,
                cur_x,
                cur_y,
                cur_num_string.len().try_into().unwrap(),
            ) {
                sum += &cur_num_string.parse().unwrap();
            }
            cur_num_string = String::new();
        }
        cur_y += 1;
    }

    sum
}

fn find_gear_ratio_sum(input: &str) -> u32 {
    let mut sum: u32 = 0;

    let mut cur_num_string = String::new();

    let mut gear_hash: HashMap<Coords2D, Vec<u32>> = HashMap::new();

    let mut cur_x;
    let mut cur_y = 0;
    for line in input.lines() {
        cur_x = 0;
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                cur_num_string.push(ch);
            } else if !cur_num_string.is_empty() {
                for location in get_gear_locations(
                    input,
                    cur_x,
                    cur_y,
                    cur_num_string.len().try_into().unwrap(),
                ) {
                    gear_hash
                        .entry(location)
                        .or_default()
                        .push(cur_num_string.parse().unwrap());
                }
                // if get_gear_locations(input, cur_x, cur_y, cur_num_string.len()) {
                //     sum += &cur_num_string.parse().unwrap();
                // }
                cur_num_string = String::new();
            }
            cur_x += 1;
        }
        if !cur_num_string.is_empty() {
            get_gear_locations(
                input,
                cur_x,
                cur_y,
                cur_num_string.len().try_into().unwrap(),
            )
            .into_iter()
            .for_each(|location| {
                gear_hash
                    .entry(location)
                    .or_default()
                    .push(cur_num_string.parse().unwrap());
            });
            cur_num_string = String::new();
        }
        cur_y += 1;
    }

    for (_key, value) in gear_hash {
        if value.len() == 2 {
            let product: u32 = value.iter().product();
            sum += product;
        }
    }
    sum
}

fn get_gear_locations(input: &str, cur_x: i32, cur_y: i32, length: i32) -> Vec<Coords2D> {
    let _has_gear = false;
    let mut surrounding_string = String::new();
    let mut top_bound = 0;

    let mut gear_locations: Vec<Coords2D> = Vec::new();

    if cur_y as i32 - 1 > 0 {
        top_bound = cur_y - 1;
    }
    let mut left_bound = 0;
    if cur_x as i32 - length as i32 - 1 > 0 {
        left_bound = cur_x - length - 1;
    }
    let mut right_bound = get_width(input) - 1;
    if cur_x < get_width(input).try_into().unwrap() {
        right_bound = cur_x;
    }
    let mut bottom_bound = get_height(input) - 1;
    if cur_y + 1 < get_height(input).try_into().unwrap() {
        bottom_bound = cur_y + 1;
    }

    for y in top_bound..=bottom_bound {
        for x in left_bound..=right_bound {
            //let cur_line = input.lines().nth(y).unwrap();
            let cur_char = input
                .lines()
                .nth(y.try_into().unwrap())
                .unwrap()
                .chars()
                .nth(x.try_into().unwrap())
                .unwrap();
            surrounding_string.push(cur_char);

            if cur_char == '*' {
                gear_locations.push(Coords2D {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
        }
        surrounding_string.push('\n');
    }

    // if is_summable {
    //     println!("line: {cur_y} char: {cur_x}");
    //     println!("{surrounding_string}");
    // }

    gear_locations
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    // same as test lines 1, but with right column missing
    const TEST_LINES2: &str = r"467..114.
...*.....
..35..633
......#..
617*.....
.....+.58
..592....
......755
...$.*...
.664.598.";

    #[test]
    fn test_get_sum() {
        //assert_ne!(get_sum(TEST_LINES1), 4533);
        //assert_eq!(find_numbers_sum(TEST_LINES1), 4361);
        assert_eq!(find_numbers_sum(TEST_LINES2), 4361);
    }
    #[test]
    fn test_get_gear_sum() {
        //assert_ne!(get_sum(TEST_LINES1), 4533);
        //assert_eq!(find_gear_ratio_sum(TEST_LINES1), 4361);
        assert_eq!(find_gear_ratio_sum(TEST_LINES2), 467_835);
    }
}
