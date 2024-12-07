use std::collections::HashSet;

use itertools::Itertools;

fn process(input: &str) -> usize {
    let directions: Vec<(i32, i32)> = vec![
        (0, -1), // Up
        (1, 0),  // Right
        (0, 1),  // Down
        (-1, 0), // Left
    ];

    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut guard_pos: (i32, i32) = (6, 4);

    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, char)| {
            if *char == '^' {
                guard_pos = (x as i32, y as i32);
            }
        });
    });

    let grid_width = grid.first().expect("should exist").len() as i32;
    let grid_height = grid.len() as i32;

    let mut current_direction = 0;
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(format!("{},{}", guard_pos.0, guard_pos.1));

    loop {
        let next_direction = directions
            .get(current_direction)
            .expect("direction should exist");

        let next_x = guard_pos.0 + next_direction.0;
        let next_y = guard_pos.1 + next_direction.1;

        let next_pos_char = grid
            .get(next_y as usize)
            .and_then(|row| row.get(next_x as usize));

        // let next_pos = (next_x, next_y);
        // dbg!(guard_pos, next_pos, next_pos_char);

        if next_x < 0 || next_x >= grid_width || next_y < 0 || next_y >= grid_height {
            dbg!("breaking", (next_x, next_y));
            break;
        }

        if let Some('#') = next_pos_char {
            current_direction = (current_direction + 1) % 4;
        } else {
            guard_pos = (next_x, next_y);
            visited.insert(format!("{},{}", guard_pos.0, guard_pos.1));
        }
    }

    visited.len()
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
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(41, process(input));
    }
}
