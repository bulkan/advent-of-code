use std::collections::{BTreeMap, HashSet};

use glam::{u32, IVec2};
use itertools::Itertools;

fn process(input: &str) -> u32 {
    let mut antennas: BTreeMap<char, Vec<IVec2>> = BTreeMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
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
    });

    let grid_height = input.lines().count();
    let grid_width = input.lines().next().unwrap().len();
    let y_bound = 0i32..grid_height as i32;
    let x_bound = 0i32..grid_width as i32;

    // dbg!(grid_height, grid_width);
    // dbg!(&antennas);

    let mut results: HashSet<IVec2> = HashSet::new();

    antennas.iter().for_each(|(_antenna, positions)| {
        positions.iter().combinations(2).for_each(|perms| {
            let a = perms.first().expect("should have first");
            let b = perms.get(1).expect("should have second element");

            let d = IVec2::new(b.x - a.x, b.y - a.y);

            let mut b1 = **b + d;
            while x_bound.contains(&b1.x) && y_bound.contains(&b1.y) {
                results.insert(b1);
                b1 += d;
            }

            let mut a1 = **a - d;
            while x_bound.contains(&a1.x) && y_bound.contains(&a1.y) {
                results.insert(a1);
                a1 -= d;
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

        assert_eq!(34, process(input));
    }
}
