use itertools::Itertools;

pub fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|n| n.parse::<i32>().expect("should be a digit"))
        .tuple_combinations::<(i32, i32)>()
        .filter(|(a, b)| a + b == 2020)
        .fold(0, |acc, (a, b)| acc + a * b)
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
        let input = "1721
979
366
299
675
1456";
        assert_eq!(514579, process(input));
    }
}
