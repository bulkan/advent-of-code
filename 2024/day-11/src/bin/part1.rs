use std::ops::Mul;

fn process(input: &str, run_count: u32) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|stone| stone.parse::<i64>().expect("should convert"))
        .collect::<Vec<_>>();

    let mut count = 0;

    loop {
        let new_stones = stones
            .iter()
            .flat_map(|stone| {
                if *stone == 0 {
                    vec![1]
                } else if stone.to_string().len() % 2 == 0 {
                    let s = stone.to_string();
                    let mid = s.len() / 2;

                    let (l, r) = s.split_at(mid);

                    vec![l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()]
                } else {
                    vec![stone.mul(2024)]
                }
            })
            .collect::<Vec<_>>();

        if count >= run_count {
            break;
        }

        count += 1;

        stones = new_stones;
    }

    // dbg!(&stones);

    stones.len()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = process(input, 6);
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "125 17";
        assert_eq!(222, process(input, 1));
    }
}
