//! Common

use std::collections::HashMap;

use aoc_sx::{itertools::Itertools, once_cell::sync::Lazy, regex::Regex};

#[derive(Debug, PartialEq, Eq)]
pub struct Scratchcard {
    id: usize,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Scratchcard {
    pub fn from_input(input: &str) -> Self {
        static RGX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"Card\s+(?P<id>\d+):\s+(?P<winning>(?:[0-9]+\s*)+)\|\s*(?P<yours>(?:[0-9]+\s*)+)",
            )
            .unwrap()
        });

        let mat = RGX.captures(input).unwrap();
        let card_id = mat
            .name("id")
            .expect("id capture group should exist")
            .as_str()
            .parse::<usize>()
            .unwrap();
        let winning_numbers = mat
            .name("winning")
            .expect("winning capture group should exist")
            .as_str()
            .split_whitespace()
            .map(|e| e.trim().parse::<u32>().unwrap())
            .collect_vec();
        let your_numbers = mat
            .name("yours")
            .expect("yours capture group should exist")
            .as_str()
            .split_whitespace()
            .map(|e| e.trim().parse::<u32>().unwrap())
            .collect_vec();

        Self {
            id: card_id,
            winning_numbers,
            your_numbers,
        }
    }

    pub fn winning_numbers_count(&self) -> usize {
        self.your_numbers
            .iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count()
    }

    pub fn get_points_value(&self) -> usize {
        2usize.pow((self.winning_numbers_count() - 1) as u32)
    }
}

pub struct ScratchcardPile {
    cards: Vec<Scratchcard>,
}

impl ScratchcardPile {
    pub fn from_input(input: &str) -> Self {
        Self {
            cards: input.trim().lines().map(Scratchcard::from_input).collect(),
        }
    }

    pub fn get_total_points_value(&self) -> usize {
        self.cards.iter().map(|c| c.get_points_value()).sum()
    }

    pub fn compute_cards_count(&self) -> usize {
        let card_with_winnings: HashMap<usize, usize> = self
            .cards
            .iter()
            .map(|c| (c.id, c.winning_numbers_count()))
            .collect();
        let mut remaining_cards_to_scan: Vec<usize> = self.cards.iter().map(|_| 1).collect();

        let mut count = 0;
        let mut current = 0;

        while current < remaining_cards_to_scan.len() {
            if remaining_cards_to_scan[current] == 0 {
                current += 1;
                continue;
            }

            let winning_count = *card_with_winnings.get(&(current + 1)).unwrap();
            for offset in 0..winning_count {
                remaining_cards_to_scan[current + offset + 1] += 1;
            }

            count += 1;
            remaining_cards_to_scan[current] -= 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::{Scratchcard, ScratchcardPile};

    const SAMPLE: &str = indoc! {r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#};

    const SAMPLE2: &str = indoc! {r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#};

    #[test]
    fn parse_scratchcard() {
        assert_eq!(
            Scratchcard::from_input("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Scratchcard {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                your_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        )
    }

    #[test]
    fn get_points_value() {
        assert_eq!(
            Scratchcard::from_input("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .get_points_value(),
            8
        );
    }

    #[test]
    fn get_total_points_value() {
        assert_eq!(
            ScratchcardPile::from_input(SAMPLE).get_total_points_value(),
            13
        );
    }

    #[test]
    fn get_total_scratchcards() {
        assert_eq!(
            ScratchcardPile::from_input(SAMPLE2).compute_cards_count(),
            30
        );
    }
}
