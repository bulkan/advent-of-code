use itertools::Itertools;

fn process(input: &str) -> u32 {
    let directions: Vec<(i32, i32)> = vec![
        (0, -1), // Up
        (1, 0),  // Right
        (0, 1),  // Down
        (-1, 0), // Left
    ];

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("should be able to parse"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let zeros = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(
                    move |(x, height)| {
                        if *height == 0 {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    dbg!(zeros);
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
        let input = "0123
1234
8765
9876
";
        assert_eq!(42, process(input));
    }
}
