#![warn(clippy::pedantic)]
use std::fs;

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

    //println!("Part 1:\n{}", get_sum(input));
    //println!("Part 2:\n{}", get_sum_powers(input));
    let width = input.lines().nth(0).unwrap().len();

    let grid = get_grid_from_input(input);
    parse_grid(
        grid,
        width.try_into().unwrap(),
        input.lines().count().try_into().unwrap(),
    );
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

    #[test]
    fn test_grid_dimensions() {
        let test_grid = get_grid_from_input(TEST_LINES1);
        assert_eq!(test_grid.len(), 100);
    }
    #[test]
    fn test_coords() {
        let width = 10;
        let coords_test1 = get_coords_from_1D(width, 99);
        assert_eq!(coords_test1.x, 9);
        assert_eq!(coords_test1.y, 9);
        let coords_test2 = get_coords_from_1D(width, 54);
        assert_eq!(coords_test2.x, 4);
        assert_eq!(coords_test2.y, 5);

        assert_eq!(get_1D_from_coords(width, Coords_2D { x: 9, y: 9 }), 99);
        assert_eq!(get_1D_from_coords(width, Coords_2D { x: 4, y: 5 }), 54);
    }

    #[test]
    fn test_count_symbols() {
        assert_eq!(count_symbols(TEST_LINES1), 6)
    }

    #[test]
    fn test_grid_parse() {
        let width = TEST_LINES1.lines().nth(0).unwrap().len();
        let test_grid = get_grid_from_input(TEST_LINES1);
        let sum = parse_grid(
            test_grid,
            width.try_into().unwrap(),
            TEST_LINES1.lines().count().try_into().unwrap(),
        );
        assert_eq!(sum, 4361);
    }
}
