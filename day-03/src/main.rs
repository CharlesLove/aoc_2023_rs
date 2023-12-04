#![warn(clippy::pedantic)]
use std::{borrow::BorrowMut, fs};

#[derive(Debug, Clone, Copy)]
struct Coords_2D {
    x: u32,
    y: u32,
}

struct Number {
    number: u32,
    left_bound: Coords_2D,
    right_bound: Coords_2D,
}

/*
 *  Game plan:
 *  We have a grid of digits and symbols, rust allows you to check both
 *  I plan to iterate through the grid, checking if symbol or digit
 *  If digit:
 *      Create a temporary string and insert that digit, while also
 *      noting the left bound (have these be a struct of some sort?)
 *      (number, left[x,y], right[x,y])
 *
 *      I then move forward as usual, and if the chain hasn't been unbroken,
 *      add another digit to the string. When chain is broken by a . or symbol,
 *      add to a vector of previously described number structs
 *
 *  If symbol:
 *      Add to vector with a struct of the symbols x and y coordinates,
 *      symbol name doesn't really matter
 *
 *  Then, I iterate through the symbol vector and check for collisions
 *  with any digits, if so remove them from the vector and add that number to
 *  the sum.
 *
 *  Potentially easier game plan:
 *      Only scan for symbols. When one is found, do a scan around until a
 *      digit is found. If it is, take a break and scan left and right to
 *      get the full number (replacing those chars with periods as you go).
 *      Once the extent of it is discovered, add the number to the sum and
 *      continue looking around the symbol.
*/

fn main() {
    let binding = fs::read_to_string("./day-03/inputs/input.txt").unwrap();
    let input = binding.trim();

    // 1382231 is too high
    println!("Part 1:\n{}", get_sum(input));
    //println!("Part 2:\n{}", get_sum_powers(input));
    //let width = get_width(&input);
    //let height = get_height(&input);

    //let grid = get_grid_from_input(&input);
    // parse_grid(
    //     grid,
    //     width.try_into().unwrap(),
    //     input.lines().count().try_into().unwrap(),
    // );

    //parse_input(input, width, height);
}

fn get_grid_from_input(input: &str) -> Vec<char> {
    // lines actually removes the new line so I don't have to worry about it
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();

    // Try sticking with a 1d array using something like
    // arr[y * w + x]
    let mut grid: Vec<char> = Vec::new();
    for current_char in input.replace('\n', "").chars() {
        grid.push(current_char);
    }

    // println!("{:?}", grid);
    // println!("{:?}", input.lines().nth(0).unwrap());
    // println!("Width: {}", width);
    // println!("Height: {}", height);
    // println!("Grid length: {}", grid.len());
    // println!("Input length: {}", input.replace('\n', "").len());

    grid
}

fn parse_grid(grid: Vec<char>, width: u32, height: u32) -> u32 {
    // let symbols_vector: Vec<Coords_2D> = Vec::new();
    // let numbers_vector: Vec<Coords_2D> = Vec::new();
    let sum: u32 = 0;

    let mut cur_cell = 0;
    for cell_char in grid.iter() {
        // we found a symbol
        if is_symbol(*cell_char) {
            let coords = get_coords_from_1D(width, cur_cell);
            // println!("Found {cell_char} at {coords:?}");

            // check around the symbol for a number
            // up left
            {
                let checked_x: i32 = (coords.x - 1).try_into().unwrap();
                let checked_y: i32 = (coords.y - 1).try_into().unwrap();

                if checked_x > 0 || checked_y > 0 {
                    let checked_coords = Coords_2D {
                        x: checked_x as u32,
                        y: checked_y as u32,
                    };
                    let checked_cell = get_1D_from_coords(width, checked_coords);
                    let checked_char = grid[checked_cell];

                    if checked_char.is_ascii_digit() {
                        println!("We found {checked_char} at {0:?}", checked_coords)
                    }
                }
            }
            // up
            // up right
            // left
            // right
            // down left
            // down
            // down right
        }
        cur_cell += 1;
    }

    sum
}

fn is_symbol(c: char) -> bool {
    if c != '.' && c.is_ascii_punctuation() {
        return true;
    }
    false
}

fn get_coords_from_1D(width: u32, cell: u32) -> Coords_2D {
    let mut coords: Coords_2D = Coords_2D { x: 0, y: 0 };
    // cell = y * w + x
    coords.y = cell / width;
    coords.x = cell - (coords.y * width);
    coords
}

fn get_1D_from_coords(width: u32, coords: Coords_2D) -> usize {
    // cell = y * w + x
    let cell: usize = (coords.y * width + coords.x).try_into().unwrap();
    cell
}

fn count_symbols(input: &str) -> u32 {
    let mut symbol_count = 0;
    for cur_char in input.replace('\n', "").chars() {
        if is_symbol(cur_char) {
            symbol_count += 1;
        }
    }
    symbol_count
}

fn get_check_dir(input: &str, checked_x: usize, checked_y: usize) -> Option<char> {
    input.lines().nth(checked_y).unwrap().chars().nth(checked_x)
}

