use std::{char, collections::HashMap};

use nom::{
    bytes::complete::take_while,
    character::{
        complete::{alpha1, line_ending},
        is_newline,
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;

fn main() {
    todo!()
}

fn process(input: &str) -> u32 {
    0
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<char>, ErrorTree<&str>> {
    let (input, directions) = take_while(|c| !is_newline(c as u8))(input)?;

    let directions = directions.chars().collect::<Vec<_>>();

    Ok((input, directions))
}

#[derive(Debug, PartialEq)]
struct WastelandMap<'a> {
    // instructions: Vec<&char>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
    first_node: &'a str,
}

impl<'a> WastelandMap<'a> {
    fn new(nodes: Vec<(&'a str, (&'a str, &'a str))>) -> WastelandMap {
        let first_node = nodes[0].0;

        let nodes: HashMap<&'a str, (&'a str, &'a str)> = nodes.into_iter().collect();

        WastelandMap { nodes, first_node }
    }
}

// (BBB,  BBB)
fn parse_node_values(input: &str) -> IResult<&str, (&str, &str), ErrorTree<&str>> {
    let (input, _) = tag("(")(input)?;

    let (input, values) = separated_pair(alpha1, tag(", "), alpha1)(input)?;

    let (input, _) = tag(")")(input)?;

    Ok((input, values))
}

// AAA = (BBB, BBB)
fn parse_nodes(input: &str) -> IResult<&str, WastelandMap, ErrorTree<&str>> {
    let (input, nodes_vec) = separated_list1(
        line_ending,
        separated_pair(alpha1, tag(" = "), parse_node_values),
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
            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)"};

        let expected = WastelandMap {
            first_node: "AAA",
            nodes: HashMap::from([("AAA", ("BBB", "BBB")), ("BBB", ("AAA", "ZZZ"))]),
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
            ("\n\nAAA = (BBB, BBB)\n", vec!['L', 'L', 'R']),
        );
    }

    #[test]
    fn it_works_as_expected() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};

        let res = process(input);

        assert_eq!(res, 6);
    }
}
