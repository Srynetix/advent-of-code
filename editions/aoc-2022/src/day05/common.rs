//! Common

use aoc_sx::{itertools::Itertools, once_cell::sync::Lazy, regex::Regex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateStackIndex(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crate(char);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CrateStack {
    stack: Vec<Crate>,
}

impl CrateStack {
    pub fn pop_n_elements(&mut self, n: usize) -> Vec<Crate> {
        let mut new_vec = vec![];

        for _ in 0..n {
            new_vec.push(self.stack.pop().unwrap());
        }

        new_vec
    }

    pub fn push_elements(&mut self, elements: Vec<Crate>) {
        self.stack.extend(elements);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RearrangementProcedure {
    amount: usize,
    source: CrateStackIndex,
    destination: CrateStackIndex,
}

impl RearrangementProcedure {
    pub fn from_input(input: &str) -> Self {
        static RGX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"move (?<amount>[0-9]+) from (?<source>[0-9+]) to (?<destination>[0-9]+)"#)
                .unwrap()
        });

        let captures = RGX.captures(input).unwrap();
        let amount = captures["amount"].parse::<usize>().unwrap();
        let source = captures["source"].parse::<usize>().unwrap();
        let destination = captures["destination"].parse::<usize>().unwrap();

        Self {
            amount,
            source: CrateStackIndex(source),
            destination: CrateStackIndex(destination),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ship {
    stacks: Vec<CrateStack>,
    procedures: Vec<RearrangementProcedure>,
    crane_type: CraneType,
}

impl Ship {
    pub fn from_input(input: &str) -> Self {
        let input_split = input.split("\n\n").collect::<Vec<&str>>();
        assert_eq!(input_split.len(), 2);

        let stack_input = input_split[0];
        let proc_input = input_split[1];

        let stacks = Self::parse_stacks(stack_input);
        let procedures = Self::parse_procedures(proc_input);

        Self {
            stacks,
            procedures,
            crane_type: CraneType::CrateMover9000,
        }
    }

    pub fn set_crane_type(&mut self, crane_type: CraneType) {
        self.crane_type = crane_type;
    }

    fn get_stack_count_from_input(input: &str) -> usize {
        static RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)^\s*1.*$"#).unwrap());
        let cap = RGX.captures(input).unwrap();

        cap.get(0)
            .unwrap()
            .as_str()
            .trim()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }

    fn parse_stacks(input: &str) -> Vec<CrateStack> {
        let stack_count = Self::get_stack_count_from_input(input);
        let mut stacks = vec![CrateStack::default(); stack_count];

        for line in input.lines() {
            if line.starts_with(" 1") {
                break;
            } else if line.is_empty() {
                continue;
            }

            for (line_index, (a, b, c)) in line.chars().tuple_windows().enumerate() {
                if a == '[' && c == ']' {
                    let stack_index = line_index / 4;
                    stacks[stack_index].stack.insert(0, Crate(b));
                }
            }
        }

        stacks
    }

    fn parse_procedures(input: &str) -> Vec<RearrangementProcedure> {
        input
            .lines()
            .map(RearrangementProcedure::from_input)
            .collect()
    }

    pub fn apply_internal_procedures(&mut self) {
        let procedures = self.procedures.clone();
        for proc in procedures {
            self.apply_procedure(&proc);
        }
    }

    pub fn apply_procedure(&mut self, procedure: &RearrangementProcedure) {
        let mut new_vec = self.stacks[procedure.source.0 - 1].pop_n_elements(procedure.amount);
        if self.crane_type == CraneType::CrateMover9001 {
            // Reverse!
            new_vec = new_vec.into_iter().rev().collect::<Vec<_>>();
        }

        self.stacks[procedure.destination.0 - 1].push_elements(new_vec);
    }

    pub fn get_stack_tops_to_string(&self) -> String {
        let mut output = String::new();
        for stack in &self.stacks {
            output.push(stack.stack[stack.stack.len() - 1].0);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "    [D]\n",
        "[N] [C]\n",
        "[Z] [M] [P]\n",
        " 1   2   3\n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2"
    );

    #[test]
    fn parse_rearrangement_procedure() {
        assert_eq!(
            RearrangementProcedure::from_input("move 1 from 2 to 1"),
            RearrangementProcedure {
                amount: 1,
                source: CrateStackIndex(2),
                destination: CrateStackIndex(1),
            }
        );

        assert_eq!(
            RearrangementProcedure::from_input("move 3 from 1 to 3"),
            RearrangementProcedure {
                amount: 3,
                source: CrateStackIndex(1),
                destination: CrateStackIndex(3)
            }
        );
    }

    #[test]
    fn parse_ship() {
        let ship = Ship::from_input(SAMPLE);

        assert_eq!(
            ship,
            Ship {
                crane_type: CraneType::CrateMover9000,
                stacks: vec![
                    CrateStack {
                        stack: vec![Crate('Z'), Crate('N')],
                    },
                    CrateStack {
                        stack: vec![Crate('M'), Crate('C'), Crate('D')],
                    },
                    CrateStack {
                        stack: vec![Crate('P')]
                    },
                ],
                procedures: vec![
                    RearrangementProcedure {
                        amount: 1,
                        source: CrateStackIndex(2),
                        destination: CrateStackIndex(1),
                    },
                    RearrangementProcedure {
                        amount: 3,
                        source: CrateStackIndex(1),
                        destination: CrateStackIndex(3),
                    },
                    RearrangementProcedure {
                        amount: 2,
                        source: CrateStackIndex(2),
                        destination: CrateStackIndex(1),
                    },
                    RearrangementProcedure {
                        amount: 1,
                        source: CrateStackIndex(1),
                        destination: CrateStackIndex(2),
                    }
                ]
            }
        );
    }

    #[test]
    fn apply_procedures_9000() {
        let mut ship = Ship::from_input(SAMPLE);
        ship.apply_internal_procedures();

        assert_eq!(ship.get_stack_tops_to_string(), "CMZ");
    }

    #[test]
    fn apply_procedures_9001() {
        let mut ship = Ship::from_input(SAMPLE);
        ship.set_crane_type(CraneType::CrateMover9001);
        ship.apply_internal_procedures();

        assert_eq!(ship.get_stack_tops_to_string(), "MCD");
    }
}
