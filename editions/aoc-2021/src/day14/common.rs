//! Common

use std::collections::{hash_map::Entry, HashMap};

use aoc_sx::itertools::{Itertools, MinMaxResult};

#[derive(Debug)]
pub struct Polymer {
    template: String,
    pairs: HashMap<(char, char), char>,
}

pub struct PolymerSum<'a> {
    polymer: &'a Polymer,
    pairs_count: HashMap<(char, char), usize>,
    char_count: HashMap<char, usize>,
}

impl<'a> PolymerSum<'a> {
    pub fn new(polymer: &'a Polymer) -> Self {
        let mut pairs_count: HashMap<(char, char), usize> = HashMap::new();
        let mut char_count: HashMap<char, usize> = HashMap::new();

        let chars: Vec<_> = polymer.template.chars().collect();
        for window in chars.windows(2) {
            let pair = (window[0], window[1]);
            *pairs_count.entry(pair).or_insert(0) += 1;
        }

        for c in chars {
            *char_count.entry(c).or_insert(0) += 1;
        }

        Self {
            polymer,
            pairs_count,
            char_count,
        }
    }

    pub fn step(&mut self) {
        let mut new_pair_counts = self.pairs_count.clone();
        let mut new_char_counts = self.char_count.clone();

        for (&(p0, p1), &target) in &self.polymer.pairs {
            let mut found = false;
            let mut found_value = 0;
            if let Entry::Occupied(e) = self.pairs_count.entry((p0, p1)) {
                if *e.get() > 0 {
                    found_value = *e.get();
                    *new_pair_counts.entry((p0, p1)).or_insert(0) -= found_value;
                    found = true;
                }
            }

            if found {
                *new_pair_counts.entry((p0, target)).or_insert(0) += found_value;
                *new_pair_counts.entry((target, p1)).or_insert(0) += found_value;
                *new_char_counts.entry(target).or_insert(0) += found_value;
            }
        }

        self.char_count = new_char_counts;
        self.pairs_count = new_pair_counts;
    }

    pub fn step_for(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }

    pub fn get_common_score(&self) -> u64 {
        match self.char_count.iter().minmax_by_key(|(_, &c)| c) {
            MinMaxResult::MinMax((_, &min), (_, &max)) => max as u64 - min as u64,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Polymer {
    fn from(s: &str) -> Self {
        let mut split = s.split("\n\n");
        let template = split.next().unwrap().to_string();
        let pairs = split
            .next()
            .unwrap()
            .lines()
            .map(|x| {
                let mut split = x.split(" -> ");
                let p0 = split.next().unwrap().chars().collect::<Vec<_>>();
                let p1 = split.next().unwrap();
                ((p0[0], p0[1]), p1.chars().next().unwrap())
            })
            .collect();

        Self { template, pairs }
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::{Polymer, PolymerSum};

    const SAMPLE_DATA: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C"
    };

    #[test]
    fn test_sample() {
        let polymer = Polymer::from(SAMPLE_DATA);
        let mut chain = PolymerSum::new(&polymer);
        chain.step_for(10);
        assert_eq!(chain.get_common_score(), 1588);
    }

    #[test]
    fn test_sample_40() {
        let polymer = Polymer::from(SAMPLE_DATA);
        let mut chain = PolymerSum::new(&polymer);
        chain.step_for(40);
        assert_eq!(chain.get_common_score(), 2_188_189_693_529)
    }
}
