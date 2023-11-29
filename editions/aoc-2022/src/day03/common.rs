//! Common

use std::{collections::HashSet, convert::Infallible, str::FromStr};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Item(char);

impl Item {
    pub fn new(c: char) -> Self {
        Self(c)
    }

    pub fn priority(&self) -> u32 {
        match self.0 {
            c @ 'a'..='z' => (c as u32 - 'a' as u32) + 1,
            c @ 'A'..='Z' => (c as u32 - 'A' as u32) + 27,
            c => panic!("unsupported char {c}"),
        }
    }
}

#[derive(Debug)]
pub struct Compartment {
    items: HashSet<Item>,
}

impl Compartment {
    pub fn intersection(&self, other: &Self) -> HashSet<Item> {
        self.items.intersection(&other.items).copied().collect()
    }

    pub fn items(&self) -> &HashSet<Item> {
        &self.items
    }
}

impl FromStr for Compartment {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            items: text.chars().map(Item::new).collect(),
        })
    }
}

#[derive(Debug)]
pub struct Rucksack {
    compartments: [Compartment; 2],
}

impl Rucksack {
    pub fn only_common_item(&self) -> Item {
        let [compartment1, compartment2] = &self.compartments;
        let common_items = compartment1.intersection(compartment2);

        assert!(
            common_items.len() == 1,
            "there should be only one common item in a rucksack"
        );
        *common_items.iter().next().unwrap()
    }

    pub fn total_items(&self) -> HashSet<Item> {
        self.compartments
            .iter()
            .flat_map(|x| x.items())
            .copied()
            .collect::<HashSet<_>>()
    }
}

impl FromStr for Rucksack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_at(s.len() / 2);
        Ok(Self {
            compartments: [
                Compartment::from_str(p1).unwrap(),
                Compartment::from_str(p2).unwrap(),
            ],
        })
    }
}

#[derive(Debug)]
pub struct RucksackParser {
    rucksacks: Vec<Rucksack>,
}

impl RucksackParser {
    pub fn priority_sum(&self) -> u32 {
        self.rucksacks
            .iter()
            .map(|r| r.only_common_item().priority())
            .sum()
    }

    fn group_only_common_item(&self, group: &[Rucksack]) -> Item {
        let [g1, g2, g3] = group else {
            panic!("chunk should be of size 3");
        };

        let items = g1
            .total_items()
            .intersection(&g2.total_items())
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&g3.total_items())
            .copied()
            .collect::<HashSet<_>>();
        assert!(
            items.len() == 1,
            "there should be only one common item in a group"
        );

        *items.iter().next().unwrap()
    }

    pub fn group_priority_sum(&self) -> u32 {
        self.rucksacks
            .chunks_exact(3)
            .map(|group| self.group_only_common_item(group).priority())
            .sum()
    }
}

impl FromStr for RucksackParser {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rucksacks = s.lines().map(|s| Rucksack::from_str(s).unwrap()).collect();
        Ok(Self { rucksacks })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use aoc_sx::indoc::indoc;

    use super::RucksackParser;

    const SAMPLE: &str = indoc! {r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "#};

    #[test]
    fn test_sample() {
        assert_eq!(
            RucksackParser::from_str(SAMPLE).unwrap().priority_sum(),
            157
        );
        assert_eq!(
            RucksackParser::from_str(SAMPLE)
                .unwrap()
                .group_priority_sum(),
            70
        );
    }
}
