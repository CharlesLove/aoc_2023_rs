#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

struct CardHand {
    score: u32,
    bid: u32,
    cards: String,
}

fn main() {
    let binding = fs::read_to_string("./day-07/inputs/input.txt").unwrap();
    let input = binding.trim();

    println!("Part 1:\n{}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

// Too high
fn part_one(input: &str) -> u32 {
    let mut card_hand_vec: Vec<CardHand> = Vec::new();
    let mut total_winnings = 0;

    for line in input.lines() {
        let mut split_line = line.trim().split_whitespace();

        let cur_cards = split_line.next().unwrap().trim();
        let cur_bid: u32 = split_line.next().unwrap().trim().parse().unwrap();

        let cur_hand = get_card_hand(cur_cards, cur_bid);

        card_hand_vec.push(cur_hand);
        card_hand_vec.sort_unstable_by_key(|x| x.score);
    }

    let mut rank: u32 = 1;
    for cur_vec in card_hand_vec {
        total_winnings += cur_vec.bid * rank;
        rank += 1;
    }

    total_winnings
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

fn get_card_hand(input_cards: &str, input_bid: u32) -> CardHand {
    let mut card_counts_map: HashMap<char, u32> = HashMap::new();

    let mut base_score = 0;
    let mut bonus_multiplier = 1;

    for cur_char in input_cards.chars() {
        *card_counts_map.entry(cur_char.to_owned()).or_default() += 1;
        base_score += get_card_value(cur_char);
    }

    let mut biggest_dup: (char, u32) = (' ', 0);
    let mut second_biggest_dup: (char, u32) = (' ', 0);

    for (key, value) in card_counts_map {
        if value > biggest_dup.1 {
            second_biggest_dup = biggest_dup;
            biggest_dup.0 = key;
            biggest_dup.1 = value
        } else if value > second_biggest_dup.1 {
            second_biggest_dup.0 = key;
            second_biggest_dup.1 = value
        }
    }

    // high
    if biggest_dup.1 == 1 {
        // do nothing, we've already found score
    }
    // pair
    else if biggest_dup.1 == 2 && second_biggest_dup.1 < 2 {
        bonus_multiplier = 10;
    }
    // 2-pair
    else if biggest_dup.1 == 2 && second_biggest_dup.1 == 2 {
        // find which dup is worth more
        if get_card_value(second_biggest_dup.0) > get_card_value(biggest_dup.0) {
            let swap_biggest_dup = biggest_dup;
            biggest_dup = second_biggest_dup;
            second_biggest_dup = swap_biggest_dup;
        }

        // don't bother with bonus, just do score
        base_score -= get_card_value(biggest_dup.0) * 2 + get_card_value(second_biggest_dup.0) * 2;

        base_score = (get_card_value(biggest_dup.0) * 200)
            + get_card_value(second_biggest_dup.0) * 20
            + base_score;
    }
    // 3-kind
    else if biggest_dup.1 == 3 && second_biggest_dup.1 < 2 {
        bonus_multiplier = 1_000;
    }
    // full-house
    else if biggest_dup.1 == 3 && second_biggest_dup.1 == 2 {
        bonus_multiplier = 10_000;
    }
    // 4-kind
    else if biggest_dup.1 == 4 {
        bonus_multiplier = 100_000;
    }
    // 5-kind
    else if biggest_dup.1 == 5 {
        bonus_multiplier = 1_000_000;
    } else {
        panic!("Something went wrong when calculating this hand!");
    }

    let new_card_hand = CardHand {
        score: base_score * bonus_multiplier,
        bid: input_bid,
        cards: input_cards.to_string(),
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
    // #[test]
    // fn test_two() {
    //     assert_eq!(part_two(TEST_LINES1), 8);
    // }

    #[test]
    fn test_single_card() {
        assert_eq!(get_card_value('2'), 2);
        assert_eq!(get_card_value('A'), 14);
    }
    #[test]
    fn test_single_hand() {
        // high
        assert_eq!(get_card_hand("65432", 1).score, 20);
        // pair
        assert_eq!(get_card_hand("32T3K", 1).score, 310);
        // 2-pair
        assert_eq!(get_card_hand("KTJJT", 1).score, 2_413);
        // 3-kind
        assert_eq!(get_card_hand("T55J5", 1).score, 36_000);
        // full-house
        assert_eq!(get_card_hand("T55T5", 1).score, 350_000);
        // 4-kind
        assert_eq!(get_card_hand("T5555", 1).score, 3_000_000);
        // 5-kind
        assert_eq!(get_card_hand("55555", 1).score, 25_000_000);
    }
}
