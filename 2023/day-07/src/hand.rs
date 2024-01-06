use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq)]
enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    FullHouse,
    ThreeOfAKind,
    FourOfAKind,
    FiveOfAKind,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Hand<'a> {
    cards: Vec<&'a str>,
    bet: u32,
    rank: PokerHand,
}

impl<'a> Hand<'a> {
    fn get_rank(frequency: BTreeMap<&str, u32>) -> PokerHand {
        match frequency.len() {
            1 => PokerHand::FiveOfAKind,
            2 => {
                let mut values = frequency.values();
                if values.all(|v| *v == 3 || *v == 2) {
                    PokerHand::FullHouse
                } else if values.all(|v| *v == 4 || *v == 1) {
                    PokerHand::FourOfAKind
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

        // frequency.len() == self.frequency.values().all(|v| *v == 3 || *v == 2)
    }

    pub fn new(cards: Vec<&'a str>, bet: u32) -> Hand {
        let frequency: BTreeMap<&str, u32> = cards.iter().fold(BTreeMap::new(), |mut acc, card| {
            acc.entry(card).and_modify(|card| *card += 1).or_insert(1);

            acc
        });

        Hand {
            cards,
            bet,
            rank: Hand::get_rank(frequency),
        }
    }

    pub fn strength(&self) -> u32 {
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
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::hand::{Hand, PokerHand};

    #[test]
    fn hand_rank_is_calculated_correctly() {
        let high_card = Hand::new(vec!["3", "2", "T", "A", "K"], 1);
        let pair = Hand::new(vec!["3", "2", "T", "3", "K"], 1);
        let two_pair = Hand::new(vec!["3", "2", "K", "3", "K"], 1);
        let three_of_a_kind = Hand::new(vec!["T", "5", "5", "J", "5"], 1);
        let five_of_a_kind = Hand::new(vec!["5", "5", "5", "5", "5"], 1);
        let four_of_a_kind = Hand::new(vec!["5", "5", "5", "J", "5"], 1);

        assert_eq!(high_card.rank, PokerHand::HighCard);
        assert_eq!(five_of_a_kind.rank, PokerHand::FiveOfAKind);
        assert_eq!(four_of_a_kind.rank, PokerHand::FourOfAKind);
        assert_eq!(pair.rank, PokerHand::OnePair);
        assert_eq!(two_pair.rank, PokerHand::TwoPair);
        assert_eq!(three_of_a_kind.rank, PokerHand::ThreeOfAKind);
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
        // assert_eq!(full_house.frequency, BTreeMap::from([("K", 3), ("A", 2)]));

        // assert!(!full_house.is_full_house());

        let pair = Hand::new(vec!["K", "K", "2", "A", "3"], 0);
        // assert_eq!(
        //     pair.frequency,
        //     BTreeMap::from([("K", 2), ("A", 1), ("2", 1), ("3", 1)])
        // );

        // assert!(!pair.is_full_house());
    }
}
