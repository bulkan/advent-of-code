use std::{collections::HashMap, ops::Deref};

use itertools::{Itertools, Position};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHand {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq)]
pub struct Hand {
    pub cards_strength: Vec<u32>,
    pub bet: u32,
    pub rank: PokerHand,
}

impl Hand {
    fn get_rank(counts: HashMap<&str, usize>) -> PokerHand {
        use PokerHand::*;

        let values = if let Some(joker_count) = counts.get(&"J") {
            if *joker_count == 5 {
                "5".to_string()
            } else {
                counts
                    .iter()
                    .filter_map(|(key, value)| (key != &"J").then_some(value))
                    .sorted()
                    .with_position()
                    .map(|(position, value)| match position {
                        Position::Last | Position::Only => value + joker_count,
                        _ => *value,
                    })
                    .join("")
            }
        } else {
            counts.values().sorted().join("")
        };

        match values.deref() {
            "5" => FiveOfAKind,
            "14" => FourOfAKind,
            "23" => FullHouse,
            "113" => ThreeOfAKind,
            "122" => TwoPair,
            "1112" => OnePair,
            "11111" => HighCard,
            value => panic!("should never happen. Encountered `{}`", value),
        }
    }

    pub fn new(cards: Vec<&str>, bet: u32) -> Hand {
        let frequency = cards.clone().into_iter().counts();

        // part2  - J is now Joker
        let card_strength_map: HashMap<&str, u32> =
            HashMap::from([("A", 13), ("K", 12), ("Q", 11), ("T", 10), ("J", 1)]);

        let cards_strength = cards
            .iter()
            .map(|c| {
                card_strength_map
                    .get(c)
                    .map_or_else(|| c.parse::<u32>().expect("the card is digit"), |&val| val)
            })
            .collect::<Vec<u32>>();

        Hand {
            cards_strength,
            bet,
            rank: Hand::get_rank(frequency),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hand2::{Hand, PokerHand};

    #[test]
    fn rank_with_jokers() {
        let high_card = Hand::new(vec!["3", "J", "T", "A", "K"], 1);
        let pair = Hand::new(vec!["3", "2", "J", "3", "K"], 1);
        let two_pair = Hand::new(vec!["3", "J", "K", "3", "K"], 1);
        let three_of_a_kind = Hand::new(vec!["T", "5", "5", "J", "5"], 1);
        let full_house = Hand::new(vec!["J", "5", "5", "J", "5"], 1);

        assert_eq!(high_card.rank, PokerHand::OnePair);
        assert_eq!(pair.rank, PokerHand::ThreeOfAKind);
        assert_eq!(two_pair.rank, PokerHand::FullHouse);
        assert_eq!(three_of_a_kind.rank, PokerHand::FourOfAKind);
        assert_eq!(full_house.rank, PokerHand::FiveOfAKind);
    }

    #[test]
    fn hand_rank_is_calculated_correctly() {
        let high_card = Hand::new(vec!["3", "2", "T", "A", "K"], 1);
        let pair = Hand::new(vec!["3", "2", "T", "3", "K"], 1);
        let two_pair = Hand::new(vec!["3", "2", "K", "3", "K"], 1);
        let three_of_a_kind = Hand::new(vec!["T", "5", "5", "A", "5"], 1);
        let full_house = Hand::new(vec!["T", "5", "5", "T", "5"], 1);
        let five_of_a_kind = Hand::new(vec!["5", "5", "5", "5", "5"], 1);
        let four_of_a_kind = Hand::new(vec!["5", "5", "5", "A", "5"], 1);

        assert_eq!(high_card.rank, PokerHand::HighCard);
        assert_eq!(pair.rank, PokerHand::OnePair);
        assert_eq!(two_pair.rank, PokerHand::TwoPair);
        assert_eq!(three_of_a_kind.rank, PokerHand::ThreeOfAKind);
        assert_eq!(full_house.rank, PokerHand::FullHouse);
        assert_eq!(five_of_a_kind.rank, PokerHand::FiveOfAKind);
        assert_eq!(four_of_a_kind.rank, PokerHand::FourOfAKind);
    }
}
