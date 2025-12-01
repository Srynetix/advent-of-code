//! Common

use std::{fmt::Display, str::FromStr};

pub struct Safe {
    arrow: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rotation {
    pub direction: Direction,
    pub degrees: usize,
}

impl Rotation {
    pub fn normalize(self) -> (Self, usize) {
        let full_rotations = self.degrees / 100;
        let remaining_degrees = self.degrees % 100;
        (
            Rotation {
                direction: self.direction,
                degrees: remaining_degrees,
            },
            full_rotations,
        )
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = match self.direction {
            Direction::Left => "L",
            Direction::Right => "R",
        };
        write!(f, "{}{}", dir, self.degrees)
    }
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, deg) = s.split_at(1);
        let direction = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(()),
        };
        let degrees = deg.parse().unwrap();
        Ok(Rotation { direction, degrees })
    }
}

pub fn parse_document(input: &str) -> Vec<Rotation> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

impl Safe {
    pub fn new() -> Self {
        Self { arrow: 50 }
    }

    pub fn rotate(&mut self, rotation: Rotation) -> usize {
        let mut zeroes = 0;

        for _ in 0..rotation.degrees {
            self.step(rotation.direction);
            if self.arrow == 0 {
                zeroes += 1;
            }
        }

        zeroes
    }

    pub fn step(&mut self, direction: Direction) {
        self.arrow = match direction {
            Direction::Left => (self.arrow as isize - 1).rem_euclid(100) as usize,
            Direction::Right => (self.arrow + 1).rem_euclid(100),
        }
    }

    pub fn peek(&self) -> usize {
        self.arrow
    }
}

impl Default for Safe {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rotation() {
        let r: Rotation = "L90".parse().unwrap();
        assert_eq!(
            r,
            Rotation {
                direction: Direction::Left,
                degrees: 90
            }
        );

        let r: Rotation = "R90".parse().unwrap();
        assert_eq!(
            r,
            Rotation {
                direction: Direction::Right,
                degrees: 90
            }
        );

        let r: Rotation = "L0".parse().unwrap();
        assert_eq!(
            r,
            Rotation {
                direction: Direction::Left,
                degrees: 0
            }
        );
    }
}