fn get_width(input: &str) -> usize {
    input.lines().nth(0).unwrap().len()
}

fn get_height(input: &str) -> usize {
    input.lines().count()
}

// instead of all this grid stuff, why not just operate on input?
fn parse_input(mut input: String, width: usize, height: usize) {
    for cur_y in 0..height {
        let cur_line = input.lines().nth(cur_y).unwrap();
        for cur_x in 0..width {
            let cur_char = input
                .lines()
                .nth(cur_y)
                .unwrap()
                .chars()
                .nth(cur_x)
                .unwrap();
            if is_symbol(cur_char) {
                //println!("{}", cur_char);
                let mut checked_x = cur_x;
                let mut checked_y = cur_y;

                let mut check_dir = get_check_dir(&input, checked_x, checked_y);

                // up left
                checked_x = cur_x - 1;
                checked_y = cur_y - 1;
                check_dir = get_check_dir(&input, checked_x, checked_y);
                if check_dir.is_some() {
                    if check_dir.unwrap().is_ascii_digit() {
                        println!("{} at {}, {}", check_dir.unwrap(), checked_x, checked_y);
                    }
                }
                // up
                checked_x = cur_x;
                checked_y = cur_y - 1;
                check_dir = get_check_dir(&input, checked_x, checked_y);
                if check_dir.is_some() {
                    if check_dir.unwrap().is_ascii_digit() {
                        println!("{} at {}, {}", check_dir.unwrap(), checked_x, checked_y);

                        // TODO: get full number
                        let mut num_string = check_dir.unwrap().to_string();
                        // go left
                        let mut l_i = 1;
                        let mut left_char = get_check_dir(&input, checked_x - l_i, checked_y);
                        while left_char.is_some() {
                            num_string.insert_str(0, &left_char.unwrap().to_string());

                            // change input to period if possible
                            // if not, have to do number first approach
                            //input.replace_range(2..2, "+");

                            l_i += 1;
                            left_char = get_check_dir(&input, checked_x - l_i, checked_y);
                        }
                    }
                }
                // up right
                checked_x = cur_x + 1;
                checked_y = cur_y - 1;
                check_dir = get_check_dir(&input, checked_x, checked_y);
                if check_dir.is_some() {
                    if check_dir.unwrap().is_ascii_digit() {
                        println!("{} at {}, {}", check_dir.unwrap(), checked_x, checked_y);
                    }
                }
                // left
                checked_x = cur_x - 1;
                checked_y = cur_y;
                check_dir = get_check_dir(&input, checked_x, checked_y);
                if check_dir.is_some() {
                    if check_dir.unwrap().is_ascii_digit() {
                        println!("{} at {}, {}", check_dir.unwrap(), checked_x, checked_y);
                    }
                }
                // right
                checked_x = cur_x + 1;
                checked_y = cur_y;
                check_dir = get_check_dir(&input, checked_x, checked_y);
                if check_dir.is_some() {
                    if check_dir.unwrap().is_ascii_digit() {
                        println!("{} at {}, {}", check_dir.unwrap(), checked_x, checked_y);
                    }
                }
                // down left
                checked_x = cur_x - 1;
                checked_y = cur_y + 1;
                check_dir = get_check_dir(&input, checked_x, checked_y);
                if check_dir.is_some() {
                    if check_dir.unwrap().is_ascii_digit() {
                        println!("{} at {}, {}", check_dir.unwrap(), checked_x, checked_y);
                    }
                }
                // down
                checked_x = cur_x;
                checked_y = cur_y + 1;
                check_dir = get_check_dir(&input, checked_x, checked_y);
                if check_dir.is_some() {
                    if check_dir.unwrap().is_ascii_digit() {
                        println!("{} at {}, {}", check_dir.unwrap(), checked_x, checked_y);
                    }
                }
                // down right
                checked_x = cur_x + 1;
                checked_y = cur_y + 1;
                check_dir = get_check_dir(&input, checked_x, checked_y);
                if check_dir.is_some() {
                    if check_dir.unwrap().is_ascii_digit() {
                        println!("{} at {}, {}", check_dir.unwrap(), checked_x, checked_y);
                    }
                }
            }
        }
    }
}

