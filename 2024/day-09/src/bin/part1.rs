use std::iter;

use itertools::Itertools;

fn process(input: &str) -> i64 {
    let mut disk: Vec<Option<i64>> = vec![];
    let mut file_id: i64 = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let x = c.to_digit(10).unwrap();

        match i % 2 {
            0 => {
                disk.append(&mut iter::repeat_n(Some(file_id), x as usize).collect_vec());
                file_id += 1;
            }
            _ => disk.append(&mut iter::repeat_n(None, x as usize).collect_vec()),
        }
    }

    while disk.contains(&None) {
        let last = disk.pop().unwrap();
        if last.is_none() {
            continue;
        }

        if disk.len() <= 1 {
            break;
        }

        if let Some(first_none) = disk.iter_mut().find(|x| x.is_none()) {
            *first_none = last;
        }
    }

    disk.iter()
        .enumerate()
        .map(|(i, x)| i as i64 * x.unwrap())
        .sum()
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
        let input = "2333133121414131402";
        assert_eq!(60, process(input));
    }
}
