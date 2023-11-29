//! Common

use std::{convert::Infallible, str::FromStr};

use aoc_sx::itertools::Itertools;

#[derive(Debug)]
pub struct ElfCalorieReader {
    calories_per_elf: Vec<u32>,
}

impl ElfCalorieReader {
    pub fn max_calories(&self) -> u32 {
        *self.calories_per_elf.iter().max().unwrap()
    }

    pub fn top_three_max_calories(&self) -> u32 {
        self.calories_per_elf.iter().sorted().rev().take(3).sum()
    }
}

impl FromStr for ElfCalorieReader {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories = s
            .split("\n\n")
            .map(|subl| subl.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
            .collect();

        Ok(Self {
            calories_per_elf: calories,
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use aoc_sx::indoc::indoc;

    use super::ElfCalorieReader;

    const SAMPLE: &str = indoc! {r#"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "#};

    #[test]
    fn test_sample() {
        let reader = ElfCalorieReader::from_str(SAMPLE).unwrap();
        assert_eq!(reader.max_calories(), 24_000);
    }
}
