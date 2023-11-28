use std::{fmt::Display, str::FromStr};

const MIN_YEAR: u16 = 2015;
const MAX_YEAR: u16 = 2023;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid year {0}, no content available.")]
    InvalidYear(u16),

    #[error("Invalid day: {0}")]
    InvalidDay(u8),

    #[error("Invalid part: {0}")]
    InvalidPart(String),
}

#[derive(Debug, Clone, Copy)]
pub struct ExerciseYear(u16);

impl Display for ExerciseYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}", self.0))
    }
}

impl TryFrom<u16> for ExerciseYear {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < MIN_YEAR || value > MAX_YEAR {
            Err(Error::InvalidYear(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl FromStr for ExerciseYear {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u16_value: u16 = s.parse().map_err(|_| Error::InvalidYear(0))?;
        Self::try_from(u16_value)
    }
}

impl FromStr for ExerciseDay {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u8_value: u8 = s.parse().map_err(|_| Error::InvalidDay(0))?;
        Self::try_from(u8_value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExerciseDay(u8);

impl Display for ExerciseDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl TryFrom<u8> for ExerciseDay {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 25 {
            Err(Error::InvalidDay(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl ExerciseDay {
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExercisePart {
    First,
    Second,
}

impl ExercisePart {
    pub fn as_level(&self) -> &'static str {
        match self {
            Self::First => "1",
            Self::Second => "2",
        }
    }
}

impl TryFrom<&str> for ExercisePart {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &value.to_lowercase()[..] {
            "first" => Ok(Self::First),
            "second" => Ok(Self::Second),
            other => Err(Error::InvalidPart(other.into())),
        }
    }
}

impl FromStr for ExercisePart {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
