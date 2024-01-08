use itertools::{Itertools, Position};

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let mut end_numbers: Vec<i64> = vec![];

            loop {
                if nums.iter().all(|n| n == &0) {
                    break;
                }

                nums = nums
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        match position {
                            Position::Last | Position::Only => {
                                end_numbers.push(*right);
                            }
                            _ => {}
                        };

                        right - left
                    })
                    .collect::<Vec<i64>>();
            }

            end_numbers.iter().sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::process;

    #[test]
    fn example_should_work() {
        let input = indoc! {"
            3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45"};

        assert_eq!(process(input), 114);
    }
}
