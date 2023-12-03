#![warn(clippy::pedantic)]
use std::fs;

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
    parse_grid(grid, width.try_into().unwrap());
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

    println!("{:?}", grid);
    println!("{:?}", input.lines().nth(0).unwrap());
    println!("Width: {}", width);
    println!("Height: {}", height);
    println!("Grid length: {}", grid.len());
    println!("Input length: {}", input.replace('\n', "").len());

    grid
}

fn parse_grid(grid: Vec<char>, width: u32) {
    // let symbols_vector: Vec<Coords_2D> = Vec::new();
    // let numbers_vector: Vec<Coords_2D> = Vec::new();
    let sum = 0;

    let mut cur_cell = 0;
    for cell_char in grid {
        if cell_char != '.' && cell_char.is_ascii_punctuation() {
            let coords = get_coords_from_1D(width, cur_cell);
            println!("Found {cell_char} at {cur_cell} coords: {coords:?}");
        }
        cur_cell += 1;
    }
}

fn get_coords_from_1D(width: u32, cell: u32) -> (u32, u32) {
    // cell = y * w + x
    let y = cell / width;
    let x = cell - (y * width);
    (x, y)
}

fn get_1D_from_coords(width: u32, coords: (u32, u32)) -> u32 {
    // cell = y * w + x
    let cell = coords.1 * width + coords.0;
    cell
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
        assert_eq!(get_coords_from_1D(width, 99), (9, 9));
        assert_eq!(get_coords_from_1D(width, 54), (4, 5));

        assert_eq!(get_1D_from_coords(width, (9, 9)), 99);
        assert_eq!(get_1D_from_coords(width, (4, 5)), 54);
    }
    #[test]
    fn test_symbols() {
        let width = TEST_LINES1.lines().nth(0).unwrap().len();
        let test_grid = get_grid_from_input(TEST_LINES1);
        //let symbol_vect =
    }
}
