//! Common

use std::{borrow::Cow, collections::HashMap};

use aoc_sx::{itertools::Itertools, num::Integer, once_cell::sync::Lazy, regex::Regex};

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
pub struct MapName(Cow<'static, str>);

impl MapName {
    pub fn new(value: String) -> Self {
        Self(value.into())
    }

    pub const fn new_const(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }

    pub fn is_starting_map(&self) -> bool {
        self.0.ends_with('A')
    }

    pub fn is_ending_map(&self) -> bool {
        self.0.ends_with('Z')
    }

    pub const START_LOCATION: Self = MapName::new_const("AAA");
    pub const END_LOCATION: Self = MapName::new_const("ZZZ");
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
            Lazy::new(|| Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap());

        let capture = RGX.captures(input).unwrap();
        Self {
            name: capture
                .get(1)
                .map(|v| MapName::new(v.as_str().into()))
                .unwrap(),
            left: capture
                .get(2)
                .map(|v| MapName::new(v.as_str().into()))
                .unwrap(),
            right: capture
                .get(3)
                .map(|v| MapName::new(v.as_str().into()))
                .unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct GhostMemory {
    lines: Vec<GhostMemoryLine>,
}

impl GhostMemory {
    pub fn new(size: usize) -> Self {
        Self {
            lines: (0..size).map(|_| GhostMemoryLine::new()).collect(),
        }
    }

    pub fn compute(&self) -> usize {
        self.lines
            .iter()
            .map(|l| l.diff)
            .reduce(|acc, e| acc.lcm(&e))
            .unwrap()
    }
}

impl GhostMemory {
    pub fn update_line(&mut self, index: usize, steps: usize) -> usize {
        self.lines[index].update(steps)
    }

    pub fn is_ready(&self) -> bool {
        self.lines.iter().all(GhostMemoryLine::is_ready)
    }
}

#[derive(Debug)]
pub struct GhostMemoryLine {
    loops: usize,
    diff: usize,
    last_steps: usize,
}

impl Default for GhostMemoryLine {
    fn default() -> Self {
        Self::new()
    }
}

impl GhostMemoryLine {
    pub fn new() -> Self {
        Self {
            loops: 0,
            diff: 0,
            last_steps: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.loops > 1
    }

    pub fn update(&mut self, steps: usize) -> usize {
        let last_steps = self.last_steps;
        let diff = steps - last_steps;
        self.diff = diff;
        self.last_steps = steps;
        self.loops += 1;
        diff
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

    pub fn iter_starting_maps(&self) -> impl Iterator<Item = MapName> + '_ {
        self.nodes.keys().filter(|&m| m.is_starting_map()).cloned()
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
        let mut current_map = MapName::START_LOCATION;

        for instruction in map.instructions.iter().cycle() {
            current_map = map.get_map_at_direction(&current_map, *instruction);
            steps += 1;

            if current_map == MapName::END_LOCATION {
                return steps;
            }
        }

        panic!("Oops, I'm lost")
    }

    pub fn parallel_steps_to_zzz(&self, map: &CamelMap) -> usize {
        let starting_maps = map.iter_starting_maps().collect_vec();
        let mut steps = 0;
        let mut current_maps = starting_maps.clone();
        let mut memory = GhostMemory::new(current_maps.len());

        for instruction in map.instructions.iter().cycle() {
            let mut arrived = true;

            for (id, map_name) in current_maps.iter_mut().enumerate() {
                *map_name = map.get_map_at_direction(map_name, *instruction);

                if !map_name.is_ending_map() {
                    arrived = false;
                } else {
                    let diff = memory.update_line(id, steps);
                    println!("[Z] ID: {id} / Delta: {diff} / Map: {map_name:?}");
                }
            }

            if memory.is_ready() {
                // Shortcut!
                return memory.compute();
            }

            steps += 1;

            if arrived {
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

    const SAMPLE3: &str = indoc! {r#"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
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

    #[test]
    fn parallel_steps() {
        let map = CamelMap::from_input(SAMPLE3);
        let walker = MapWalker::new();
        assert_eq!(walker.parallel_steps_to_zzz(&map), 6);
    }
}
