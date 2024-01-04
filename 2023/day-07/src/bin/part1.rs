use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{self, digit0, digit1},
    multi::{many0, many1, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;

// const CARD_STRENGTH: HashMap<&str, u32> = HashMap::from([("A", 14), ("K", 13), ("Q", 12)]);

fn main() {
    todo!();
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    cards: Vec<&'a str>,
    bet: u32,
}

// fn parse_cards(input: &str) -> IResult<&str, Vec<Hand>> {
//     many0(pair(parse_hand, complete::u32))(input)
// }

fn parse_hand(input: &str) -> IResult<&str, Vec<&str>, ErrorTree<&str>> {
    many1(alt((
        tag("A"),
        tag("K"),
        tag("Q"),
        tag("J"),
        tag("T"),
        tag("9"),
        tag("8"),
        tag("7"),
        tag("6"),
        tag("5"),
        tag("4"),
        tag("3"),
        tag("2"),
    )))(input)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::parse_hand;

    #[test]
    fn parse_hand_works() {
        let hand = "T55J5";

        assert_eq!(
            parse_hand(hand).unwrap(),
            ("", vec!["T", "5", "5", "J", "5"])
        );
    }

    fn should_work() {
        let input = indoc! {"
          32T3K 765
          T55J5 684
          KK677 28
          KTJJT 220
          QQQJA 483
        "};

        assert_eq!(1, 2);
    }
}
