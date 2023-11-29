//! Common

use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
pub enum StrategyMode {
    Normal,
    FromOutcome,
}

#[derive(Debug, Copy, Clone)]
pub struct Match {
    opponent: Move,
    player: Move,
    strategy_mode: StrategyMode,
}

impl Match {
    pub fn new(opponent: Move, player: Move) -> Self {
        Self {
            opponent,
            player,
            strategy_mode: StrategyMode::Normal,
        }
    }

    pub fn set_strategy_mode(&mut self, mode: StrategyMode) {
        self.strategy_mode = mode;
    }

    pub fn outcome(&self) -> MatchOutcome {
        match (self.opponent, self.player) {
            (Move::Rock, Move::Paper) => MatchOutcome::Win,
            (Move::Rock, Move::Scissors) => MatchOutcome::Loss,
            (Move::Paper, Move::Rock) => MatchOutcome::Loss,
            (Move::Paper, Move::Scissors) => MatchOutcome::Win,
            (Move::Scissors, Move::Rock) => MatchOutcome::Win,
            (Move::Scissors, Move::Paper) => MatchOutcome::Loss,
            (_, _) => MatchOutcome::Draw,
        }
    }

    pub fn determine_player_move(&self, outcome: MatchOutcome) -> Move {
        match (self.opponent, outcome) {
            (Move::Rock, MatchOutcome::Loss) => Move::Scissors,
            (Move::Rock, MatchOutcome::Win) => Move::Paper,
            (Move::Paper, MatchOutcome::Loss) => Move::Rock,
            (Move::Paper, MatchOutcome::Win) => Move::Scissors,
            (Move::Scissors, MatchOutcome::Loss) => Move::Paper,
            (Move::Scissors, MatchOutcome::Win) => Move::Rock,
            (m, MatchOutcome::Draw) => m,
        }
    }

    pub fn run(&mut self) -> u32 {
        match self.strategy_mode {
            StrategyMode::Normal => self.outcome().score() + self.player.score(),
            StrategyMode::FromOutcome => {
                let wanted_outcome = MatchOutcome::from_player_move(self.player);
                wanted_outcome.score() + self.determine_player_move(wanted_outcome).score()
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MatchOutcome {
    Loss,
    Draw,
    Win,
}

impl MatchOutcome {
    pub fn from_player_move(m: Move) -> Self {
        match m {
            Move::Rock => Self::Loss,
            Move::Paper => Self::Draw,
            Move::Scissors => Self::Win,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl Move {
    pub fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unknown move {s}"),
        })
    }
}

#[derive(Debug)]
pub struct GameParser {
    moves: Vec<Match>,
}

impl GameParser {
    pub fn total_score_from_outcome(self) -> u32 {
        self.moves
            .into_iter()
            .map(|mut x| {
                x.set_strategy_mode(StrategyMode::FromOutcome);
                x.run()
            })
            .sum()
    }

    pub fn total_score(self) -> u32 {
        self.moves.into_iter().map(|mut x| x.run()).sum()
    }
}

impl FromStr for GameParser {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .lines()
            .map(|x| {
                x.split_whitespace()
                    .map(|m| Move::from_str(m).unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|moves| (Match::new(moves[0], moves[1])))
            .collect();

        Ok(Self { moves })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use aoc_sx::indoc::indoc;

    use super::GameParser;

    const SAMPLE: &str = indoc! {r#"
        A Y
        B X
        C Z
    "#};

    #[test]
    fn test_sample() {
        let moves = GameParser::from_str(SAMPLE).unwrap();
        assert_eq!(moves.total_score(), 15);
    }
}
