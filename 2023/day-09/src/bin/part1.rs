fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    todo!()
}

fn generate_difference(input: Vec<i32>) -> Vec<Vec<i32>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{generate_difference, process};

    #[test]
    fn generate_difference_calcuation_works() {
        let input = vec![10, 13, 16, 21, 30, 45];

        let expected = vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3, 3, 5, 9, 15, 23],
            vec![0, 2, 4, 6, 8],
            vec![2, 2, 2, 2],
            vec![0, 0, 0],
        ];

        assert_eq!(generate_difference(input), expected);
    }

    #[test]
    fn example_should_work() {
        let input = indoc! {"
            3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45"};

        assert_eq!(process(input), 114);
    }
}
