use std::ops::Sub;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let safe = process_reports(input);

    dbg!(safe);
}

fn is_report_safe(report: &[i32]) -> bool {
    let is_increasing = report.first().unwrap() < report.get(1).unwrap();

    report.iter().tuple_windows().all(|(a, b)| {
        let d = a.sub(b);

        let f = match d {
            -3..=-1 => true,
            1..=3 => true,
            _ => false,
        };

        if is_increasing {
            return f && a < b;
        }

        f && a > b
    })
}

fn process_reports(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i32>().expect("should be a number"))
                .collect::<Vec<i32>>()
        })
        .filter(|report| is_report_safe(report))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{is_report_safe, process_reports};

    #[test]
    fn is_report_valid_test() {
        assert!(is_report_safe(&[7, 6, 4, 2, 1]));
        assert!(!is_report_safe(&[1, 2, 7, 8, 9]));
        assert!(!is_report_safe(&[9, 7, 6, 2, 1]));
        assert!(!is_report_safe(&[1, 3, 2, 4, 5]));
        assert!(!is_report_safe(&[8, 6, 4, 4, 1]));
        assert!(is_report_safe(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn it_works_as_expected() {
        let input = "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9";

        let valid_reports = process_reports(input);

        assert_eq!(valid_reports, 2);
    }
}
