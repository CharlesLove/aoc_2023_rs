#![warn(clippy::pedantic)]

use solutions::{add_line, add_line_corrected, get_input, part_one, part_two};

fn main() {
    let input = get_input(false);
    //fs::read_to_string("./day-01/inputs/input1.txt").unwrap();
    // For some reason there is an empty line at the end
    println!("Part 1:");
    println!("{}", part_one(&input));

    // 53097 was too low
    // 56001 was too low
    // 56089 was too high
    println!("Part 2:");
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LINES1: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const TEST_LINES2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

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
        assert_eq!(part_one(TEST_LINES1), 142);
    }
    #[test]
    fn test_add_lines_corrected() {
        assert_eq!(part_two(TEST_LINES2), 281);
    }
}
