use std::fs;

fn main() {
    let content = fs::read_to_string("calibrationValues.txt").expect("file no found");

    let sum = content
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|c| c.to_digit(10));

            let first = it.next().expect("should be a number");

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a number")
        })
        .sum::<u32>();

    println!("{sum}");
}
