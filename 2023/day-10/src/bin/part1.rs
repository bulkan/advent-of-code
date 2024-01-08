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
        some data "};

        assert_eq!(process(input), 114);
    }
}
