use indoc::indoc;

fn main() {
    todo!()
}

fn process(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::process;
    use indoc::indoc;

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
