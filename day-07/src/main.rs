#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

struct card_hand {
    score: u32,
    bid: u32,
    cards: str,
}

fn main() {
    let binding = fs::read_to_string("./day-07/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    0
}
fn part_two(input: &str) -> u32 {
    0
}

fn get_card_value(card_char: char) -> u32 {
    let card_value_map: HashMap<char, u32> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    card_value_map[&card_char]
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 6440);
    }
    // #[test]
    // fn test_two() {
    //     assert_eq!(part_two(TEST_LINES1), 8);
    // }

    #[test]
    fn test_single_card() {
        assert_eq!(get_card_value('2'), 2);
        assert_eq!(get_card_value('A'), 14);
    }
}
