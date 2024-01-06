use day_07::hand::Hand;

use nom::character::streaming::line_ending;
use nom::combinator::map;
use nom::{
    branch::alt,
    character::complete::{self, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;

fn main() {
    let input = include_str!("./input.txt");

    camel_cards(input);
}

fn camel_cards(input: &str) -> u32 {
    let (_, mut hands) = parse_cards(input).expect("parsing failed");

    hands.sort_by(|a: &Hand, b: &Hand| {
        let ordering = a.rank.cmp(&b.rank);

        match ordering {
            std::cmp::Ordering::Equal => a.cards_strength.cmp(&b.cards_strength),
            _ => ordering,
        }
    });

    dbg!(&hands);

    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bet * (1 + u32::try_from(index).expect("couldn't u32 the index")))
        .sum::<u32>()
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Hand>, ErrorTree<&str>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Hand, ErrorTree<&str>> {
    map(separated_pair(parse_hand, space1, complete::u32), |s| {
        Hand::new(s.0, s.1)
    })(input)
}

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

    use crate::{camel_cards, parse_cards, parse_hand, parse_line, Hand};

    #[test]
    fn it_should_work_with_example() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        assert_eq!(camel_cards(input), 6440);
    }

    #[test]
    fn parse_hand_works() {
        let hand = "T55J5";

        assert_eq!(
            parse_hand(hand).unwrap(),
            ("", vec!["T", "5", "5", "J", "5"])
        );
    }

    #[test]
    fn parse_line_works() {
        let input = "32T3K 765";

        assert_eq!(
            parse_line(input).unwrap(),
            ("", Hand::new(vec!["3", "2", "T", "3", "K"], 765))
        );
    }

    #[test]
    fn parse_cards_works() {
        let input = indoc! {"
          32T3K 765
          T55J5 684
        "};

        let (_, hands) = parse_cards(input).unwrap();

        assert_eq!(
            hands,
            vec![
                Hand::new(vec!["3", "2", "T", "3", "K"], 765),
                Hand::new(vec!["T", "5", "5", "J", "5"], 684)
            ]
        );
    }
}
