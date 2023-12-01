use core::panic;
use std::collections::{hash_map, HashMap};
use std::fs;

fn main() {
    let lines = fs::read_to_string("./inputs/input1.txt").unwrap();
    // For some reason there is an empty line at the end
    println!("Part 1:");
    println!("{}", add_lines(lines.trim_end()));
    println!("Part 2:");
}

fn add_line(line: &str) -> u32 {
    let mut digit_1: i32 = -1;
    let mut digit_2: i32 = -1;
    let mut combined_digit = "".to_owned();

    // find first digit
    for c in line.chars() {
        if c.is_numeric() {
            digit_1 = c.to_digit(10).unwrap() as i32;
            break;
        }
    }
    // find last digit
    for c in line.chars().rev() {
        if c.is_numeric() {
            digit_2 = c.to_digit(10).unwrap() as i32;
            break;
        }
    }

    if digit_1 < 0 || digit_2 < 0 {
        panic!("Incorrect values with line = '{}'", &line);
    }
    combined_digit.push_str(&digit_1.to_string());
    combined_digit.push_str(&digit_2.to_string());
    combined_digit.parse().unwrap()
}

// TODO: remove premature optimization
fn add_line_corrected(line: &str) -> u32 {
    let mut digit_1: i32 = -1;
    let mut digit_2: i32 = -1;
    let mut combined_digit = "".to_owned();

    let mut current_number_string = "".to_owned();

    // find first digit
    for c in line.chars() {
        if c.is_numeric() {
            if digit_1 == -1 {
                digit_1 = c.to_digit(10).unwrap() as i32;
            } else {
                digit_2 = c.to_digit(10).unwrap() as i32;
            }
            // a new numeric clears the current number string
            current_number_string = "".to_string();
        } else if c.is_ascii_alphabetic() {
            current_number_string.push_str(&c.to_string());
            // check if current string is matching or starting to match
            // a number
            if check_match_progress(&current_number_string) {
                // we're making progress...
                // check if we've found a match
                let found_digit = check_match(&current_number_string);
                if found_digit != -1 {
                    if digit_1 == -1 {
                        digit_1 = found_digit;
                    } else {
                        digit_2 = found_digit;
                    }
                    // we found a digit, so clear the current number string
                    current_number_string = "".to_string();
                }
            } else {
                // no progress made clear the current number string
                current_number_string = "".to_string();
            }
        }
    }
    // find last digit
    for c in line.chars().rev() {
        if c.is_numeric() {
            digit_2 = c.to_digit(10).unwrap() as i32;
            break;
        }
    }

    if digit_1 < 0 || digit_2 < 0 {
        panic!("Incorrect values with line = '{}'", &line);
    }
    combined_digit.push_str(&digit_1.to_string());
    combined_digit.push_str(&digit_2.to_string());
    combined_digit.parse().unwrap()
}

fn add_lines(lines: &str) -> u32 {
    let mut sum = 0;
    let split_lines = lines.split("\n");
    for l in split_lines {
        sum += add_line(&l.to_string());
    }
    sum
}
fn add_lines_corrected(lines: &str) -> u32 {
    let mut sum = 0;
    let split_lines = lines.split("\n");
    for l in split_lines {
        sum += add_line(&l.to_string());
    }
    sum
}

// check if making progress towards match
fn check_match_progress(checked_string: &str) -> bool {
    if "one".starts_with(checked_string)
        || "two".starts_with(checked_string)
        || "three".starts_with(checked_string)
        || "four".starts_with(checked_string)
        || "five".starts_with(checked_string)
        || "six".starts_with(checked_string)
        || "seven".starts_with(checked_string)
        || "eight".starts_with(checked_string)
        || "nine".starts_with(checked_string)
    {
        return true;
    }
    false
}
// check if match is made
fn check_match(checked_string: &str) -> i32 {
    let number_spellings: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    if number_spellings.contains_key(checked_string) {
        return number_spellings[checked_string];
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LINES1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    const TEST_LINES2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_add() {
        assert_eq!(add_line("1abc2"), 12);
        assert_eq!(add_line("pqr3stu8vwx"), 38);
        assert_eq!(add_line("a1b2c3d4e5f"), 15);
        assert_eq!(add_line("treb7uchet"), 77);
    }
    #[test]
    fn test_add_corrected() {
        assert_eq!(add_line_corrected("two1nine"), 29);
        assert_eq!(add_line_corrected("eightwothree"), 83);
        assert_eq!(add_line_corrected("abcone2threexyz"), 13);
        assert_eq!(add_line_corrected("xtwone3four"), 24);
        assert_eq!(add_line_corrected("4nineeightseven2"), 42);
        assert_eq!(add_line_corrected("zoneight234"), 14);
        assert_eq!(add_line_corrected("7pqrstsixteen"), 76);
    }
    #[test]
    fn test_add_lines() {
        assert_eq!(add_lines(TEST_LINES1), 142);
    }
    #[test]
    fn test_add_lines_corrected() {
        assert_eq!(add_lines_corrected(TEST_LINES2), 281);
    }

    #[test]
    fn test_matches() {
        assert_eq!(check_match("one"), 1);
        assert_eq!(check_match("two"), 2);

        assert_eq!(check_match("on"), -1);
        assert_eq!(check_match("twx"), -1);
    }
    #[test]
    fn test_progress() {
        assert_eq!(check_match_progress("o"), true);
        assert_eq!(check_match_progress("on"), true);
        assert_eq!(check_match_progress("one"), true);

        assert_eq!(check_match_progress("onx"), false);
        assert_eq!(check_match_progress("x"), false);
    }
}
