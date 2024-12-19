pub fn process(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = process(input);
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("))(((((", 3)]
    #[case(")())())", -3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    fn test_process(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(expected, process(input))
    }
}
