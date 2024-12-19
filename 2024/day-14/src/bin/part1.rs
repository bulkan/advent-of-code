use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

const TILES_SIZE: IVec2 = if cfg!(test) {
    IVec2::new(11, 7)
} else {
    IVec2::new(101, 103)
};

fn main() {
    let input = include_str!("./input.txt");
    let res = process(input);
    dbg!(res);
}

fn process(input: &str) -> usize {
    let robots = simulate_robots_movements(input, 100);

    calculate_robots_in_quadrants(&robots)
}

fn calculate_robots_in_quadrants(robots: &[Robot]) -> usize {
    let middle = TILES_SIZE / 2;

    let filtered_bots = robots
        .iter()
        .filter(|robot| {
            if robot.position.x == middle.x || robot.position.y == middle.y {
                return false;
            }

            true
        })
        .collect::<Vec<_>>();

    // dbg!(&filtered_bots);
    // dbg!(&robots.len(), filtered_bots.len());

    let mut q1 = 0;

    // q1
    for x in 0..middle.x {
        for y in 0..middle.y {
            let p = IVec2::from((x, y));
            q1 += filtered_bots
                .iter()
                .filter(|bot| bot.position.x == p.x && bot.position.y == p.y)
                .count();
        }
    }

    let mut q2 = 0;
    for x in middle.x..TILES_SIZE.x {
        for y in 0..=middle.y {
            let p = IVec2::from((x, y));
            q2 += filtered_bots
                .iter()
                .filter(|bot| bot.position.x == p.x && bot.position.y == p.y)
                .count();
        }
    }

    let mut q3 = 0;

    // q3
    for x in 0..=middle.x {
        for y in middle.y..TILES_SIZE.y {
            let p = IVec2::from((x, y));
            q3 += filtered_bots
                .iter()
                .filter(|bot| bot.position.x == p.x && bot.position.y == p.y)
                .count();
        }
    }

    let mut q4 = 0;
    // q4
    for x in middle.x..TILES_SIZE.x {
        for y in middle.y..TILES_SIZE.y {
            let p = IVec2::from((x, y));
            q4 += filtered_bots
                .iter()
                .filter(|bot| bot.position.x == p.x && bot.position.y == p.y)
                .count();
        }
    }

    dbg!(q1, q2, q3, q4);

    q1 * q2 * q3 * q4
}

fn simulate_robots_movements(input: &str, seconds: usize) -> Vec<Robot> {
    let (_, mut robots) = parse(input).unwrap();

    for _ in 0..seconds {
        for robot in robots.iter_mut() {
            robot.position = (robot.position + robot.velocity).rem_euclid(TILES_SIZE);
        }
    }

    robots
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Robot> {
    let (input, (position, velocity)) =
        separated_pair(parse_position, tag(" "), parse_velocity)(input)?;

    Ok((
        input,
        Robot {
            position: IVec2::from(position),
            velocity: IVec2::from(velocity),
        },
    ))
}

fn parse_position(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(
        tag("p="),
        separated_pair(complete::i32, tag(","), complete::i32),
    )(input)
}

fn parse_velocity(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(
        tag("v="),
        separated_pair(complete::i32, tag(","), complete::i32),
    )(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use rstest::rstest;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[rstest]
    #[case("p=6,3", (6,3))]
    #[case("p=10,3", (10,3))]
    fn test_parse_position(#[case] input: &str, #[case] expected: (i32, i32)) {
        let (_input, pos) = parse_position(input).unwrap();
        assert_eq!(expected, pos);
    }

    #[rstest]
    #[case("v=1,-3", (1,-3))]
    #[case("v=-3,-3", (-3,-3))]
    fn test_parse_velocity(#[case] input: &str, #[case] expected: (i32, i32)) {
        let (_input, v) = parse_velocity(input).unwrap();
        assert_eq!(expected, v);
    }

    #[test]
    fn test_parse_parse_line() {
        let (_input, robot) = parse_line("p=10,3 v=-3,-3").unwrap();
        let expected = Robot {
            position: IVec2::from((10, 3)),
            velocity: IVec2::from((-3, -3)),
        };
        assert_eq!(expected, robot);
    }

    #[rstest]
    #[case("p=2,4 v=2,-3", 1, (4,1))]
    #[case("p=2,4 v=2,-3", 2, (6,5))]
    #[case("p=2,4 v=2,-3", 3, (8,2))]
    #[case("p=2,4 v=2,-3", 4, (10,6))]
    #[case("p=2,4 v=2,-3", 5, (1,3))]
    fn test_simulate_with_single_robot_input(
        #[case] input: &str,
        #[case] seconds: usize,
        #[case] expected: (i32, i32),
    ) {
        let expected_pos = IVec2::from(expected);
        let robots = simulate_robots_movements(input, seconds);
        assert_eq!(robots.first().unwrap().position, expected_pos);
    }

    //  width = 11, height = 7
    #[rstest]
    #[case("p=2,0 v=2,-3", 1, (4,4))]
    fn test_simulate_edges(
        #[case] input: &str,
        #[case] seconds: usize,
        #[case] expected: (i32, i32),
    ) {
        let expected_pos = IVec2::from(expected);
        let robots = simulate_robots_movements(input, seconds);
        assert_eq!(robots.first().unwrap().position, expected_pos);
    }

    #[test]
    fn test_simulate() {
        let robots = simulate_robots_movements(INPUT, 100);

        let expected_robots = HashSet::from([
            IVec2::from((6, 0)),
            IVec2::from((9, 0)),
            IVec2::from((0, 2)),
            IVec2::from((1, 3)),
            IVec2::from((2, 3)),
            IVec2::from((5, 4)),
            IVec2::from((3, 5)),
            IVec2::from((4, 5)),
            IVec2::from((1, 6)),
            IVec2::from((6, 6)),
        ]);

        assert!(robots
            .iter()
            .all(|bot| expected_robots.contains(&bot.position)));
    }

    #[test]
    fn test_process() {
        assert_eq!(12, process(INPUT));
    }
}
