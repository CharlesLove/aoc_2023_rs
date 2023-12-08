#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

struct CardHand {
    score: u32,
    bid: u32,
    _cards: String,
}

fn main() {
    let binding = fs::read_to_string("./day-07/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));

    // too low 253304242
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let mut card_hand_vec: Vec<CardHand> = Vec::new();
    let mut total_winnings = 0;

    for line in input.lines() {
        let cur_cards = line.split_whitespace().nth(0).unwrap().trim();
        let cur_bid: u32 = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let cur_hand = get_card_hand(cur_cards, cur_bid);

        card_hand_vec.push(cur_hand);
    }
    card_hand_vec.sort_unstable_by_key(|x| x.score);

    let mut rank: u32 = 1;
    for cur_vec in card_hand_vec {
        total_winnings += cur_vec.bid * rank;
        rank += 1;
    }

    total_winnings
}
fn part_two(input: &str) -> u32 {
    let mut card_hand_vec: Vec<CardHand> = Vec::new();
    let mut total_winnings = 0;

    for line in input.lines() {
        let cur_cards = line.split_whitespace().nth(0).unwrap().trim();
        let cur_bid: u32 = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let cur_hand = get_card_hand_revised(cur_cards, cur_bid);

        card_hand_vec.push(cur_hand);
    }
    card_hand_vec.sort_unstable_by_key(|x| x.score);

    let mut rank: u32 = 1;
    for cur_vec in card_hand_vec {
        total_winnings += cur_vec.bid * rank;
        rank += 1;
    }

    total_winnings
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
fn get_card_value_revised(card_char: char) -> u32 {
    let card_value_map: HashMap<char, u32> = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    card_value_map[&card_char]
}

fn get_card_hand(input_cards: &str, input_bid: u32) -> CardHand {
    let mut card_counts_map: HashMap<char, u32> = HashMap::new();

    let mut base_value = 0;
    let mut bonus_value = 0;

    // convert values using psuedo-hex
    let mut i = 0;
    for cur_char in input_cards.chars().rev() {
        *card_counts_map.entry(cur_char.to_owned()).or_default() += 1;
        base_value += 16u32.pow(i) * get_card_value(cur_char);
        i += 1;
    }

    let mut first_dup: (char, u32) = (' ', 0);
    let mut second_dup: (char, u32) = (' ', 0);

    for (key, value) in card_counts_map {
        if value > first_dup.1 {
            second_dup = first_dup;
            first_dup.0 = key;
            first_dup.1 = value
        } else if value > second_dup.1 {
            second_dup.0 = key;
            second_dup.1 = value
        }
    }

    // high
    if first_dup.1 == 1 {
        // do nothing, we've already found score
    }
    // pair
    else if first_dup.1 == 2 && second_dup.1 < 2 {
        bonus_value = 1;
    }
    // 2-pair
    else if first_dup.1 == 2 && second_dup.1 == 2 {
        bonus_value = 2;
    }
    // 3-kind
    else if first_dup.1 == 3 && second_dup.1 < 2 {
        bonus_value = 3;
    }
    // full-house
    else if first_dup.1 == 3 && second_dup.1 == 2 {
        bonus_value = 4;
    }
    // 4-kind
    else if first_dup.1 == 4 {
        bonus_value = 5;
    }
    // 5-kind
    else if first_dup.1 == 5 {
        bonus_value = 6;
    } else {
        panic!("Something went wrong when calculating this hand!");
    }

    let new_card_hand = CardHand {
        score: base_value + 16u32.pow(i) * bonus_value,
        bid: input_bid,
        _cards: input_cards.to_string(),
    };

    new_card_hand
}
fn get_card_hand_revised(input_cards: &str, input_bid: u32) -> CardHand {
    let mut card_counts_map: HashMap<char, u32> = HashMap::new();

    let mut base_value = 0;
    let mut bonus_value = 0;

    // convert values using psuedo-hex
    let mut i = 0;
    for cur_char in input_cards.chars().rev() {
        *card_counts_map.entry(cur_char.to_owned()).or_default() += 1;
        base_value += 16u32.pow(i) * get_card_value_revised(cur_char);
        i += 1;
    }

    let mut first_dup: (char, u32) = (' ', 0);
    let mut second_dup: (char, u32) = (' ', 0);
    let mut joker_count: u32 = 0;

    for (key, value) in card_counts_map {
        if key == 'J' {
            joker_count = value;
        }
        if value > first_dup.1 {
            second_dup = first_dup;
            first_dup.0 = key;
            first_dup.1 = value
        } else if value > second_dup.1 {
            second_dup.0 = key;
            second_dup.1 = value
        }
    }

    // high
    if first_dup.1 == 1 {
        if joker_count == 1 {
            // 1-pair
            bonus_value = 1;
        }
    }
    // pair
    else if first_dup.1 == 2 && second_dup.1 < 2 {
        if joker_count == 1 || joker_count == 2 {
            // 3-kind
            bonus_value = 3
        } else {
            bonus_value = 1;
        }
    }
    // 2-pair
    else if first_dup.1 == 2 && second_dup.1 == 2 {
        if joker_count == 1 {
            // full house
            bonus_value = 4;
        } else if joker_count == 2 {
            // 4-kind
            bonus_value = 5;
        } else {
            bonus_value = 2;
        }
    }
    // 3-kind
    else if first_dup.1 == 3 && second_dup.1 < 2 {
        if joker_count == 1 {
            // 4-kind
            bonus_value = 5;
        } else if joker_count == 3 {
            // 4-kind
            bonus_value = 5;
        } else {
            bonus_value = 3;
        }
    }
    // full-house
    else if first_dup.1 == 3 && second_dup.1 == 2 {
        if joker_count == 2 || joker_count == 3 {
            // 5-kind
            bonus_value = 6;
        } else {
            bonus_value = 4;
        }
    }
    // 4-kind
    else if first_dup.1 == 4 {
        if joker_count == 1 || joker_count == 4 {
            // 5-kind
            bonus_value = 6;
        } else {
            bonus_value = 5;
        }
    }
    // 5-kind
    else if first_dup.1 == 5 {
        bonus_value = 6;
    } else {
        panic!("Something went wrong when calculating this hand!");
    }

    // bonus_value += joker_count * 2;

    // if bonus_value > 6 {
    //     bonus_value = 6;
    // }

    let new_card_hand = CardHand {
        score: base_value + 16u32.pow(i) * bonus_value,
        bid: input_bid,
        _cards: input_cards.to_string(),
    };

    new_card_hand
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
    #[test]
    fn test_two() {
        assert_eq!(part_two(TEST_LINES1), 5905);
    }

    #[test]
    fn test_single_card() {
        assert_eq!(get_card_value('2'), 2);
        assert_eq!(get_card_value('A'), 14);
    }
}
