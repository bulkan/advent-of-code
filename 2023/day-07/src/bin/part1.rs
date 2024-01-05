use std::collections::{BTreeMap, HashMap};

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

    hands.sort_by_key(|hand| hand.strength());

    dbg!(hands);

    0
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    cards: Vec<&'a str>,
    frequency: BTreeMap<&'a str, u32>,
    bet: u32,
}

impl<'a> Hand<'a> {
    fn new(cards: Vec<&'a str>, bet: u32) -> Hand {
        let frequency: BTreeMap<&str, u32> = cards.iter().fold(BTreeMap::new(), |mut acc, card| {
            acc.entry(card).and_modify(|card| *card += 1).or_insert(1);

            acc
        });

        Hand {
            cards,
            bet,
            frequency,
        }
    }

    fn strength(&self) -> u32 {
        let card_strength: HashMap<&str, u32> =
            HashMap::from([("A", 14), ("K", 13), ("Q", 12), ("J", 11), ("T", 10)]);

        self.cards
            .iter()
            .map(|c| {
                card_strength
                    .get(c)
                    .map_or_else(|| c.parse::<u32>().expect("the card is digit"), |&val| val)
            })
            .sum::<u32>()
    }

    fn is_full_house(&self) -> bool {
        self.frequency.len() == 2 && self.frequency.values().any(|frequency| *frequency == 3)
    }
}

// impl PartialEq for Hand {
//     fn eq(&self, other: &Self) -> bool {
//         // Define your custom equality logic here
//         self.field1 == other.field1 && self.field2.len() == other.field2.len()
//     }
// }

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
    use std::collections::BTreeMap;

    use indoc::indoc;

    use crate::{camel_cards, parse_cards, parse_hand, parse_line, Hand};

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

    #[test]
    fn hand_strength_works() {
        let hands = [
            Hand::new(vec!["3", "2", "T", "3", "K"], 765),
            Hand::new(vec!["T", "5", "5", "J", "5"], 684),
        ];

        assert_eq!(hands[0].strength(), 31);
        assert_eq!(hands[1].strength(), 36);
    }

    #[test]
    fn hand_frequency_is_correct() {
        let full_house = Hand::new(vec!["K", "K", "K", "A", "A"], 0);
        assert_eq!(full_house.frequency, BTreeMap::from([("K", 3), ("A", 2)]));

        assert!(full_house.is_full_house());

        let pair = Hand::new(vec!["K", "K", "2", "A", "3"], 0);
        assert_eq!(
            pair.frequency,
            BTreeMap::from([("K", 2), ("A", 1), ("2", 1), ("3", 1)])
        );

        assert!(!pair.is_full_house());
    }

    // #[test]
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
}
