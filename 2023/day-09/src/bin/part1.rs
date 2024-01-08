fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    todo!()
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
