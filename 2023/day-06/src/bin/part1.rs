use indoc::indoc;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::{collections::HashSet, u32};

#[derive(Debug, PartialEq)]
struct Race {
    duration: u32,
    distance: u32,
}

fn main() {
    let mut races = indoc! {"
        Time:        60     80     86     76
        Distance:   601   1163   1559   1300
    "}
    .lines();
}

fn parse_digit(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_time(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = preceded(
        tuple((tag("Time:"), space0)),
        separated_list1(space1, parse_digit),
    )(input)?;

    Ok((input, digits))
}

fn parse_distance(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = preceded(
        tuple((tag("Distance:"), space0)),
        separated_list1(space1, parse_digit),
    )(input)?;

    Ok((input, digits))
}

// TODO:  Move this as a Impl on Race
fn possible_races(duration: u32) -> HashSet<u32> {
    (0..duration + 1)
        .map(|hold_time| {
            let speed = hold_time;

            let remaining_duration = duration - speed;

            remaining_duration * speed
        })
        .collect::<HashSet<u32>>()
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut input = input.lines();
    let line1 = input.next().expect("should be line 1");
    let line2 = input.next().expect("should be line 2");

    let (_, durations) = parse_time(line1).expect("failed to parse time");

    let (_, distances) = parse_distance(line2).expect("failed to parse distances");

    durations
        .iter()
        .zip(distances.iter())
        .map(|(&duration, &distance)| Race { duration, distance })
        .collect()
}

fn process(input: &str) -> u32 {
    // let duration = 7;

    // dbg!(possible_races(duration));
    0
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{parse_distance, parse_input, parse_time, possible_races, process, Race};
    use indoc::indoc;

    #[test]
    fn parse_input_works() {
        let input = indoc! {"
          Time:      7 10 
          Distance:  9 20
        "};

        let races = parse_input(input);

        let expected_races = vec![
            Race {
                duration: 7,
                distance: 9,
            },
            Race {
                duration: 10,
                distance: 20,
            },
        ];

        assert_eq!(races, expected_races);
    }

    #[test]
    fn parse_times_works() {
        let input = "Time: 7 15 30";

        assert_eq!(parse_time(input), Ok(("", vec![7, 15, 30])));
    }

    #[test]
    fn parse_distance_works() {
        let input = "Distance: 17 115 30";

        assert_eq!(parse_distance(input), Ok(("", vec![17, 115, 30])));
    }

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