fn get_sum(input: &str) -> u32 {
    let mut sum = 0;
    // change iterate through input: line by line, char by char
    // if we find a digit, look around it if symbol found mark summable as true
    // build out the string digit until we hit a non-digit
    // then if summable add to sum

    let input_width = get_width(input);
    let input_height = get_height(input);

    let mut cur_digit_string = "".to_string();
    let mut summable = false;

    for cur_y in 0..input_height {
        let cur_line = input.lines().nth(cur_y).unwrap();
        for cur_x in 0..input_width {
            let cur_char = cur_line.chars().nth(cur_x).unwrap();
            //println!("{}", cur_char);
            if cur_char.is_ascii_digit() {
                cur_digit_string.push(cur_char);

                // check around digit to see if summable
                for y_range in -1..=1 {
                    let checked_y: i32 = cur_y as i32 + y_range;
                    if checked_y < 0 || checked_y >= input_height as i32 {
                        continue;
                    }
                    for x_range in -1..=1 {
                        let checked_x: i32 = cur_x as i32 + x_range;
                        if checked_x < 0 || checked_x >= input_width as i32 {
                            continue;
                        }

                        let checked_char =
                            get_check_dir(input, checked_x as usize, checked_y as usize);

                        if checked_char.is_some() {
                            if checked_char.unwrap().is_ascii_punctuation()
                                && checked_char.unwrap() != '.'
                            {
                                summable = true;
                            }

                            // if first digit, only check:
                            //      up-left, up, left, down-left,
                            //      down
                            // otherwise check:
                            //      up, up-right, right, down,
                            //      down-right
                            // match y_range {
                            //     -1 => surrounding_string_above.push(checked_char.unwrap()),
                            //     0 => surrounding_string_on.push(checked_char.unwrap()),
                            //     1 => surrounding_string_below.push(checked_char.unwrap()),
                            //     _ => todo!(),
                            // }
                        }
                    }
                }
            } else if cur_digit_string.len() > 0 {
                if summable {
                    //println!("{cur_digit_string}");
                    print_surrounding(input, cur_x, cur_y, cur_digit_string.len());
                    sum += &cur_digit_string.parse().unwrap();
                    //println!("{surrounding_string_above}\n{surrounding_string_on}\n{surrounding_string_below}\n");
                }
                summable = false;
                cur_digit_string = "".to_string();
            }
        }
    }

    sum
}

fn print_surrounding(input: &str, cur_x: usize, cur_y: usize, length: usize) {
    let mut surrounding_string = "".to_string();
    let mut top_bound = 0;
    if cur_y as i32 - 1 > 0 {
        top_bound = cur_y - 1;
    }
    let mut left_bound = 0;
    if cur_x as i32 - length as i32 - 1 > 0 {
        left_bound = cur_x - length - 1;
    }
    let mut right_bound = 0;
    if cur_x < get_width(input) {
        right_bound = cur_x;
    }
    let mut bottom_bound = 0;
    if cur_y + 1 < get_height(input) {
        bottom_bound = cur_y + 1;
    }

    for y in top_bound..=bottom_bound {
        for x in left_bound..=right_bound {
            //let cur_line = input.lines().nth(y).unwrap();
            let cur_char = input.lines().nth(y).unwrap().chars().nth(x).unwrap();
            surrounding_string.push(cur_char);
        }
        surrounding_string.push('\n');
    }

    println!("line: {cur_y} char: {cur_x}");
    println!("{surrounding_string}");
}

fn find_numbers_sum(input: &str) -> u32 {
    let sum: u32 = 0;
    let num_vec: Vec<&str> = Vec::new();
    let mut cur_num_string = "".to_string();

    let mut cur_x = 0;
    let mut cur_y = 0;
    for line in input.lines() {
        cur_x = 0;
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                cur_num_string.push(ch);
            } else {
                if cur_num_string != "" {
                    print_surrounding(input, cur_x, cur_y, cur_num_string.len());
                    cur_num_string = "".to_string();
                }
            }
            cur_x += 1;
        }
        if cur_num_string != "" {
            cur_num_string = "".to_string();
        }
        cur_y += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LINES1: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    // #[test]
    // fn test_grid_dimensions() {
    //     let test_grid = get_grid_from_input(TEST_LINES1);
    //     assert_eq!(test_grid.len(), 100);
    // }
    // #[test]
    // fn test_coords() {
    //     let width = 10;
    //     let coords_test1 = get_coords_from_1D(width, 99);
    //     assert_eq!(coords_test1.x, 9);
    //     assert_eq!(coords_test1.y, 9);
    //     let coords_test2 = get_coords_from_1D(width, 54);
    //     assert_eq!(coords_test2.x, 4);
    //     assert_eq!(coords_test2.y, 5);

    //     assert_eq!(get_1D_from_coords(width, Coords_2D { x: 9, y: 9 }), 99);
    //     assert_eq!(get_1D_from_coords(width, Coords_2D { x: 4, y: 5 }), 54);
    // }

    // #[test]
    // fn test_count_symbols() {
    //     assert_eq!(count_symbols(TEST_LINES1), 6)
    // }

    // #[test]
    // fn test_grid_parse() {
    //     let width = TEST_LINES1.lines().nth(0).unwrap().len();
    //     let test_grid = get_grid_from_input(TEST_LINES1);
    //     let sum = parse_grid(
    //         test_grid,
    //         width.try_into().unwrap(),
    //         TEST_LINES1.lines().count().try_into().unwrap(),
    //     );
    //     assert_eq!(sum, 4361);
    // }

    // #[test]
    // fn test_parse_input() {
    //     //assert_eq!('+'.is_ascii_punctuation(), true);
    //     parse_input(
    //         TEST_LINES1.to_string(),
    //         get_width(TEST_LINES1),
    //         get_height(TEST_LINES1),
    //     );
    //     assert_eq!(0, 4361);
    // }

    #[test]
    fn test_get_sum() {
        //assert_ne!(get_sum(TEST_LINES1), 4533);
        assert_eq!(find_numbers_sum(TEST_LINES1), 4361);
    }
}
