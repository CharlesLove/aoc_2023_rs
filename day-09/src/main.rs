#![warn(clippy::pedantic)]
use solutions::{get_input, part_one, part_two};
fn main() {
    let input = get_input(false);

    println!("Part 1:\n{}", part_one(&input));
    // -210 not right answer
    // 210 is too low
    println!("Part 2:\n{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 114);
    }
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES1), 2);
    }
}
