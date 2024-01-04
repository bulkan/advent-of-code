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

// const CARD_STRENGTH: HashMap<&str, u32> = HashMap::from([("A", 14), ("K", 13), ("Q", 12)]);

fn main() {
    todo!();
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    cards: Vec<&'a str>,
    bet: u32,
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Hand>, ErrorTree<&str>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Hand, ErrorTree<&str>> {
    map(separated_pair(parse_hand, space1, complete::u32), |s| {
        Hand {
            cards: s.0,
            bet: s.1,
        }
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

    use crate::{parse_cards, parse_hand, parse_line, Hand};

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
            (
                "",
                Hand {
                    cards: vec!["3", "2", "T", "3", "K"],
                    bet: 765
                }
            )
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
                Hand {
                    cards: vec!["3", "2", "T", "3", "K"],
                    bet: 765
                },
                Hand {
                    cards: vec!["T", "5", "5", "J", "5"],
                    bet: 684
                }
            ]
        );
    }
}
