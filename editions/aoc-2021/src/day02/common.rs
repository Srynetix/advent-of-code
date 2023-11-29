//! Common

#[derive(Default)]
pub struct Submarine {
    depth: i32,
    horizontal: i32,
    aim: Option<i32>,
}

#[derive(Debug, PartialEq)]
enum MoveDirection {
    Up,
    Down,
    Forward,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    direction: MoveDirection,
    amount: i32,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        let direction = split
            .next()
            .expect("First parameter should be a valid direction")
            .into();
        let amount = split
            .next()
            .and_then(|x| x.parse().ok())
            .expect("Second parameter should be a valid integer");

        Self { direction, amount }
    }
}

impl From<&str> for MoveDirection {
    fn from(value: &str) -> Self {
        match value {
            "forward" => Self::Forward,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => panic!("Unknown direction '{}'.", value),
        }
    }
}

impl Submarine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_aim() -> Self {
        Self {
            depth: 0,
            horizontal: 0,
            aim: Some(0),
        }
    }

    pub fn from_moves(input: &[&str]) -> Self {
        let mut sub = Self::new();
        sub.execute_moves(&Self::parse_moves(input));
        sub
    }

    pub fn from_moves_with_aim(input: &[&str]) -> Self {
        let mut sub = Self::new_with_aim();
        sub.execute_moves(&Self::parse_moves(input));
        sub
    }

    pub fn parse_moves(input: &[&str]) -> Vec<Move> {
        input.iter().map(|&d| d.into()).collect()
    }

    pub fn execute_moves(&mut self, moves: &[Move]) {
        for mv in moves {
            match self.aim {
                None => match mv.direction {
                    MoveDirection::Up => self.depth -= mv.amount,
                    MoveDirection::Down => self.depth += mv.amount,
                    MoveDirection::Forward => self.horizontal += mv.amount,
                },
                Some(a) => match mv.direction {
                    MoveDirection::Up => self.aim = Some(a - mv.amount),
                    MoveDirection::Down => self.aim = Some(a + mv.amount),
                    MoveDirection::Forward => {
                        self.horizontal += mv.amount;
                        self.depth += a * mv.amount;
                    }
                },
            }
        }
    }

    pub fn score(&self) -> i32 {
        self.depth * self.horizontal
    }
}

#[cfg(test)]
mod tests {
    use super::{Move, MoveDirection, Submarine};

    const SAMPLE_DATA: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_parse_move() {
        let moves: Vec<Move> = Submarine::parse_moves(&SAMPLE_DATA);
        assert_eq!(
            moves,
            vec![
                Move {
                    direction: MoveDirection::Forward,
                    amount: 5
                },
                Move {
                    direction: MoveDirection::Down,
                    amount: 5
                },
                Move {
                    direction: MoveDirection::Forward,
                    amount: 8
                },
                Move {
                    direction: MoveDirection::Up,
                    amount: 3
                },
                Move {
                    direction: MoveDirection::Down,
                    amount: 8
                },
                Move {
                    direction: MoveDirection::Forward,
                    amount: 2
                },
            ]
        )
    }

    #[test]
    fn test_execute_moves() {
        let sub = Submarine::from_moves(&SAMPLE_DATA);
        assert_eq!((sub.horizontal, sub.depth, sub.score()), (15, 10, 150));
    }

    #[test]
    fn test_execute_moves_with_aim() {
        let sub = Submarine::from_moves_with_aim(&SAMPLE_DATA);
        assert_eq!((sub.horizontal, sub.depth, sub.score()), (15, 60, 900));
    }
}
