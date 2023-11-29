//! Common

use std::collections::HashMap;

use aoc_sx::{once_cell::sync::Lazy, regex::Regex};

pub const INPUT_COLOR_NAME: &str = "shiny gold";

static MAIN_RULE_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<color>\w+ \w+) bags contain (?P<rules>.*)\.$").unwrap());

static SIMPLE_RULE_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<amount>\d+) (?P<color>\w+ \w+) bags?").unwrap());

const NO_OTHER_BAGS_STR: &str = "no other bags";

/// Bag color
#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BagColor(String);

impl BagColor {
    /// Create a bag color from a str.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn new(input: &str) -> Self {
        Self(input.to_owned())
    }
}

impl From<&str> for BagColor {
    fn from(input: &str) -> Self {
        Self(input.to_owned())
    }
}

/// Bag relation
#[derive(Debug, Clone)]
pub struct BagRelation {
    color: BagColor,
    amount: usize,
}

impl BagRelation {
    /// Create a bag relation.
    ///
    /// # Arguments
    ///
    /// * `color` - Bag color
    /// * `amount` - Bag amount
    pub const fn new(color: BagColor, amount: usize) -> Self {
        Self { color, amount }
    }
}

/// Bag system
#[derive(Debug)]
pub struct BagSystem(HashMap<BagColor, Vec<BagRelation>>);

impl BagSystem {
    /// Parse rule.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_rule(&mut self, input: &str) {
        let capture = MAIN_RULE_RGX.captures(input.trim()).unwrap();
        let color = capture.name("color").map(|x| x.as_str()).unwrap();
        let rules = capture.name("rules").map(|x| x.as_str()).unwrap();

        let mut relations: Vec<BagRelation> = Vec::new();
        for rule in rules.split(", ") {
            if rule == NO_OTHER_BAGS_STR {
                continue;
            }

            let rule_capture = SIMPLE_RULE_RGX.captures(rule.trim()).unwrap();
            let rule_amount = rule_capture.name("amount").map(|x| x.as_str()).unwrap();
            let rule_color = rule_capture.name("color").map(|x| x.as_str()).unwrap();
            relations.push(BagRelation::new(
                BagColor::new(rule_color),
                rule_amount.parse().unwrap(),
            ));
        }

        self.0.insert(BagColor::new(color), relations);
    }

    /// Parse multiple rules.
    ///
    /// # Arguments
    ///
    /// * `entries` - Input string
    pub fn parse_rules(&mut self, entries: &str) {
        for entry in entries.lines() {
            self.parse_rule(entry);
        }
    }

    /// Create new bag system.
    pub fn new_from_rules(rules: &str) -> Self {
        let mut instance = Self(HashMap::new());

        instance.parse_rules(rules);
        instance
    }

    /// Get direct links for a known bag color.
    ///
    /// # Arguments
    ///
    /// * `color` - Known color
    pub fn get_direct_links_for_color(&self, color: &BagColor) -> Vec<BagColor> {
        self.0
            .iter()
            .filter_map(|(k, v)| {
                if k == color {
                    None
                } else {
                    let values: Vec<BagColor> = v
                        .iter()
                        .filter_map(|r| {
                            if r.color == *color {
                                Some(k.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    Some(values)
                }
            })
            .flatten()
            .collect()
    }

    /// Find container bag colors for a target color.
    ///
    /// # Arguments
    ///
    /// * `color` - Known color
    pub fn find_container_colors_for_color(&self, color: &BagColor) -> Vec<BagColor> {
        let mut colors_to_scan: Vec<BagColor> = self.get_direct_links_for_color(color);
        let mut seen_colors: Vec<BagColor> = vec![];

        while let Some(scanned_color) = colors_to_scan.pop() {
            if seen_colors.contains(&scanned_color) {
                continue;
            }

            seen_colors.push(scanned_color.clone());
            let linked_colors = self.get_direct_links_for_color(&scanned_color);
            for linked_color in linked_colors {
                if seen_colors.contains(&linked_color)
                    || colors_to_scan.contains(&linked_color)
                    || &linked_color == color
                {
                    continue;
                }

                colors_to_scan.push(linked_color.clone());
            }
        }

        seen_colors
    }

    /// Count needed bags for a target color.
    ///
    /// # Arguments
    ///
    /// * `color` - Known color
    pub fn count_needed_bags_for_color(&self, color: &BagColor) -> usize {
        self.count_inner_bags_for_color(color) - 1
    }

    /// Count inner bags for a target color.
    ///
    /// # Arguments
    ///
    /// * `color` - Known color
    pub fn count_inner_bags_for_color(&self, inner_color: &BagColor) -> usize {
        let relations = self.0.get(inner_color).unwrap();
        let sum = relations
            .iter()
            .map(|x| x.amount * self.count_inner_bags_for_color(&x.color))
            .sum::<usize>()
            + 1;
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FIXTURE_EX1: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags."#;

    const EXAMPLE_FIXTURE_EX2: &str = r#"shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags."#;

    #[test]
    fn test_parse_rules() {
        BagSystem::new_from_rules(EXAMPLE_FIXTURE_EX1);
    }

    #[test]
    fn test_find_container_bag_colors() {
        let system = BagSystem::new_from_rules(EXAMPLE_FIXTURE_EX1);
        let color: BagColor = "shiny gold".into();
        let mut colors = system.find_container_colors_for_color(&color);
        colors.sort();

        let target: Vec<BagColor> =
            vec!["bright white", "dark orange", "light red", "muted yellow"]
                .into_iter()
                .map(Into::into)
                .collect();
        assert_eq!(colors, target);
    }

    #[test]
    fn test_count_needed_bags_for_color() {
        let system = BagSystem::new_from_rules(EXAMPLE_FIXTURE_EX2);
        let color: BagColor = "shiny gold".into();
        assert_eq!(system.count_needed_bags_for_color(&color), 126);
    }
}
