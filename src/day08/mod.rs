use crate::util::HashMap;
use num::integer::lcm;
use std::iter::repeat;

#[derive(Debug)]
pub enum Dir {
    L,
    R,
}

pub struct Map<'a> {
    instructions: Vec<Dir>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

pub fn parse(input: &str) -> Map<'_> {
    let mut input = input.split("\n\n");
    let instructions = input.next().unwrap();
    let nodes = input.next().unwrap();

    let instructions = instructions
        .chars()
        .map(|c| match c {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!("invalid direction: {c:?}"),
        })
        .collect();

    let nodes = nodes
        .lines()
        .map(|node| {
            let (node, neighbors) = node.split_once(" = (").unwrap();
            let (left, right) = neighbors.trim_end_matches(')').split_once(", ").unwrap();

            (node, (left, right))
        })
        .collect();

    Map {
        instructions,
        nodes,
    }
}

pub fn part1(input: &str) -> usize {
    const GOAL: &str = "ZZZ";
    let Map {
        instructions,
        nodes,
    } = parse(input);

    let mut node = "AAA";
    repeat(instructions.iter())
        .flatten()
        .enumerate()
        .find_map(|(i, d)| {
            let (l, r) = nodes[node];
            node = match d {
                Dir::L => l,
                Dir::R => r,
            };
            dbg!(d, node);

            (node == GOAL).then_some(i + 1)
        })
        .expect("this iterator goes on forever")
}

pub fn part2(input: &str) -> usize {
    let Map {
        instructions,
        nodes,
    } = parse(input);

    let starting_nodes: Vec<_> = nodes
        .iter()
        .map(|(node, _)| *node)
        .filter(|node| &node[2..] == "A")
        .collect();

    starting_nodes
        .iter()
        .map(|&starting| {
            let mut node = starting;
            let steps_to_z = repeat(instructions.iter())
                .flatten()
                .enumerate()
                .find_map(|(i, d)| {
                    let (l, r) = nodes[node];
                    node = match d {
                        Dir::L => l,
                        Dir::R => r,
                    };

                    (&node[2..] == "Z").then_some(i + 1)
                })
                .expect("this iterator goes on forever");

            steps_to_z
        })
        .fold(1, lcm)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 6);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1337);
    }
}
