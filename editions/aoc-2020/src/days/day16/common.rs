//! Common

use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use thiserror::Error;

/// Day error.
#[derive(Debug, Error)]
pub enum DayError {
    /// Rule parse error
    #[error("Rule parse error: {0}")]
    RuleParseError(String),
    /// Input parse error
    #[error("Input parse error: {0}")]
    InputParseError(String),
    /// Ticket parse error
    #[error("Ticket parse error: {0}")]
    TicketParseError(String),
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Ticket rule.
#[derive(Debug, PartialEq, Eq)]
pub struct TicketRule {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl TicketRule {
    /// Validate number.
    ///
    /// # Arguments
    ///
    /// * `number` - Number
    pub fn validate_number(&self, number: usize) -> bool {
        self.ranges.iter().filter(|x| x.contains(&number)).count() > 0
    }
}

impl From<&str> for TicketRule {
    fn from(value: &str) -> Self {
        let mut split_iter = value.trim().split(':');
        let (name, ranges_rule) = (
            split_iter.next().unwrap().to_string(),
            split_iter.next().unwrap().to_string(),
        );

        let ranges: Vec<RangeInclusive<usize>> = ranges_rule
            .trim()
            .split("or")
            .map(|r| {
                let vec: Vec<_> = r
                    .trim()
                    .split('-')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();

                let first = vec.get(0).copied().unwrap();
                let second = vec.get(1).copied().unwrap();

                RangeInclusive::new(first, second)
            })
            .collect();

        Self { name, ranges }
    }
}

/// Ticket.
#[derive(Debug, PartialEq, Eq)]
pub struct Ticket {
    pub numbers: Vec<usize>,
}

impl From<&str> for Ticket {
    fn from(value: &str) -> Self {
        let numbers: Vec<_> = value
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Self { numbers }
    }
}

/// Input parser.
#[derive(Debug, PartialEq, Eq)]
pub struct InputParser {
    rules: Vec<TicketRule>,
    pub your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl InputParser {
    /// Validate number using rules.
    ///
    /// # Arguments
    ///
    /// * `number` - Number
    pub fn validate_number(&self, number: usize) -> bool {
        self.rules
            .iter()
            .filter(|&r| r.validate_number(number))
            .count()
            > 0
    }

    /// Validate ticket using rules.
    /// Returns numbers that are not valid (for all rules).
    ///
    /// # Arguments
    ///
    /// * `ticket` - Ticket
    pub fn validate_ticket(&self, ticket: &Ticket) -> Vec<usize> {
        ticket
            .numbers
            .iter()
            .filter(|&&n| !self.validate_number(n))
            .copied()
            .collect()
    }

    /// Validate nearby tickets.
    /// Returns numbers that are not valid (for all rules), for all tickets.
    pub fn validate_nearby_tickets(&self) -> Vec<usize> {
        self.nearby_tickets
            .iter()
            .flat_map(|t| self.validate_ticket(t))
            .collect()
    }

    /// Map ticket fields with position.
    pub fn map_ticket_fields(&self) -> HashMap<&str, usize> {
        // Filter nearby tickets
        let remaining_tickets: Vec<_> = self
            .nearby_tickets
            .iter()
            .filter(|t| self.validate_ticket(t).is_empty())
            .collect();

        let mut rules_affectation: HashMap<&str, usize> =
            self.rules.iter().map(|x| (&*x.name, usize::MAX)).collect();
        let mut remaining_rules: Vec<&str> = self.rules.iter().map(|x| &*x.name).collect();
        let mut invalid_positions: HashMap<&str, HashSet<usize>> = self
            .rules
            .iter()
            .map(|x| (&*x.name, HashSet::new()))
            .collect();
        let mut positions: HashMap<&str, Vec<usize>> =
            self.rules.iter().map(|x| (&*x.name, vec![])).collect();

        // Accumulate potential positions
        for t in &remaining_tickets {
            for r in &self.rules {
                for (idx, n) in t.numbers.iter().enumerate() {
                    if r.validate_number(*n) {
                        positions.get_mut(&*r.name).unwrap().push(idx);
                    } else {
                        invalid_positions.get_mut(&*r.name).unwrap().insert(idx);
                    }
                }
            }
        }

        while !remaining_rules.is_empty() {
            // Remove invalid positions from positions
            for (rn, counter) in &mut positions {
                let new_counter: Vec<_> = counter
                    .iter()
                    .filter(|x| invalid_positions.get(rn).map_or(true, |v| !v.contains(x)))
                    .copied()
                    .collect();

                *counter = new_counter;
            }

            // Get the most seen value per rule
            let mut rules_to_remove = vec![];
            for (idx, rn) in remaining_rules.iter().enumerate() {
                let counter = positions.get(rn).unwrap();
                let count_map = Self::count_occurences_in_vec(counter);

                // Simple case, only one number available in counter
                if count_map.len() == 1 {
                    let position = *count_map.keys().next().unwrap();
                    rules_affectation.insert(rn, position);
                    rules_to_remove.push(idx);

                    // Add as invalid positions for other rules
                    for r in remaining_rules.iter().filter(|&&o_rn| o_rn != *rn) {
                        invalid_positions.get_mut(r).unwrap().insert(position);
                    }
                }
            }

            // Remove already found rules
            while let Some(r_idx) = rules_to_remove.pop() {
                remaining_rules.remove(r_idx);
            }
        }

        // Increment affectations to match needed output
        for pos in rules_affectation.values_mut() {
            *pos += 1;
        }

        rules_affectation
    }

    fn count_occurences_in_vec(v: &[usize]) -> HashMap<usize, usize> {
        let mut m = HashMap::new();
        for n in v {
            *m.entry(*n).or_default() += 1;
        }
        m
    }
}

impl From<&str> for InputParser {
    fn from(value: &str) -> Self {
        let groups: Vec<&str> = value.trim().split("\n\n").collect();
        let rules_section = groups.get(0).cloned().unwrap();
        let your_ticket_section = groups.get(1).cloned().unwrap();
        let nearby_tickets_section = groups.get(2).cloned().unwrap();

        let rules: Vec<_> = rules_section.trim().lines().map(TicketRule::from).collect();
        let your_ticket = Ticket::from(your_ticket_section.trim().lines().nth(1).unwrap());
        let nearby_tickets: Vec<_> = nearby_tickets_section
            .trim()
            .lines()
            .skip(1)
            .map(Ticket::from)
            .collect();

        Self {
            rules,
            your_ticket,
            nearby_tickets,
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::{indoc::indoc, maplit::hashmap};

    use super::*;

    const SAMPLE: &str = indoc! {"
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12
    "};

    const SAMPLE_2: &str = indoc! {"
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9
    "};

    #[test]
    fn test_ticket_rule_parse() {
        assert_eq!(
            TicketRule::from("class: 1-3 or 5-7"),
            TicketRule {
                name: "class".into(),
                ranges: vec![
                    RangeInclusive::<usize>::new(1, 3),
                    RangeInclusive::<usize>::new(5, 7)
                ]
            }
        );
    }

    #[test]
    fn test_ticket_parse() {
        assert_eq!(
            Ticket::from("7,1,14"),
            Ticket {
                numbers: vec![7, 1, 14]
            }
        );
    }

    #[test]
    fn test_input_parse() {
        assert_eq!(
            InputParser::from(SAMPLE),
            InputParser {
                rules: vec![
                    TicketRule {
                        name: "class".into(),
                        ranges: vec![
                            RangeInclusive::<usize>::new(1, 3),
                            RangeInclusive::<usize>::new(5, 7),
                        ]
                    },
                    TicketRule {
                        name: "row".into(),
                        ranges: vec![
                            RangeInclusive::<usize>::new(6, 11),
                            RangeInclusive::<usize>::new(33, 44),
                        ]
                    },
                    TicketRule {
                        name: "seat".into(),
                        ranges: vec![
                            RangeInclusive::<usize>::new(13, 40),
                            RangeInclusive::<usize>::new(45, 50),
                        ]
                    }
                ],
                your_ticket: Ticket {
                    numbers: vec![7, 1, 14]
                },
                nearby_tickets: vec![
                    Ticket {
                        numbers: vec![7, 3, 47]
                    },
                    Ticket {
                        numbers: vec![40, 4, 50]
                    },
                    Ticket {
                        numbers: vec![55, 2, 20]
                    },
                    Ticket {
                        numbers: vec![38, 6, 12]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_validate_nearby_tickets() {
        let parser = InputParser::from(SAMPLE);
        assert_eq!(parser.validate_nearby_tickets(), vec![4, 55, 12]);
        assert_eq!(parser.validate_nearby_tickets().iter().sum::<usize>(), 71);
    }

    #[test]
    fn test_map_ticket_fields() {
        let parser = InputParser::from(SAMPLE_2);
        let res: HashMap<&str, usize> = hashmap! {
            "class" => 2,
            "row" => 1,
            "seat" => 3
        };

        assert_eq!(parser.map_ticket_fields(), res);
    }
}
