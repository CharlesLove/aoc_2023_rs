#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs, num};

fn main() {
    let binding = fs::read_to_string("./day-04/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let mut wins = 0;
    for card in input.lines() {
        let mut cur_card_wins = 0;
        // cut off Card #: text
        let mut useful_half = card.split(':').last().unwrap().split('|');
        let winning_numbers: Vec<&str> = useful_half
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();
        let my_numbers: Vec<&str> = useful_half
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();

        for winning_num in winning_numbers {
            for my_num in my_numbers.clone() {
                if my_num == winning_num {
                    cur_card_wins += 1;
                }
            }
        }
        if cur_card_wins > 0 {
            wins += 2u32.pow(cur_card_wins - 1);
        }
    }
    wins
}
fn part_two(input: &str) -> u32 {
    let mut cards = 0;

    let mut card_count_hash: HashMap<usize, usize> = HashMap::new();
    for i in 1..=input.lines().count() {
        card_count_hash.entry(i).or_insert(1);
    }

    for card in input.lines() {
        let mut cur_card_matches = 0;

        let mut card_number: usize = card
            .split(':')
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let mut useful_half = card.split(':').last().unwrap().split('|');
        let winning_numbers: Vec<&str> = useful_half
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();
        let my_numbers: Vec<&str> = useful_half
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();

        let mut matches = 0;
        for winning_num in winning_numbers {
            for my_num in my_numbers.clone() {
                if my_num == winning_num {
                    matches += 1;
                    *card_count_hash.entry(card_number + matches).or_insert(1) +=
                        card_count_hash[&card_number];
                }
            }
        }
    }

    for (_, card_count) in card_count_hash.iter() {
        cards += card_count;
    }
    cards.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_one() {
        assert_eq!(part_one(TEST_LINES1), 13);
    }
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES1), 30);
    }
}
