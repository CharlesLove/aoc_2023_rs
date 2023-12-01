use std::ptr::null;

fn main() {
    println!("Hello, world!");
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
        panic!("Incorrect values!");
    }
    combined_digit.push_str(&digit_1.to_string());
    combined_digit.push_str(&digit_2.to_string());
    combined_digit.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_line("1abc2"), 12);
        assert_eq!(add_line("pqr3stu8vwx"), 38);
        assert_eq!(add_line("a1b2c3d4e5f"), 15);
        assert_eq!(add_line("treb7uchet"), 77);
    }
}
