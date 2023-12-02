use core::panic;
use std::collections::HashMap;
use std::fs;

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

// Simplification game plan
// 1. Iterate through length of line string
// 2. take the current iterator and create a string slice from there to end of line string
// 3. If first position is digit, store it and quit the loop
// 4. Else, see if starts with is in dictionary, if so return it and quit

fn add_line_corrected(line: &str) -> u32 {
    let mut digit_1: i32 = 0;
    let mut digit_2: i32 = 0;
    let mut combined_digit = "".to_owned();

    let mut i = 0;
    while i < line.len() {
        let cur_slice = &line[i..];
        let first_char = cur_slice.chars().nth(0).unwrap();
        if first_char.is_ascii_digit() {
            if digit_1 == 0 {
                digit_1 = first_char.to_digit(10).unwrap() as i32;
            }
            digit_2 = first_char.to_digit(10).unwrap() as i32;
        } else {
            let cur_match = find_match(&cur_slice);
            if cur_match > 0 {
                if digit_1 == 0 {
                    digit_1 = cur_match;
                }
                digit_2 = cur_match;
            }
        }
        i += 1;
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

fn find_match(checked_string: &str) -> i32 {
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

    for (key, value) in number_spellings.clone().into_iter() {
        if checked_string.starts_with(key) {
            return number_spellings[key];
        }
    }
    0
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
}
