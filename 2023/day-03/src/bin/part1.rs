fn main() {
    let schematic = include_str!("./input.txt");

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

#[cfg(test)]
mod tests {
    use crate::gear_ratios;

    // fn get_numbers_from_lines_returns_numbers() {
    //     let line1 = vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'];
    //     let line2 = vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'];
    //     let result = get_numbers_from_lines(line1, line2);
    //
    //     assert_eq!(result, [467]);
    // }

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
