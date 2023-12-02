use core::panic;
use std::collections::HashMap;
use std::{fs, iter};

fn main() {
    let lines = fs::read_to_string("./inputs/input1.txt").unwrap();
    // For some reason there is an empty line at the end
    println!("Part 1:");
    println!("{}", add_lines(lines.trim_end()));

    // 53097 was too low
    // 56001 was too low
    // 56089 was too high
    println!("Part 2:");
    println!("{}", add_lines_corrected(lines.trim_end()));
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
fn add_line_corrected(line: &str) -> u32 {
    let mut digit_1: i32 = -1;
    let mut digit_2: i32 = -1;
    let mut combined_digit = "".to_owned();

    let mut current_number_string = "".to_owned();

    // find first digit
    let mut cur = 0;
    while cur < line.len() {
        let c = line.chars().nth(cur).unwrap();
        if c.is_numeric() {
            digit_1 = c.to_digit(10).unwrap() as i32;
            break;
        } else if c.is_ascii_alphabetic() {
            current_number_string.push_str(&c.to_string());
            if check_match_progress(&current_number_string, false) {
                // we're making progress...
                // check if we've found a match
                let found_digit = check_match(&current_number_string);
                if found_digit != -1 {
                    if digit_1 == -1 {
                        digit_1 = found_digit;
                    }
                    // we found a digit, so break
                    break;
                }
            } else {
                // go next index past string start
                cur = cur - (current_number_string.len() - 1);
                current_number_string = "".to_string();
                // if we don't continue the iterator doesn't get set correctly
            }
        }
        cur += 1;
    }
    current_number_string = "".to_owned();

    // find last digit
    cur = line.len() - 1;
    while cur >= 0 {
        let c = line.chars().nth(cur).unwrap();
        if c.is_numeric() {
            digit_2 = c.to_digit(10).unwrap() as i32;
            break;
        } else if c.is_ascii_alphabetic() {
            // add to front of string
            current_number_string = c.to_string() + &current_number_string;
            if check_match_progress(&current_number_string, true) {
                // we're making progress...
                // check if we've found a match
                let found_digit = check_match(&current_number_string);
                if found_digit != -1 {
                    if digit_2 == -1 {
                        digit_2 = found_digit;
                    }
                    // we found a digit, so break
                    break;
                }
            } else {
                // go next index past string start
                // TODO: figure out how to do this iteration correctly
                cur = cur - (current_number_string.len() - 1);
                current_number_string = "".to_string();
                // if we don't continue the iterator doesn't get set correctly
                continue;
            }
        }
        cur -= 1;
    }

    if digit_1 < 0 || digit_2 < 0 {
        panic!("Incorrect values with line = '{}'", &line);
    }
    combined_digit.push_str(&digit_1.to_string());
    combined_digit.push_str(&digit_2.to_string());
    combined_digit.parse().unwrap()
}

// TODO: do a similar backwards and forward to part 1

fn add_line_corrected_complicated(line: &str) -> u32 {
    let mut digit_1: i32 = -1;
    let mut digit_2: i32 = -1;
    let mut combined_digit = "".to_owned();

    let mut current_number_string = "".to_owned();

    // find first and last digits
    let mut cur = 0;
    while cur < line.len() {
        // for (mut i, c) in line.chars().enumerate() {
        let c = line.chars().nth(cur).unwrap();
        if c.is_numeric() {
            if digit_1 == -1 {
                digit_1 = c.to_digit(10).unwrap() as i32;
            }
            digit_2 = c.to_digit(10).unwrap() as i32;
            // a new numeric clears the current number string
            current_number_string = "".to_string();
        } else if c.is_ascii_alphabetic() {
            current_number_string.push_str(&c.to_string());
            // check if current string is matching or starting to match
            // a number
            if check_match_progress(&current_number_string, false) {
                // we're making progress...
                // check if we've found a match
                let found_digit = check_match(&current_number_string);
                if found_digit != -1 {
                    if digit_1 == -1 {
                        digit_1 = found_digit;
                    }
                    digit_2 = found_digit;
                    // we found a digit, so clear the current number string
                    current_number_string = "".to_string();
                }
            } else {
                // start at 2nd index of number string
                cur = cur - (current_number_string.len() - 1);
                current_number_string = current_number_string.remove(0).to_string();

                // if we don't continue the iterator doesn't get set correctly
                continue;
            }
        }
        cur += 1;
    }

    // if either digit is -1, the string doesn't have any numbers or is malformed
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
        //let line_value = add_line_corrected(&l.to_string());
        sum += add_line_corrected(&l.to_string());
    }
    sum
}

// check if making progress towards match
fn check_match_progress(checked_string: &str, is_reverse: bool) -> bool {
    if is_reverse {
        if "one".ends_with(checked_string)
            || "two".ends_with(checked_string)
            || "three".ends_with(checked_string)
            || "four".ends_with(checked_string)
            || "five".ends_with(checked_string)
            || "six".ends_with(checked_string)
            || "seven".ends_with(checked_string)
            || "eight".ends_with(checked_string)
            || "nine".ends_with(checked_string)
        {
            return true;
        }
    } else {
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
        assert_eq!(add_line_corrected("28"), 28);
        assert_eq!(add_line_corrected("2"), 22);
        assert_eq!(add_line_corrected("two"), 22);
        assert_eq!(add_line_corrected("two1nine"), 29);
        assert_eq!(add_line_corrected("eightwothree"), 83);
        assert_eq!(add_line_corrected("abcone2threexyz"), 13);
        assert_eq!(add_line_corrected("xtwone3four"), 24);
        assert_eq!(add_line_corrected("4nineeightseven2"), 42);
        assert_eq!(add_line_corrected("zoneight234"), 14);
        assert_eq!(add_line_corrected("7pqrstsixteen"), 76);

        assert_eq!(add_line_corrected("3n"), 33);
        assert_eq!(add_line_corrected("faxthreedexa"), 33);

        // this case has been leading to inflated sums
        assert_eq!(add_line_corrected("2ctfonekpns"), 21);
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
        assert_eq!(check_match_progress("o", false), true);
        assert_eq!(check_match_progress("on", false), true);
        assert_eq!(check_match_progress("one", false), true);

        assert_eq!(check_match_progress("onx", false), false);
        assert_eq!(check_match_progress("x", false), false);
    }
}
