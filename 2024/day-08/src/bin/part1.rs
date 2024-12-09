use std::collections::{BTreeMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

fn process(input: &str) -> u32 {
    let mut antennas: BTreeMap<char, Vec<IVec2>> = BTreeMap::new();

    let mut grid_height = 0;
    let mut grid_width = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if grid_height == 0 {
                grid_width += 1;
            }
            if char != '.' {
                let p = IVec2::new(x as i32, y as i32);
                antennas
                    .entry(char)
                    .and_modify(|positions| {
                        positions.push(p);
                    })
                    .or_insert(vec![p]);
            }
        });
        grid_height += 1;
    });

    // dbg!(grid_height, grid_width);
    // dbg!(&antennas);

    let mut results: HashSet<IVec2> = HashSet::new();

    antennas.iter().for_each(|(_antenna, positions)| {
        positions.iter().permutations(2).for_each(|perms| {
            let a = perms.first().expect("should have first");
            let b = perms.get(1).expect("should have second element");

            let (dx, dy) = (b.x - a.x, b.y - a.y);

            let a1 = IVec2::new(a.x - dx, a.y - dy);
            if 0 <= a1.x && a1.x < grid_width && 0 <= a1.y && a1.y < grid_width {
                results.insert(a1);
            }

            let b1 = IVec2::new(b.x + dx, b.y + dy);

            if 0 <= b1.x && b1.x < grid_height && 0 <= b1.y && b1.y < grid_height {
                results.insert(b1);
            }
        });
    });

    results.len() as u32
}

fn main() {
    let input = include_str!("./input.txt");
    let res = process(input);
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_vecs() {
    //     let a = IVec2::new(4, 3);
    //     let b = IVec2::new(5, 5);
    //
    //     // dx, dy = x2 - x1, y2 - y1
    //     let (dx, dy) = (b.x - a.x, b.y - a.y);
    //
    //     let (a2, b2) = (b.x + dx, b.y + dy);
    //     let (a1, b1) = (a.x - dx, a.y - dy);
    //
    //     dbg!(a1, b1);
    //     dbg!(a2, b2);
    //     // find all antennas
    //     //   - hashmap antenna to position
    //     //   - for each antenna get distance to next antenna
    //     //          (a, b)
    //
    //     assert!(false);
    // }

    #[test]
    fn test_process() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(14, process(input));
    }
}
