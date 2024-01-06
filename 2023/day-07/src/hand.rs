use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHand {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    Unknown = 0,
}

#[derive(Debug, PartialEq)]
pub struct Hand<'a> {
    cards: Vec<&'a str>,
    pub cards_strength: Vec<u32>,
    pub bet: u32,
    pub rank: PokerHand,
}

impl<'a> Hand<'a> {
    fn get_rank(frequency: BTreeMap<&str, u32>) -> PokerHand {
        match frequency.len() {
            1 => PokerHand::FiveOfAKind,
            2 => {
                let mut values = frequency.values();
                if values.all(|v| *v == 4 || *v == 1) {
                    PokerHand::FourOfAKind
                } else if values.all(|v| *v == 3 || *v == 2) {
                    PokerHand::FullHouse
                } else {
                    PokerHand::Unknown
                }
            }
            3 => {
                let mut values = frequency.values();

                if values.any(|v| *v == 3) {
                    PokerHand::ThreeOfAKind
                } else if values.all(|v| *v == 2 || *v == 1) {
                    PokerHand::TwoPair
                } else {
                    PokerHand::Unknown
                }
            }
            4 => PokerHand::OnePair,
            5 => PokerHand::HighCard,
            _ => PokerHand::Unknown,
        }
    }

    pub fn new(cards: Vec<&'a str>, bet: u32) -> Hand {
        let frequency: BTreeMap<&str, u32> = cards.iter().fold(BTreeMap::new(), |mut acc, card| {
            acc.entry(card).and_modify(|card| *card += 1).or_insert(1);

            acc
        });

        let card_strength_map: HashMap<&str, u32> =
            HashMap::from([("A", 14), ("K", 13), ("Q", 12), ("J", 11), ("T", 10)]);

        let cards_strength = cards
            .iter()
            .map(|c| {
                card_strength_map
                    .get(c)
                    .map_or_else(|| c.parse::<u32>().expect("the card is digit"), |&val| val)
            })
            .collect::<Vec<u32>>();

        Hand {
            cards,
            cards_strength,
            bet,
            rank: Hand::get_rank(frequency),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hand::{Hand, PokerHand};

    #[test]
    fn hand_rank_is_calculated_correctly() {
        let high_card = Hand::new(vec!["3", "2", "T", "A", "K"], 1);
        let pair = Hand::new(vec!["3", "2", "T", "3", "K"], 1);
        let two_pair = Hand::new(vec!["3", "2", "K", "3", "K"], 1);
        let three_of_a_kind = Hand::new(vec!["T", "5", "5", "J", "5"], 1);
        let full_house = Hand::new(vec!["T", "5", "5", "T", "5"], 1);
        let five_of_a_kind = Hand::new(vec!["5", "5", "5", "5", "5"], 1);
        let four_of_a_kind = Hand::new(vec!["5", "5", "5", "J", "5"], 1);

        assert_eq!(high_card.rank, PokerHand::HighCard);
        assert_eq!(pair.rank, PokerHand::OnePair);
        assert_eq!(two_pair.rank, PokerHand::TwoPair);
        assert_eq!(three_of_a_kind.rank, PokerHand::ThreeOfAKind);
        assert_eq!(full_house.rank, PokerHand::FullHouse);
        assert_eq!(five_of_a_kind.rank, PokerHand::FiveOfAKind);
        assert_eq!(four_of_a_kind.rank, PokerHand::FourOfAKind);
    }
}
