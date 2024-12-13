use nom::IResult;

#[derive(Debug)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    char: char,
}

#[derive(Debug)]
struct Password<'a> {
    policy: PasswordPolicy,
    password: &'a str,
}

fn parse_line(input: &str) -> IResult<&str, Password> {
    // let (input, _) = separated_pair(tag("abc"), tag(" "), tag("efg"))
}

fn process(input: &str) -> i32 {
    0
}

fn main() {
    let input = include_str!("./input.txt");
    let res = process(input);
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(2, process(input));
    }
}
