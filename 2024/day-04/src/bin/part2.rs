use std::collections::HashMap;

#[derive(Debug)]
struct Grid {
    data: HashMap<(usize, usize), char>,
    col_count: i32,
    row_count: i32,
}

fn load_grid(input: &str) -> Grid {
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();

    let mut row_count: i32 = 0;
    let mut col_count: i32 = 0;

    for (row, line) in input.lines().enumerate() {
        row_count += 1;
        for (col, c) in line.chars().enumerate() {
            col_count += if row == 0 { 1 } else { 0 };
            grid.insert((row, col), c);
        }
    }

    Grid {
        data: grid,
        row_count,
        col_count,
    }
}

fn count_pattern(grid: &Grid, pattern: &[(char, i32, i32)]) -> i32 {
    let mut count = 0;

    for row in 0..grid.row_count {
        for col in 0..grid.col_count {
            for (index, (letter, row_offset, col_offset)) in pattern.iter().enumerate() {
                let new_row = row + row_offset;
                let new_col = col + col_offset;

                let new_c = grid.data.get(&(new_row as usize, new_col as usize));

                if new_row < 0
                    || new_row >= grid.row_count
                    || new_col < 0
                    || new_col >= grid.col_count
                    || new_c != Some(letter)
                {
                    break;
                }

                if index == pattern.len() - 1 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn process(input: &str) -> i32 {
    let patterns: Vec<Vec<(char, i32, i32)>> = vec![
        vec![
            ('A', 0, 0),
            ('M', -1, -1),
            ('S', 1, 1),
            ('M', 1, -1),
            ('S', -1, 1),
        ],
        vec![
            ('A', 0, 0),
            ('S', -1, -1),
            ('M', 1, 1),
            ('M', 1, -1),
            ('S', -1, 1),
        ],
        vec![
            ('A', 0, 0),
            ('M', -1, -1),
            ('S', 1, 1),
            ('S', 1, -1),
            ('M', -1, 1),
        ],
        vec![
            ('A', 0, 0),
            ('S', -1, -1),
            ('M', 1, 1),
            ('S', 1, -1),
            ('M', -1, 2),
        ],
    ];

    let grid = load_grid(input);

    patterns
        .iter()
        .map(|pattern| count_pattern(&grid, pattern))
        .sum::<i32>()
}

fn main() {
    let input = include_str!("./input.txt");

    let sum: i32 = process(input);

    dbg!(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(9, process(input));
    }
}
