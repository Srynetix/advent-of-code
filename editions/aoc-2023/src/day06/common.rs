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

    fn get_min_time_to_beat_the_record(&self) -> usize {
        let mut scan_range = 0..self.time / 2;

        loop {
            let dist = (scan_range.end - scan_range.start).div_ceil(2);
            let n = scan_range.start + dist;

            if n * (self.time - n) > self.distance {
                if dist == 1 {
                    // That's the one!
                    return n;
                } else {
                    scan_range.end = n;
                }
            } else {
                scan_range.start = n;
            }

            if dist == 0 {
                panic!("Oops, cannot beat record.")
            }
        }
    }

    pub fn count_ways_to_beat_the_record(&self) -> usize {
        1 + self.time - self.get_min_time_to_beat_the_record() * 2
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

    pub fn with_merged_distances(&self) -> Self {
        let (new_time, new_distance) = self.races.iter().map(|r| (r.time, r.distance)).fold(
            (String::new(), String::new()),
            |(mut t_acc, mut d_acc), (t, d)| {
                t_acc.push_str(&t.to_string());
                d_acc.push_str(&d.to_string());
                (t_acc, d_acc)
            },
        );

        Self {
            races: vec![BoatRace::new(
                new_time.parse::<usize>().unwrap(),
                new_distance.parse::<usize>().unwrap(),
            )],
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
    use aoc_sx::indoc::indoc;

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
        assert_eq!(races.races[0].count_ways_to_beat_the_record(), 4);
        assert_eq!(races.races[1].count_ways_to_beat_the_record(), 8);
        assert_eq!(races.races[2].count_ways_to_beat_the_record(), 9);

        assert_eq!(races.product_ways_to_beat_records(), 288);
    }

    #[test]
    fn ways_to_beat_the_record_merged() {
        let races = BoatRaces::from_input(SAMPLE).with_merged_distances();

        assert_eq!(races.product_ways_to_beat_records(), 71_503);
    }
}
