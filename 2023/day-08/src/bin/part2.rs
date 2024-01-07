use std::{char, collections::HashMap};

use nom::{
    bytes::complete::take_while,
    character::{
        complete::{alphanumeric1, line_ending},
        is_newline,
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    let (input, instructions) =
        parse_instructions(input).expect("parsing of instruction line failed");

    let (_, wasteland_map) = parse_nodes(input).expect("parsing of nodes failed");

    let mut step = 0;
    let mut current_node = "AAA";

    let mut instuctions_iter = instructions.iter().cycle();

    loop {
        let direction = instuctions_iter.next();

        if step % 1000000 == 0 {
            println!("{current_node} {direction:?} {step}");
        }

        if current_node == "ZZZ" {
            break;
        }

        match direction {
            Some('L') => current_node = wasteland_map.nodes[current_node].0,
            Some('R') => current_node = wasteland_map.nodes[current_node].1,
            _ => panic!("this shouldnt happen"),
        }

        step += 1;
    }

    step
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<char>, ErrorTree<&str>> {
    let (input, directions) = take_while(|c| !is_newline(c as u8))(input)?;

    let directions = directions.chars().collect::<Vec<_>>();

    // skip the blank line
    let (input, _) = take_while(|c| is_newline(c as u8))(input)?;

    Ok((input, directions))
}

#[derive(Debug, PartialEq)]
struct WastelandMap<'a> {
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
    starting_nodes: Vec<&'a str>,
}

impl<'a> WastelandMap<'a> {
    fn new(nodes: Vec<(&'a str, (&'a str, &'a str))>) -> WastelandMap {
        let starting_nodes = nodes
            .iter()
            .filter_map(|(node, (_, _))| {
                if node.ends_with("A") {
                    Some(*node)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let nodes: HashMap<&'a str, (&'a str, &'a str)> = nodes.into_iter().collect();

        WastelandMap {
            nodes,
            starting_nodes,
        }
    }
}

// (BBB,  BBB)
fn parse_node_values(input: &str) -> IResult<&str, (&str, &str), ErrorTree<&str>> {
    let (input, _) = tag("(")(input)?;

    let (input, values) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;

    let (input, _) = tag(")")(input)?;

    Ok((input, values))
}

// AAA = (BBB, BBB)
fn parse_nodes(input: &str) -> IResult<&str, WastelandMap, ErrorTree<&str>> {
    let (input, nodes_vec) = separated_list1(
        line_ending,
        separated_pair(alphanumeric1, tag(" = "), parse_node_values),
    )(input)?;

    Ok((input, WastelandMap::new(nodes_vec)))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_instructions, parse_node_values, parse_nodes, process, WastelandMap};
    use indoc::indoc;

    #[test]
    fn parse_node_values_returns_correct_values() {
        assert_eq!(
            parse_node_values("(BBB, BBB)").unwrap(),
            ("", ("BBB", "BBB"))
        );
    }

    #[test]
    fn parse_nodes_returns_correct_values() {
        let input = indoc! {"
            11A = (11B, XXX)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            XXX = (XXX, XXX)"};

        let expected = WastelandMap {
            starting_nodes: vec!["11A", "22A"],
            nodes: HashMap::from([
                ("11A", ("11B", "XXX")),
                ("11Z", ("11B", "XXX")),
                ("22A", ("22B", "XXX")),
                ("XXX", ("XXX", "XXX")),
            ]),
        };

        assert_eq!(parse_nodes(input).unwrap(), ("", expected))
    }

    #[test]
    fn parse_instructions_returns_correct_values() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
        "};

        assert_eq!(
            parse_instructions(input).unwrap(),
            ("AAA = (BBB, BBB)\n", vec!['L', 'L', 'R']),
        );
    }

    #[test]
    fn it_works_with_sample_data_2() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};

        let res = process(input);

        assert_eq!(res, 6);
    }

    #[test]
    fn it_works_with_sample_data_1() {
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)"};

        let res = process(input);

        assert_eq!(res, 6);
    }
}
