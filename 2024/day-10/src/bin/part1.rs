use std::usize;

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

    let grid_width = grid.first().expect("should exist").len() as i32;
    let grid_height = grid.len() as i32;

    zeros
        .iter()
        .map(|(x, y)| {
            let mut current = 0;

            let next_direction = directions.iter().find(|(dx, dy)| {
                let next_x = *x as i32 + dx;
                let next_y = *y as i32 + dy;

                if next_x < 0 || next_x > grid_width || next_y < 0 || next_y > grid_height {
                    return false;
                }

                let next_val = grid
                    .get(next_y as usize)
                    .and_then(|row| row.get(next_x as usize))
                    .copied();

                if Some(current + 1) == next_val {
                    dbg!(next_val, current);
                }

                false
            });
        })
        .collect_vec();

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
        assert_eq!(36, process(input));
    }
}
