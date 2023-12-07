//! Common

use std::ops::Range;

use aoc_sx::{once_cell::sync::Lazy, regex::Regex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionMapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl ConversionMapType {
    pub fn from_input(input: &str) -> Self {
        static RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*) map:").unwrap());
        let capture = RGX.captures(input).unwrap();

        match capture.get(1).unwrap().as_str() {
            "seed-to-soil" => Self::SeedToSoil,
            "soil-to-fertilizer" => Self::SoilToFertilizer,
            "fertilizer-to-water" => Self::FertilizerToWater,
            "water-to-light" => Self::WaterToLight,
            "light-to-temperature" => Self::LightToTemperature,
            "temperature-to-humidity" => Self::TemperatureToHumidity,
            "humidity-to-location" => Self::HumidityToLocation,
            other => panic!("Unknown map type: {other}"),
        }
    }

    pub fn list_in_order() -> &'static [Self] {
        &[
            Self::SeedToSoil,
            Self::SoilToFertilizer,
            Self::FertilizerToWater,
            Self::WaterToLight,
            Self::LightToTemperature,
            Self::TemperatureToHumidity,
            Self::HumidityToLocation,
        ]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConversionMapLine {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl ConversionMapLine {
    pub fn from_input(input: &str) -> Self {
        static RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) (\d+) (\d+)").unwrap());
        let captures = RGX.captures(input).unwrap();

        Self {
            destination_range_start: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            source_range_start: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            range_length: captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }

    pub fn map_number(&self, number: usize) -> Option<usize> {
        let source_range = self.source_range_start..self.source_range_start + self.range_length;
        if source_range.contains(&number) {
            let offset = number - self.source_range_start;
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }

    pub fn reverse_map_number(&self, number: usize) -> Option<usize> {
        let dest_range =
            self.destination_range_start..self.destination_range_start + self.range_length;
        if dest_range.contains(&number) {
            let offset = number - self.destination_range_start;
            Some(self.source_range_start + offset)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConversionMap {
    map_type: ConversionMapType,
    lines: Vec<ConversionMapLine>,
}

impl ConversionMap {
    pub fn map_number(&self, number: usize) -> usize {
        for line in &self.lines {
            if let Some(value) = line.map_number(number) {
                return value;
            }
        }

        // Not found, return same number
        number
    }

    pub fn reverse_map_number(&self, number: usize) -> usize {
        for line in &self.lines {
            if let Some(value) = line.reverse_map_number(number) {
                return value;
            }
        }

        // Not found, return same number
        number
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Seeds {
    values: Vec<usize>,
}

impl Seeds {
    pub fn from_input(input: &str) -> Self {
        static RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"seeds: ((\d+\s*)+)").unwrap());

        let captures = RGX.captures(input.trim()).unwrap();
        Self {
            values: captures
                .get(1)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        }
    }

    pub fn seed_ranges(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        self.values
            .chunks_exact(2)
            .map(|elems| (elems[0]..elems[0] + elems[1]))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    seeds: Seeds,
    conversion_maps: Vec<ConversionMap>,
}

impl Almanac {
    pub fn from_input(input: &str) -> Self {
        let mut parts = input.trim().split("\n\n");

        let seeds = Seeds::from_input(parts.next().unwrap());
        let mut conversion_maps = vec![];

        for part in parts {
            let mut lines = part.lines();
            let conversion_map_type = ConversionMapType::from_input(lines.next().unwrap());
            let mut conversion_map_lines = vec![];

            for line in lines {
                let conversion_map_line = ConversionMapLine::from_input(line);
                conversion_map_lines.push(conversion_map_line);
            }

            conversion_maps.push(ConversionMap {
                map_type: conversion_map_type,
                lines: conversion_map_lines,
            });
        }

        Self {
            seeds,
            conversion_maps,
        }
    }

    pub fn get_location_number(&self, seed_number: usize) -> usize {
        let mut cursor = seed_number;

        for map in &self.conversion_maps {
            cursor = map.map_number(cursor);
        }

        cursor
    }

    pub fn get_seed_number_from_location(&self, seed_number: usize) -> usize {
        let mut cursor = seed_number;

        for map in self.conversion_maps.iter().rev() {
            cursor = map.reverse_map_number(cursor);
        }

        cursor
    }

    fn get_lowest_location_number_for_range(&self, range: Range<usize>) -> usize {
        range
            .into_iter()
            .map(|s| self.get_location_number(s))
            .min()
            .unwrap()
    }

    pub fn get_lowest_location_numbers_from_seeds(&self) -> usize {
        self.seeds
            .values
            .iter()
            .map(|&s| self.get_location_number(s))
            .min()
            .unwrap()
    }

    pub fn get_lowest_location_numbers_from_range_seeds(&self) -> usize {
        self.seeds
            .seed_ranges()
            .map(|r| self.get_lowest_location_number_for_range(r))
            .min()
            .unwrap()
    }

    pub fn get_lowest_location_numbers_using_reverse_bruteforce(&self) -> usize {
        for n in 0..usize::MAX {
            let seed_n = self.get_seed_number_from_location(n);
            for seed in self.seeds.seed_ranges() {
                if seed.contains(&seed_n) {
                    return n;
                }
            }
        }

        panic!("Nope")
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::{Almanac, ConversionMap, ConversionMapLine, ConversionMapType, Seeds};

    const SAMPLE: &str = indoc! {r#"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "#};

    #[test]
    fn parse() {
        let almanac = Almanac::from_input(SAMPLE);
        assert_eq!(
            almanac.seeds,
            Seeds {
                values: vec![79, 14, 55, 13]
            }
        );
        assert_eq!(
            almanac.conversion_maps[0],
            ConversionMap {
                map_type: ConversionMapType::SeedToSoil,
                lines: vec![
                    ConversionMapLine {
                        destination_range_start: 50,
                        source_range_start: 98,
                        range_length: 2
                    },
                    ConversionMapLine {
                        destination_range_start: 52,
                        source_range_start: 50,
                        range_length: 48
                    }
                ]
            }
        );
    }

    #[test]
    fn conversion() {
        let almanac = Almanac::from_input(SAMPLE);

        assert_eq!(almanac.conversion_maps[0].map_number(79), 81);
        assert_eq!(almanac.conversion_maps[0].map_number(14), 14);
        assert_eq!(almanac.conversion_maps[0].map_number(55), 57);
        assert_eq!(almanac.conversion_maps[0].map_number(13), 13);
    }

    #[test]
    fn location() {
        let almanac = Almanac::from_input(SAMPLE);

        assert_eq!(almanac.get_location_number(79), 82);
        assert_eq!(almanac.get_location_number(14), 43);
        assert_eq!(almanac.get_location_number(55), 86);
        assert_eq!(almanac.get_location_number(13), 35);
    }

    #[test]
    fn lowest() {
        let almanac = Almanac::from_input(SAMPLE);

        assert_eq!(almanac.get_lowest_location_numbers_from_seeds(), 35);
    }

    #[test]
    fn lowest_flattened_range_bruteforce() {
        let almanac = Almanac::from_input(SAMPLE);

        assert_eq!(
            almanac.get_lowest_location_numbers_using_reverse_bruteforce(),
            46
        );
    }

    #[test]
    fn lowest_flattened_ranges() {
        let almanac = Almanac::from_input(SAMPLE);

        assert_eq!(almanac.get_lowest_location_numbers_from_range_seeds(), 46);
    }
}
