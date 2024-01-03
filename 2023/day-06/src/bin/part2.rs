#[derive(Debug, PartialEq)]
struct Race {
    duration: u64,
    distance: u64,
}

impl Race {
    fn possible_races(&self) -> Vec<u64> {
        (0..self.duration + 1)
            .filter_map(|hold_time| {
                let speed = hold_time;

                let remaining_duration = self.duration - speed;

                let distance = remaining_duration * speed;

                if distance > self.distance {
                    Some(distance)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    let race = Race {
        duration: 60808676,
        distance: 601116315591300,
    };

    let margin_of_error = race.possible_races().len();

    println!("{margin_of_error}");
}

#[cfg(test)]
mod tests {
    use crate::Race;

    #[test]
    fn possible_races_returns_correct_values() {
        let race = Race {
            duration: 7,
            distance: 9,
        };

        assert_eq!(race.possible_races(), vec![10, 12, 12, 10]);
    }
}
