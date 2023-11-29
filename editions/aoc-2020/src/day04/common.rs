//! Common

use std::collections::HashMap;

use aoc_sx::{once_cell::sync::Lazy, regex::Regex};

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_EYE_COLOR: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
static HEIGHT_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<amount>\d+)(?P<unit>cm|in)$").unwrap());
static COLOR_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#(?P<color>[0-9a-f]{6})$").unwrap());
static PID_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<num>\d{9})$").unwrap());

/// Passport validator.
pub struct PassportValidator(HashMap<String, String>);

impl PassportValidator {
    /// Parse passport entry.
    ///
    /// # Arguments
    ///
    /// * `entry` - Passport entry
    pub fn parse_entry(entry: &str) -> Self {
        Self(
            entry
                .split_whitespace()
                .map(|e| {
                    let mut spl = e.split_terminator(':');
                    let key = spl.next().map(ToOwned::to_owned).unwrap();
                    let value = spl.next().map(ToOwned::to_owned).unwrap();
                    (key, value)
                })
                .collect(),
        )
    }

    /// Validate passport field.
    ///
    /// # Arguments
    ///
    /// * `key` - Field key
    /// * `value` - Field value
    pub fn try_validate_field(key: &str, value: &str) -> bool {
        match key {
            "byr" => {
                // At least 1920 at most 2002
                let value: usize = value.parse().unwrap();
                (1920..=2002).contains(&value)
            }
            "iyr" => {
                // At least 2010 at most 2020
                let value: usize = value.parse().unwrap();
                (2010..=2020).contains(&value)
            }
            "eyr" => {
                // At least 2020 at most 2030
                let value: usize = value.parse().unwrap();
                (2020..=2030).contains(&value)
            }
            "hgt" => {
                if let Some(v) = HEIGHT_RGX.captures(value) {
                    let amount = v
                        .name("amount")
                        .and_then(|x| x.as_str().parse::<usize>().ok())
                        .unwrap();
                    let unit = v.name("unit").map(|x| x.as_str()).unwrap();

                    match unit {
                        // At least 150 at most 193
                        "cm" => (150..=193).contains(&amount),
                        // At least 59 at most 76
                        "in" => (59..=76).contains(&amount),
                        _ => unreachable!(),
                    }
                } else {
                    false
                }
            }
            "hcl" => COLOR_RGX.is_match(value),
            "ecl" => VALID_EYE_COLOR.contains(&value),
            "pid" => PID_RGX.is_match(value),
            _ => false,
        }
    }

    /// Check if passport is valid.
    pub fn is_valid(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .filter(|&x| self.0.contains_key(*x))
            .count()
            == REQUIRED_FIELDS.len()
    }

    /// Check if passport is valid.
    pub fn is_valid_full(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .filter(|&x| {
                self.0
                    .get(*x)
                    .map_or(false, |v| Self::try_validate_field(x, v))
            })
            .count()
            == REQUIRED_FIELDS.len()
    }

    /// Parse multiple passport entries.
    ///
    /// # Arguments
    ///
    /// * `entries` - Passport entries
    pub fn parse_entries(entries: &str) -> Vec<Self> {
        entries.split("\n\n").map(Self::parse_entry).collect()
    }
}
