use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashSet, u32};

fn main() {
    let scratchcards = include_str!("./input.txt");

    let points = process_scratchcards(scratchcards);

    println!("{points}");
}

fn process_scratchcards(input: &str) -> u32 {
    let points = input
        .lines()
        .map(|line| {
            let (_, (winning_numbers, numbers)) = parse_line(line).expect("should exist yo");

            let total_winning_numbers = winning_numbers.intersection(&numbers).count() as u32;

            dbg!(&total_winning_numbers);

            if total_winning_numbers == 0 {
                0
            } else if total_winning_numbers == 1 {
                1
            } else {
                (1..total_winning_numbers).fold(1, |acc, _| acc * 2)
            }
        })
        .sum();

    points
}

// Card 1: 12 1 | 23 32
fn parse_line(input: &str) -> IResult<&str, (HashSet<u32>, HashSet<u32>)> {
    let (input, _) = tag("Card ")(input)?;

    // card id
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":")(input)?;

    parse_card(input)
}

// 13 2 | 2 11
fn parse_card(input: &str) -> IResult<&str, (HashSet<u32>, HashSet<u32>)> {
    let (input, (winning_numbers, numbers)) =
        separated_pair(parse_numbers, tag(" | "), parse_numbers)(input)?;

    Ok((input, (winning_numbers, numbers)))
}

// 13 2 1
fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    let (input, _) = space0(input)?;

    let (input, nums) = separated_list1(space1, parse_digit)(input)?;

    let nums = nums.into_iter().collect::<HashSet<_>>();

    Ok((input, nums))
}

fn parse_digit(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{parse_card, parse_line, parse_numbers, process_scratchcards};

    #[test]
    fn parse_numbers_returns_numbers() {
        assert_eq!(
            parse_numbers("83 86   6 31 17 9 48 53"),
            Ok(("", HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])))
        );

        assert_eq!(
            parse_numbers("  1  41   20"),
            Ok(("", HashSet::from([1, 41, 20])))
        );
    }

    #[test]
    fn parse_card_handles_preceeding_text() {
        let card = "  41 20 | 1 2";
        let winning_numbers = HashSet::from([41, 20]);
        let numbers = HashSet::from([1, 2]);
        let result = parse_card(card);

        assert_eq!(result, Ok(("", (winning_numbers, numbers))));
    }

    #[test]
    fn parse_game_line_works() {
        let input = "Card 22: 13 2 | 12 2";

        let result = parse_line(input);

        assert_eq!(
            result,
            Ok(("", (HashSet::from([13, 2]), HashSet::from([12, 2]))))
        );
    }

    #[test]
    fn parse_game_line_works_with_weird_input() {
        let input = "Card 3: 1 21 |  14  1";

        let result = parse_line(input);

        assert_eq!(
            result,
            Ok(("", (HashSet::from([1, 21]), HashSet::from([14, 1]))))
        );
    }

    #[test]
    fn it_works_as_expected() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let points = process_scratchcards(input);

        assert_eq!(points, 13);
    }
}
