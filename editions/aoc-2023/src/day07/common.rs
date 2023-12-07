//! Common

use std::collections::HashMap;

use aoc_sx::itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum CardValue {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum HandType {
    FiveOfAKind {
        kind: CardValue
    },
    FourOfAKind {
        kind: CardValue,
        remainder: CardValue
    },
    FullHouse {
        greater: CardValue,
        lesser: CardValue
    },
    ThreeOfAKind {
        kind: CardValue,
        remainder: [CardValue; 2]
    },
    TwoPair {
        first: CardValue,
        second: CardValue,
        remainder: CardValue
    },
    OnePair {
        pair: CardValue,
        remainder: [CardValue; 3]
    },
    HighCard {
        greater: CardValue,
        remainder: [CardValue; 4]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    cards: [CardValue; 5],
    hand_type: Option<HandType>
}

impl Hand {
    pub fn from_input(input: &str) -> Self {

    }

    pub fn determine_hand_type(&mut self) {
        if self.hand_type.is_some() {
            return;
        }
    }

    fn determine_hand_type_inner(&self) -> HandType {
        let counter_map = self.counter_map();
        let sorted_values = counter_map.iter().map(|(&value, &count)| (value, count)).sorted_by_key(|&(_, count)| count).collect_vec();
        if counter_map.len() == 1 {
            // Five of a kind
            let kind = sorted_values[0].0;
            HandType::FiveOfAKind { kind }
        } else if counter_map.len() == 2 && sorted_values[0].1 == 4 {
            let kind = sorted_values[0].0;
            let remainder = sorted_values[1].0;
            HandType::FourOfAKind { kind, remainder }
        } else if counter_map.len() == 2 && sorted_values[0].1 == 3 {
            let greater = sorted_values[0].0;
            let lesser = sorted_values[1].0;
            HandType::FullHouse { greater, lesser }
        } else if counter_map.len() == 3 && sorted_values[0].1 == 3 {
            let kind = sorted_values[0].0;
            let remainder = sorted_values.iter().take(1).map(|&(value, _)| value).collect_vec();
            HandType::ThreeOfAKind { kind, remainder: [remainder[0], remainder[1]] }
        } else if counter_map.len() == 3 && sorted_values[0].1 == 2 && sorted_values[0].1 == 2 {
            let first = sorted_values[0].0;
            let second = sorted_values[1].0;
            let remainder = sorted_values[2].0;
            HandType::TwoPair { first, second, remainder }
        } else if counter_map.len() == 4 {
            let pair = sorted_values[0].0;
            let remainder = sorted_values.iter().take(1).map(|&(value, _)| value).collect_vec();
            HandType::OnePair { pair, remainder: [remainder[0], remainder[1], remainder[2]] }
        } else {
            let greater = sorted_values[0].0;
            let remainder = sorted_values.iter().take(1).map(|&(value, _)| value).collect_vec();
            HandType::HighCard { greater, remainder: [remainder[0], remainder[1], remainder[2], remainder[3]] }
        }
    }

    fn counter_map(&self) -> HashMap<CardValue, usize> {
        let mut counter_map = HashMap::new();

        for &card in &self.cards {
            counter_map
                .entry(card)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        counter_map
    }
}

#[derive(Debug)]
pub struct HandGame {
    hand: Hand,
    bid: usize
}

#[derive(Debug)]
pub struct Game {
    hand_games: Vec<HandGame>
}
