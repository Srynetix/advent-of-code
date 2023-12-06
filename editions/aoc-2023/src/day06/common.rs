//! Common

use aoc_sx::{itertools::Itertools, once_cell::sync::Lazy, regex::Regex};

#[derive(Debug, PartialEq, Eq)]
pub struct BoatRace {
    time: usize,
    distance: usize,
}

impl BoatRace {
    pub fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    pub fn can_win_by_pressing_button_for_milliseconds(&self, time: usize) -> bool {
        if time >= self.time {
            return false;
        }

        let remaining_time = self.time - time;
        let speed = time;

        speed * remaining_time > self.distance
    }

    pub fn ways_to_beat_the_record(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.time).filter(|&v| self.can_win_by_pressing_button_for_milliseconds(v))
    }

    pub fn count_ways_to_beat_the_record(&self) -> usize {
        self.ways_to_beat_the_record().count()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoatRaces {
    races: Vec<BoatRace>,
}

impl BoatRaces {
    pub fn from_input(input: &str) -> Self {
        static TIME_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Time:\s+(.*)").unwrap());
        static DISTANCE_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Distance:\s+(.*)").unwrap());

        let times = TIME_RGX
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();
        let distances = DISTANCE_RGX
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();

        Self {
            races: times
                .into_iter()
                .zip(distances)
                .map(|(t, d)| BoatRace::new(t, d))
                .collect(),
        }
    }

    pub fn product_ways_to_beat_records(&self) -> usize {
        self.races
            .iter()
            .map(|v| v.count_ways_to_beat_the_record())
            .product()
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::{indoc::indoc, itertools::Itertools};

    use super::{BoatRace, BoatRaces};

    const SAMPLE: &str = indoc! {r#"
        Time:      7  15   30
        Distance:  9  40  200
    "#};

    #[test]
    fn parse() {
        let races = BoatRaces::from_input(SAMPLE);
        assert_eq!(
            races,
            BoatRaces {
                races: vec![
                    BoatRace {
                        time: 7,
                        distance: 9
                    },
                    BoatRace {
                        time: 15,
                        distance: 40
                    },
                    BoatRace {
                        time: 30,
                        distance: 200
                    }
                ]
            }
        )
    }

    #[test]
    fn ways_to_beat_the_record() {
        let races = BoatRaces::from_input(SAMPLE);
        assert_eq!(
            races.races[0].ways_to_beat_the_record().collect_vec(),
            &[2, 3, 4, 5]
        );
        assert_eq!(races.races[1].count_ways_to_beat_the_record(), 8);
        assert_eq!(races.races[2].count_ways_to_beat_the_record(), 9);

        assert_eq!(races.product_ways_to_beat_records(), 288);
    }
}
