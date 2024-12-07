use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use nom_supreme::error::ErrorTree;

fn parse_mul(input: &str) -> IResult<&str, (u32, u32), ErrorTree<&str>> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, pair))
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>, ErrorTree<&str>> {
    let (input, v) = many1(many_till(anychar, parse_mul).map(|(_, p)| p))(input)?;

    Ok((input, v))
}

fn process(input: &str) -> u32 {
    let (_input, digit_pairs) = parse(input).expect("should parse");

    digit_pairs.into_iter().map(|(a, b)| a * b).sum()
}

fn main() {
    let input = include_str!("./input.txt");

    let sum: u32 = input.lines().into_iter().map(process).sum();

    dbg!(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let (_, v) = parse(input).unwrap();
        assert_eq!(v, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(process(input), 161);
    }
}
