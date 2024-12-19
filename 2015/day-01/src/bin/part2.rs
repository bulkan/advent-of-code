pub fn process(input: &str) -> i32 {
    let mut pos: i32 = 0;
    let mut sum: i32 = 0;

    for (i, c) in input.chars().enumerate() {
        pos = i as i32;

        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => panic!("wat"),
        }

        if sum == -1 {
            break;
        }
    }

    pos + 1
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
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_process(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(expected, process(input))
    }
}
