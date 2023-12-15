#![warn(clippy::pedantic)]
use std::fs;

/// # Panics
///
/// Will panic if file paths are incorrect
#[must_use]
pub fn get_input(is_bench: bool) -> String {
    let binding = if is_bench {
        fs::read_to_string("../day-09/inputs/input.txt").unwrap()
    } else {
        fs::read_to_string("./day-09/inputs/input.txt").unwrap()
    };
    binding.trim().to_string()
}

/// # Panics
///
/// Will panic if input is incorrect
pub fn part_one(input: &str) -> i64 {
    let mut history_sum = 0;
    for line in input.lines() {
        println!("{line}");
        // create vector of numbers
        let original_line_vec: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // Find various differences
        let mut line_difference_sum = *original_line_vec.last().unwrap();
        let mut cur_difference_tuple = get_difference_vector(original_line_vec);
        while !cur_difference_tuple.1 {
            line_difference_sum += cur_difference_tuple.0.last().unwrap();
            cur_difference_tuple = get_difference_vector(cur_difference_tuple.0);
        }
        line_difference_sum += cur_difference_tuple.0.last().unwrap();
        //println!("{line_difference_sum}");
        history_sum += line_difference_sum;
    }
    history_sum
}

fn get_difference_vector(input_vec: Vec<i64>) -> (Vec<i64>, bool) {
    let mut cur_difference_vec: Vec<i64> = vec![];
    let mut same_as_last_count = 0;
    for i in 0..input_vec.len() - 1 {
        let cur_difference = input_vec[i + 1] - input_vec[i];
        cur_difference_vec.push(cur_difference);

        // Check how many of the same differences we have
        if cur_difference_vec.len() > 1 && cur_difference_vec[i] == cur_difference_vec[i - 1] {
            same_as_last_count += 1;
        }
    }

    if same_as_last_count == cur_difference_vec.len() - 1 {
        (cur_difference_vec, true)
    } else {
        (cur_difference_vec, false)
    }
}
/// # Panics
///
/// Will panic if input is incorrect
pub fn part_two(input: &str) -> i64 {
    0
}
