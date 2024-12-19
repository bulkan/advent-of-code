use glam::IVec2;
use itertools::Itertools;
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

fn process(input: &str) -> i32 {
    simulate_robots_movements(input)
}

fn simulate_robots_movements(input: &str) -> i32 {
    let (_, mut robots) = parse(input).unwrap();

    let mut i = 0;

    loop {
        for robot in robots.iter_mut() {
            robot.position = (robot.position + robot.velocity).rem_euclid(TILES_SIZE);
        }
        i += 1;
        if tree_test(&robots) {
            break i;
        }
    }
}

fn tree_test(robots: &[Robot]) -> bool {
    robots
        .iter()
        .map(|Robot { position, .. }| position)
        .all_unique()
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
