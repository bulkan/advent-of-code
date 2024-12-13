use std::collections::HashMap;

use glam::IVec2;
use petgraph::{algo::condensation, data::Build, prelude::*, visit::IntoNodeReferences};

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<IVec2, char>>();

    let mut graph: UnGraphMap<(i32, i32), ()> = UnGraphMap::new();

    for (current_node, c) in map.iter() {
        let node = graph.add_node((current_node.x, current_node.y));

        for d in DIRECTIONS.iter() {
            let new_node = current_node + d;
            if map.get(&new_node).is_some_and(|c2| c == c2) {
                graph.add_edge(node, (new_node.x, new_node.y), ());
            };
        }
    }

    let new_graph = condensation(graph.clone().into_graph::<NodeIndex>(), false);

    new_graph
        .node_references()
        .map(|(_, nodes)| {
            let area = nodes.len();
            let perimeter = nodes
                .iter()
                .map(|n| 4 - graph.neighbors(*n).count())
                .sum::<usize>();

            area * perimeter
        })
        .sum::<usize>()
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
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(1930, process(input));
    }
}
