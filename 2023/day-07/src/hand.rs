use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq)]
pub struct Hand<'a> {
    cards: Vec<&'a str>,
    frequency: BTreeMap<&'a str, u32>,
    bet: u32,
}

impl<'a> Hand<'a> {
    pub fn new(cards: Vec<&'a str>, bet: u32) -> Hand {
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

    pub fn is_full_house(&self) -> bool {
        self.frequency.len() == 2 && self.frequency.values().any(|frequency| *frequency == 3)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::hand::Hand;

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
}
