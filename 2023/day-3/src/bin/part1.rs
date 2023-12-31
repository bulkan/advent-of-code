use std::fs;

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
        println!("Current: {current:?}");

        // Peek at the next value
        if let Some(&next) = iterator.peek() {
            // Use the next value without consuming it
            println!("Next (peeked): {next:?}");
        } else {
            // Handle the case where there is no next value
            println!("No next value");
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::gear_ratios;

    #[test]
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
