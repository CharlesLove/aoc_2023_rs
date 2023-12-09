#![warn(clippy::pedantic)]
use solutions::{get_input, part_one, part_two};
fn main() {
    let input = get_input(false);

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 8);
    }
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES1), 8);
    }
}
