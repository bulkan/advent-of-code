use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let numbers_map = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let content = fs::read_to_string("calibrationValues.txt").expect("file no found");

    let re = Regex::new(r"\d").expect("failed to create regex");

    let sum = content.split('\n').fold(0, |acc, line| {
        let digit_matches = re
            .captures_iter(line)
            .map(|mat| mat.get(0).unwrap().as_str())
            .collect::<Vec<_>>();

        if digit_matches.is_empty() {
            return acc;
        }

        let digit = format!(
            "{}{}",
            digit_matches[0],
            digit_matches[digit_matches.len() - 1]
        );

        if let Ok(result) = digit.parse::<u32>() {
            return acc + result;
        }

        acc
    });

    println!("{sum}");
}
