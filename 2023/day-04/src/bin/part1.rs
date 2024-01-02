use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::u32;

fn main() {}

fn parse_line(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = tag("Card ")(input)?;

    // card id
    let (input, _) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;

    parse_card(input)
}

// 13 2 | 2 11
fn parse_card(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, (winning_numbers, numbers)) =
        separated_pair(parse_numbers, tag(" | "), parse_numbers)(input)?;

    Ok((input, (winning_numbers, numbers)))
}

// 13 2 1
fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, nums) = separated_list1(
        space1,
        map(digit1, |s: &str| {
            s.parse::<u32>().expect("should be a digit")
        }),
    )(input)?;

    Ok((input, nums))
}

#[cfg(test)]
mod tests {
    use crate::{parse_card, parse_line, parse_numbers};

    #[test]
    fn parse_numbers_returns_numbers() {
        assert_eq!(parse_numbers("1 41 20"), Ok(("", vec![1, 41, 20])));
        assert_eq!(
            parse_numbers("83 86  6 31 17  9 48 53"),
            Ok(("", vec![83, 86, 6, 31, 17, 9, 48, 53]))
        );
    }

    #[test]
    fn parse_card_handles_preceeding_text() {
        let card = "41 20 | 1 2";
        let winning_numbers = vec![41, 20];
        let numbers = vec![1, 2];
        let result = parse_card(card);

        assert_eq!(result, Ok(("", (winning_numbers, numbers))));
    }

    #[test]
    fn parse_game_line_works() {
        let input = "Card 22: 13 2 | 12 2";

        let result = parse_line(input);

        assert_eq!(result, Ok(("", (vec![13, 2], vec![12, 2]))));
    }

    //     fn it_works_as_expected() {
    //         let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    //
    //         let results = scratchcards(input);
    //     }
}
