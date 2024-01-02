use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{
    collections::{BTreeMap, HashSet},
    u32,
};

fn main() {
    let scratchcards = include_str!("./input.txt");

    let points = process_scratchcards(scratchcards);

    println!("{points}");
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use std::collections::HashSet;

    #[test]
    fn it_works_as_expected() {
        let input = indoc! {"
        };

        let res = process(input);

        assert_eq!(res, 30);
    }
}
