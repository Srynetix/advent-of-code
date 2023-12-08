//! Common

use std::collections::HashMap;

use aoc_sx::{itertools::Itertools, once_cell::sync::Lazy, regex::Regex};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_input(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("unknown direction {c}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapName(String);

impl MapName {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct MapNode {
    name: MapName,
    left: MapName,
    right: MapName,
}

impl MapNode {
    pub fn from_input(input: &str) -> Self {
        static RGX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap());

        let capture = RGX.captures(input).unwrap();
        Self {
            name: capture
                .get(1)
                .map(|v| MapName(v.as_str().to_owned()))
                .unwrap(),
            left: capture
                .get(2)
                .map(|v| MapName(v.as_str().to_owned()))
                .unwrap(),
            right: capture
                .get(3)
                .map(|v| MapName(v.as_str().to_owned()))
                .unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct CamelMap {
    instructions: Vec<Direction>,
    nodes: HashMap<MapName, MapNode>,
}

impl CamelMap {
    pub fn from_input(input: &str) -> Self {
        let mut lines = input.trim().lines();
        let instructions = lines
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(Direction::from_input)
            .collect_vec();
        lines.next().unwrap();

        let nodes = lines
            .map(MapNode::from_input)
            .map(|n| (n.name.clone(), n))
            .collect();

        Self {
            instructions,
            nodes,
        }
    }

    pub fn get_map_at_direction(&self, name: &MapName, direction: Direction) -> MapName {
        let node = self.nodes.get(name).unwrap();
        match direction {
            Direction::Left => node.left.clone(),
            Direction::Right => node.right.clone(),
        }
    }
}

#[derive(Default)]
pub struct MapWalker;

impl MapWalker {
    pub fn new() -> Self {
        Self
    }

    pub fn steps_to_zzz(&self, map: &CamelMap) -> usize {
        let mut steps = 0;
        let mut current_map = MapName("AAA".into());

        for instruction in map.instructions.iter().cycle() {
            current_map = map.get_map_at_direction(&current_map, *instruction);
            steps += 1;

            if current_map.0 == "ZZZ" {
                return steps;
            }
        }

        panic!("Oops, I'm lost")
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::{CamelMap, MapWalker};

    const SAMPLE1: &str = indoc! {r#"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "#};

    const SAMPLE2: &str = indoc! {r#"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "#};

    #[test]
    fn steps_to_zzz_1() {
        let map = CamelMap::from_input(SAMPLE1);
        let walker = MapWalker::new();
        assert_eq!(walker.steps_to_zzz(&map), 2);
    }

    #[test]
    fn steps_to_zzz_2() {
        let map = CamelMap::from_input(SAMPLE2);
        let walker = MapWalker::new();
        assert_eq!(walker.steps_to_zzz(&map), 6);
    }
}
