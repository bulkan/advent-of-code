use glam::IVec2;
use std::collections::{HashMap, HashSet};

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn process(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let pos = IVec2::new(x as i32, y as i32);
                    let v = c.to_digit(10).expect("should be able to parse");

                    (pos, v)
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<IVec2, u32>>();

    grid.iter()
        .filter(|(_, height)| **height == 0)
        .map(|(position, _)| {
            let mut visited: HashSet<IVec2> = HashSet::from([]);
            let mut new_locations: HashSet<IVec2> = HashSet::from([*position]);
            while !new_locations.is_empty() {
                let newer_locations = new_locations
                    .iter()
                    .flat_map(|starting_location| {
                        DIRECTIONS
                            .iter()
                            .map(move |dir| (dir + starting_location, starting_location))
                    })
                    .filter(|(new_location, starting_location)| {
                        !new_locations.contains(new_location)
                            && grid.get(new_location).is_some_and(|h| {
                                let current_height = grid.get(starting_location).unwrap();

                                *h == current_height + 1
                            })
                    })
                    .map(|(new, _)| new)
                    .collect::<HashSet<IVec2>>();

                for loc in newer_locations.iter() {
                    visited.insert(*loc);
                }

                new_locations = newer_locations;
            }

            visited
                .iter()
                .filter(|pos| grid.get(pos).unwrap() == &9)
                .count()
        })
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
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(36, process(input));
    }
}
