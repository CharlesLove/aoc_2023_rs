#![warn(clippy::pedantic)]

use solutions::{get_input, part_one, part_two};
fn main() {
    let input = get_input(false);

    println!("Part 1:\n{}", part_one(&input));

    // too low 3102798021
    // too high 18026951026953744581
    println!("Part 2:\n{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const TEST_LINES2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const TEST_LINES3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 2);
        assert_eq!(part_one(TEST_LINES2), 6);
    }
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES3), 6);
    }
}
