use std::{char, os::unix::fs::DirEntryExt};

use indoc::indoc;
use nom::{
    bytes::complete::take_while,
    character::{complete::newline, is_newline},
    IResult,
};
use nom_supreme::error::ErrorTree;

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

#[cfg(test)]
mod tests {
    use crate::{parse_instructions, process};
    use indoc::indoc;

    #[test]
    fn parse_instructions_works() {
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
