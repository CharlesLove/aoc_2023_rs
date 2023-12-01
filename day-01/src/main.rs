use core::panic;
use std::fs;

fn main() {
    let lines = fs::read_to_string("./inputs/input1.txt").unwrap();
    // For some reason there is an empty line at the end
    println!("{}", add_lines(lines.trim_end()));
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

fn add_lines(lines: &str) -> u32 {
    let mut sum = 0;
    let split_lines = lines.split("\n");
    for l in split_lines {
        sum += add_line(&l.to_string());
    }
    sum
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LINES: &str = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;

    #[test]
    fn test_add() {
        assert_eq!(add_line("1abc2"), 12);
        assert_eq!(add_line("pqr3stu8vwx"), 38);
        assert_eq!(add_line("a1b2c3d4e5f"), 15);
        assert_eq!(add_line("treb7uchet"), 77);
    }
    #[test]
    fn test_add_lines() {
        assert_eq!(add_lines(TEST_LINES), 142);
    }
}
