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

const WIDTH: i32 = 11;
const HEIGHT: i32 = 7;

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
    let middle = IVec2::from((WIDTH / 2, HEIGHT / 2));

    robots
        .iter()
        .filter(|robot| robot.position != middle)
        .count()
}

fn simulate_robots_movements(input: &str, seconds: usize) -> Vec<Robot> {
    let (_, mut robots) = parse(input).unwrap();

    for _ in 0..seconds {
        robots = robots
            .into_iter()
            .map(|mut robot| {
                let mut next_pos = robot.position + robot.velocity;

                if next_pos.x < 0 {
                    next_pos.x += WIDTH;
                }

                if next_pos.x > WIDTH {
                    next_pos.x -= WIDTH;
                }

                if next_pos.y > HEIGHT {
                    next_pos.y -= HEIGHT;
                }

                if next_pos.y < 0 {
                    next_pos.y += HEIGHT;
                }

                robot.position = next_pos;

                robot
            })
            .collect::<Vec<_>>();
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
    use super::*;
    use rstest::rstest;

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

    //     #[test]
    //     fn test_process() {
    //         let input = "p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3";
    //         assert_eq!(0, process(input, 11, 7, 100));
    //     }
}
