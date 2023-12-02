#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let binding = fs::read_to_string("./day-02/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", get_sum(input));
    println!("Part 2:\n{}", get_sum_powers(input));
}

fn is_line_possible(cur_line: &str, total_red: i32, total_green: i32, total_blue: i32) -> bool {
    // First split and only use data after colon ':'
    let mut important_data = cur_line.split(':').nth(1).unwrap().to_string();
    // change semicolon ';' to comma ',' for easier parsing
    important_data = important_data.replace(';', ",");
    // then split between commas and trim white space in front
    let color_vector: Vec<&str> = important_data.split(',').collect();
    // then iterate through this list and compare the split.nth(1) to
    // get color and split.nth(0) to get number
    for mut l in color_vector {
        // trim data
        l = l.trim();
        // split data
        let split_data: Vec<&str> = l.split_whitespace().collect();
        // get number
        let number: i32 = split_data[0].parse().unwrap();
        // get color
        let color = split_data[1];

        match color {
            "red" => {
                if number > total_red {
                    return false;
                }
            }
            "green" => {
                if number > total_green {
                    return false;
                }
            }
            "blue" => {
                if number > total_blue {
                    return false;
                }
            }
            &_ => panic!("Something was parsed incorrectly!"),
        }
    }

    true
}

fn get_id(cur_line: &str) -> i32 {
    let game_id_split = cur_line.split(':').next().unwrap();
    game_id_split.split(' ').nth(1).unwrap().parse().unwrap()
}

fn get_sum(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for l in lines {
        let id = get_id(l);
        if is_line_possible(l, 12, 13, 14) {
            sum += id;
        }
    }
    sum
}
fn get_sum_powers(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for l in lines {
        sum += get_line_power(l);
    }
    sum
}

fn get_line_power(cur_line: &str) -> i32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    // First split and only use data after colon ':'
    let mut important_data = cur_line.split(':').nth(1).unwrap().to_string();
    // change semicolon ';' to comma ',' for easier parsing
    important_data = important_data.replace(';', ",");
    // then split between commas and trim white space in front
    let color_vector: Vec<&str> = important_data.split(',').collect();
    // then iterate through this list and compare the split.nth(1) to
    // get color and split.nth(0) to get number
    for mut l in color_vector {
        // trim data
        l = l.trim();
        // split data
        let split_data: Vec<&str> = l.split_whitespace().collect();
        // get number
        let number: i32 = split_data[0].parse().unwrap();
        // get color
        let color = split_data[1];

        match color {
            "red" => {
                if number > min_red {
                    min_red = number;
                }
            }
            "green" => {
                if number > min_green {
                    min_green = number;
                }
            }
            "blue" => {
                if number > min_blue {
                    min_blue = number;
                }
            }
            &_ => panic!("Something was parsed incorrectly!"),
        }
    }
    min_red * min_green * min_blue
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn test_ids() {
        assert_eq!(
            get_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            get_id(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        "
            ),
            4
        );
    }

    #[test]
    fn test_possibilities() {
        assert!(is_line_possible(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            12,
            13,
            14
        ));
        assert!(!is_line_possible(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            12,
            13,
            14
        ));
    }

    #[test]
    fn test_sum() {
        assert_eq!(get_sum(TEST_LINES1), 8);
    }
    #[test]
    fn test_sum_powers() {
        assert_eq!(get_sum_powers(TEST_LINES1), 2286);
    }

    #[test]
    fn test_powers() {
        assert_eq!(
            get_line_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            get_line_power(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            1560
        );
    }
}
