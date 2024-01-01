use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map,
    multi::separated_list1,
    IResult,
};
use std::u32;

fn main() {}

fn scratchcards(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("Card ")(input)?;

    // Card id
    let (input, _) = digit1(input)?;

    // delimited(digit1, );

    Ok((input, vec![]))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(
        space1,
        map(digit1, |s: &str| {
            s.parse::<u32>().expect("should be a digit")
        }),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{parse_numbers, scratchcards};

    #[test]
    fn parse_numbers_returns_numbers() {
        assert_eq!(parse_numbers("1 41 20"), Ok(("", vec![1, 41, 20])));
        assert_eq!(
            parse_numbers("83 86  6 31 17  9 48 53"),
            Ok(("", vec![83, 86, 6, 31, 17, 9, 48, 53]))
        );
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
