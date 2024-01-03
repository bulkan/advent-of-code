use indoc::indoc;
use std::{
    collections::{BTreeMap, HashSet},
    u32,
};

fn main() {
    let races = indoc! {"
        Time:        60     80     86     76
        Distance:   601   1163   1559   1300
    "};
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse_races(input: &str) -> Vec<Race> {
    todo!()
}

fn possible_races(duration: u32) -> HashSet<u32> {
    (0..duration + 1)
        .map(|hold_time| {
            let speed = hold_time;

            let remaining_duration = duration - speed;

            remaining_duration * speed
        })
        .collect::<HashSet<u32>>()
}

fn process(input: &str) -> u32 {
    let duration = 7;

    dbg!(possible_races(duration));
    0
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{possible_races, process};
    use indoc::indoc;

    #[test]
    fn possible_races_returns_correct_values() {
        let races = possible_races(7);

        assert_eq!(races, HashSet::from([6, 10, 12, 0]));
    }

    #[test]
    fn it_works_as_expected() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        let margin_of_error = process(input);

        assert_eq!(margin_of_error, 288);
    }
}
