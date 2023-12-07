//! Common

use std::{cmp::Ordering, collections::HashMap};

use aoc_sx::itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    CA,
}

impl CardValue {
    pub fn from_input(input: char) -> Self {
        match input {
            '2' => Self::C2,
            '3' => Self::C3,
            '4' => Self::C4,
            '5' => Self::C5,
            '6' => Self::C6,
            '7' => Self::C7,
            '8' => Self::C8,
            '9' => Self::C9,
            'T' => Self::CT,
            'J' => Self::CJ,
            'Q' => Self::CQ,
            'K' => Self::CK,
            'A' => Self::CA,
            other => panic!("Unknown card value: {other}"),
        }
    }

    pub fn rank_value(&self) -> usize {
        match self {
            Self::C2 => 2,
            Self::C3 => 3,
            Self::C4 => 4,
            Self::C5 => 5,
            Self::C6 => 6,
            Self::C7 => 7,
            Self::C8 => 8,
            Self::C9 => 9,
            Self::CT => 10,
            Self::CJ => 11,
            Self::CQ => 12,
            Self::CK => 13,
            Self::CA => 14,
        }
    }

    pub fn rank_value_with_joker(&self) -> usize {
        match self {
            Self::CJ => 1,
            Self::C2 => 2,
            Self::C3 => 3,
            Self::C4 => 4,
            Self::C5 => 5,
            Self::C6 => 6,
            Self::C7 => 7,
            Self::C8 => 8,
            Self::C9 => 9,
            Self::CT => 10,
            Self::CQ => 12,
            Self::CK => 13,
            Self::CA => 14,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HandType {
    HighCard {
        greater: CardValue,
        remainder: [CardValue; 4],
    },
    OnePair {
        pair: CardValue,
        remainder: [CardValue; 3],
    },
    TwoPair {
        first: CardValue,
        second: CardValue,
        remainder: CardValue,
    },
    ThreeOfAKind {
        kind: CardValue,
        remainder: [CardValue; 2],
    },
    FullHouse {
        greater: CardValue,
        lesser: CardValue,
    },
    FourOfAKind {
        kind: CardValue,
        remainder: CardValue,
    },
    FiveOfAKind {
        kind: CardValue,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cards([CardValue; 5]);

impl Cards {
    pub fn from_input(input: &str) -> Self {
        Self(
            input
                .trim()
                .chars()
                .map(CardValue::from_input)
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CardsWithJoker([CardValue; 5]);

impl CardsWithJoker {
    pub fn from_input(input: &str) -> Self {
        Self(
            input
                .trim()
                .chars()
                .map(CardValue::from_input)
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }

    pub fn iter_without_jokers(&self) -> impl Iterator<Item = CardValue> + '_ {
        self.0.iter().copied().filter(|&c| c != CardValue::CJ)
    }

    pub fn jokers_count(&self) -> usize {
        self.0.iter().filter(|&&c| c == CardValue::CJ).count()
    }
}

impl HandType {
    pub fn from_cards(cards: &Cards) -> Self {
        let counter_map = Self::counter_map(cards);
        let sorted_values = counter_map
            .iter()
            .map(|(&value, &count)| (value, count))
            .sorted_by_key(|&(value, count)| (-(count as isize), value.rank_value()))
            .collect_vec();
        Self::from_sorted_values(sorted_values)
    }

    fn from_sorted_values(sorted_values: Vec<(CardValue, usize)>) -> Self {
        if sorted_values.len() == 1 {
            // Five of a kind
            let kind = sorted_values[0].0;
            HandType::FiveOfAKind { kind }
        } else if sorted_values.len() == 2 && sorted_values[0].1 == 4 {
            let kind = sorted_values[0].0;
            let remainder = sorted_values[1].0;
            HandType::FourOfAKind { kind, remainder }
        } else if sorted_values.len() == 2 && sorted_values[0].1 == 3 {
            let greater = sorted_values[0].0;
            let lesser = sorted_values[1].0;
            HandType::FullHouse { greater, lesser }
        } else if sorted_values.len() == 3 && sorted_values[0].1 == 3 {
            let kind = sorted_values[0].0;
            let remainder = sorted_values
                .iter()
                .skip(1)
                .map(|&(value, _)| value)
                .sorted_by_key(|v| v.rank_value())
                .collect_vec();
            HandType::ThreeOfAKind {
                kind,
                remainder: [remainder[0], remainder[1]],
            }
        } else if sorted_values.len() == 3 && sorted_values[0].1 == 2 && sorted_values[1].1 == 2 {
            let first = sorted_values[0].0;
            let second = sorted_values[1].0;
            let remainder = sorted_values[2].0;
            HandType::TwoPair {
                first,
                second,
                remainder,
            }
        } else if sorted_values.len() == 4 {
            let pair = sorted_values[0].0;
            let remainder = sorted_values
                .iter()
                .skip(1)
                .map(|&(value, _)| value)
                .sorted_by_key(|v| v.rank_value())
                .collect_vec();
            HandType::OnePair {
                pair,
                remainder: [remainder[0], remainder[1], remainder[2]],
            }
        } else {
            let greater = sorted_values[0].0;
            let remainder = sorted_values
                .iter()
                .skip(1)
                .map(|&(value, _)| value)
                .sorted_by_key(|v| v.rank_value())
                .collect_vec();
            HandType::HighCard {
                greater,
                remainder: [remainder[0], remainder[1], remainder[2], remainder[3]],
            }
        }
    }

    pub fn from_cards_with_joker(cards: &CardsWithJoker) -> Self {
        let mut counter_map = HashMap::new();

        for card in cards.iter_without_jokers() {
            counter_map.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        let remaining_jokers = cards.jokers_count();
        if remaining_jokers == 5 {
            // Edge case!
            return Self::FiveOfAKind {
                kind: CardValue::CJ,
            };
        }

        // Inject jokers in higher values
        let mut sorted_values = counter_map
            .iter()
            .map(|(&value, &count)| (value, count))
            .sorted_by_key(|&(value, count)| (-(count as isize), value.rank_value()))
            .collect_vec();
        sorted_values[0].1 += remaining_jokers;

        Self::from_sorted_values(sorted_values)
    }

    fn strength(&self) -> usize {
        match self {
            Self::HighCard { .. } => 1,
            Self::OnePair { .. } => 2,
            Self::TwoPair { .. } => 3,
            Self::ThreeOfAKind { .. } => 4,
            Self::FullHouse { .. } => 5,
            Self::FourOfAKind { .. } => 6,
            Self::FiveOfAKind { .. } => 7,
        }
    }

    fn counter_map(cards: &Cards) -> HashMap<CardValue, usize> {
        let mut counter_map = HashMap::new();

        for &card in &cards.0 {
            counter_map.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        counter_map
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: Cards,
    hand_type: HandType,
}

impl Hand {
    pub fn from_input(input: &str) -> Self {
        let cards = Cards::from_input(input);
        let hand_type = HandType::from_cards(&cards);

        Self { cards, hand_type }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.strength().cmp(&other.hand_type.strength()) {
            Ordering::Equal => {
                // Compare each card
                for (c1, c2) in self.cards.0.iter().zip(other.cards.0.iter()) {
                    match c1.rank_value().cmp(&c2.rank_value()) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }

                Ordering::Equal
            }
            other => other,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HandWithJoker {
    cards: CardsWithJoker,
    hand_type: HandType,
}

impl HandWithJoker {
    pub fn from_input(input: &str) -> Self {
        let cards = CardsWithJoker::from_input(input);
        let hand_type = HandType::from_cards_with_joker(&cards);

        Self { cards, hand_type }
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.strength().cmp(&other.hand_type.strength()) {
            Ordering::Equal => {
                // Compare each card
                for (c1, c2) in self.cards.0.iter().zip(other.cards.0.iter()) {
                    match c1.rank_value_with_joker().cmp(&c2.rank_value_with_joker()) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }

                Ordering::Equal
            }
            other => other,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HandGame {
    hand: Hand,
    bid: usize,
}

impl PartialOrd for HandGame {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandGame {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl HandGame {
    pub fn from_input(input: &str) -> Self {
        let split = input.split_whitespace().collect_vec();
        let hand = Hand::from_input(split[0]);
        let bid = split[1].parse::<usize>().unwrap();

        Self { hand, bid }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HandGameWithJoker {
    hand: HandWithJoker,
    bid: usize,
}

impl PartialOrd for HandGameWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandGameWithJoker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl HandGameWithJoker {
    pub fn from_input(input: &str) -> Self {
        let split = input.split_whitespace().collect_vec();
        let hand = HandWithJoker::from_input(split[0]);
        let bid = split[1].parse::<usize>().unwrap();

        Self { hand, bid }
    }
}

#[derive(Debug)]
pub struct Game {
    hand_games: Vec<HandGame>,
}

impl Game {
    pub fn from_input(input: &str) -> Self {
        Self {
            hand_games: input.trim().lines().map(HandGame::from_input).collect(),
        }
    }

    pub fn total_winnings(&self) -> usize {
        self.hand_games
            .iter()
            .sorted()
            .enumerate()
            .map(|(idx, h)| (idx + 1) * h.bid)
            .sum()
    }
}

#[derive(Debug)]
pub struct GameWithJoker {
    hand_games: Vec<HandGameWithJoker>,
}

impl GameWithJoker {
    pub fn from_input(input: &str) -> Self {
        Self {
            hand_games: input
                .trim()
                .lines()
                .map(HandGameWithJoker::from_input)
                .collect(),
        }
    }

    pub fn total_winnings(&self) -> usize {
        self.hand_games
            .iter()
            .sorted()
            .enumerate()
            .map(|(idx, h)| (idx + 1) * h.bid)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use crate::day07::common::{CardsWithJoker, GameWithJoker, HandType};

    use super::{CardValue, Game};

    const SAMPLE: &str = indoc! {r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "#};

    #[test]
    fn rank() {
        let hands = Game::from_input(SAMPLE);
        assert_eq!(hands.total_winnings(), 6440);
    }

    #[test]
    fn jokers() {
        let cards = CardsWithJoker::from_input("KTJJT");
        assert_eq!(
            HandType::from_cards_with_joker(&cards),
            HandType::FourOfAKind {
                kind: CardValue::CT,
                remainder: CardValue::CK
            }
        );

        let cards = CardsWithJoker::from_input("T55J5");
        assert_eq!(
            HandType::from_cards_with_joker(&cards),
            HandType::FourOfAKind {
                kind: CardValue::C5,
                remainder: CardValue::CT
            }
        );

        let cards = CardsWithJoker::from_input("QQQJA");
        assert_eq!(
            HandType::from_cards_with_joker(&cards),
            HandType::FourOfAKind {
                kind: CardValue::CQ,
                remainder: CardValue::CA
            }
        );

        let hands = GameWithJoker::from_input(SAMPLE);
        assert_eq!(hands.total_winnings(), 5905);
    }
}
