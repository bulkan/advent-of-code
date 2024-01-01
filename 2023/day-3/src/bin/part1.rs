use std::fs;
use std::ops::Neg;

fn main() {
    let schematic = fs::read_to_string("input.txt").expect("file should exist");

    let res = gear_ratios(schematic.trim());

    dbg!(res);
}

// current line will need to peek at second line
//    if there is second line then while iterating over first line
//        skip over periods, when a digit is found,
//            keep track of start pos and end pos, digit as string
//
fn gear_ratios(schematic: &str) -> u32 {
    let lines_of_chars = schematic
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut iterator = lines_of_chars.iter().peekable();

    while let Some(current) = iterator.next() {
        // Use the current value
        println!("{current:?}");

        // Peek at the next value
        if let Some(&next) = iterator.peek() {
            // Use the next value without consuming it
            println!("\t{next:?}");
        }
    }

    0
}

// look at line2 for any symbols and get all the indexes as HashSet
// iterate over line1
//   if char is not '.' and is a digit
//     add it a to a string
//
//
//   check if digit is start of
//   if current char is '.' add string
//
//
fn get_numbers_from_lines(line1: Vec<char>, line2: Vec<char>) -> Vec<u32> {
    let mut part_number: Vec<char> = vec![];

    let mut numbers: Vec<u32> = vec![];

    let mut start: usize = 0;
    let mut end: usize = 0;

    let mut line1_iter = line1.iter().enumerate().peekable();

    while let Some((i, &char)) = line1_iter.next() {
        if char.is_ascii_digit() {
            part_number.push(char);
            if part_number.is_empty() {
                start = i;
            }

            if let Some((_i, &next_char)) = line1_iter.peek() {
                if next_char == '.' {
                    end = i;
                    part_number.clear();
                }

                if !next_char.is_ascii_digit() && next_char != '.' {
                    let p: u32 = part_number
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .expect("should be a number");

                    numbers.push(p);
                }
            }
        }
    }

    dbg!(numbers, start, end);

    vec![1]
}

#[cfg(test)]
mod tests {
    use crate::{gear_ratios, get_numbers_from_lines};

    #[test]
    fn get_numbers_from_lines_returns_numbers() {
        let line1 = vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'];
        let line2 = vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'];
        let result = get_numbers_from_lines(line1, line2);

        assert_eq!(result, [467]);
    }

    fn it_works_as_expected() {
        let schematic = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim();

        let result = gear_ratios(schematic);
        assert_eq!(result, 4361);
    }
}
