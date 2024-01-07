use std::{char, collections::HashMap};

use nom::{
    branch::alt,
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
    fn new() -> WastelandMap {}
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
    let (input, raw_string_vec) = separated_list1(
        line_ending,
        separated_pair(alpha1, tag(" = "), parse_node_values),
    )(input)?;

    dbg!(raw_string_vec);

    Ok((input, vec![]))
}

#[cfg(test)]
mod tests {
    use crate::{parse_instructions, parse_node_values, parse_nodes, process};
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
            BBB = (AAA, ZZZ)

        "};

        assert_eq!(parse_nodes(input).unwrap(), ("", vec![]))
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
