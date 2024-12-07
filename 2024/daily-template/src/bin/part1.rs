fn process(input: &str) {
    todo!("not implemented");
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
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input));
    }
}
