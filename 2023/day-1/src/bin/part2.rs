use std::fs;

fn main() {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let content = fs::read_to_string("calibrationValues.txt").expect("file no found");

    let mut sum = 0;

    for line in content.lines() {
        let mut digits: Vec<String> = Vec::new();

        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                digits.push(c.to_string());
            }

            for (d, number) in numbers.iter().enumerate() {
                let reduced_line = &line[i..];
                if reduced_line.starts_with(number) {
                    digits.push((d + 1).to_string())
                }
            }
        }

        if !digits.is_empty() {
            let digit = format!("{}{}", digits[0], digits[digits.len() - 1])
                .parse::<u32>()
                .unwrap_or(0);

            sum += digit;
        }
    }

    println!("\n{sum}");
}
