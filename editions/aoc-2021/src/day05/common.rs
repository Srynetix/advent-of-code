//! Common

use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PointI32 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct LineI32 {
    a: PointI32,
    b: PointI32,
}

pub struct LineParser;
pub struct LineCollisioner;

impl PointI32 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl LineI32 {
    fn cmp_i32(a: i32, b: i32) -> i32 {
        match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    pub fn iter_points(&self) -> Vec<PointI32> {
        let mut points = Vec::new();

        let x_dir = Self::cmp_i32(self.a.x, self.b.x);
        let y_dir = Self::cmp_i32(self.a.y, self.b.y);
        let len = (self.a.x - self.b.x).abs().max((self.a.y - self.b.y).abs());

        for count in 0..=len {
            let x = self.a.x + count * x_dir;
            let y = self.a.y + count * y_dir;
            points.push(PointI32::new(x, y));
        }

        points
    }
}

impl From<&str> for LineI32 {
    fn from(s: &str) -> Self {
        let vecs = s
            .split(" -> ")
            .map(|n| {
                let nums = n
                    .split(',')
                    .flat_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<i32>>();

                PointI32::new(nums[0], nums[1])
            })
            .collect::<Vec<PointI32>>();

        Self {
            a: vecs[0],
            b: vecs[1],
        }
    }
}

impl LineParser {
    pub fn parse_lines(input: &[&str]) -> Vec<LineI32> {
        input.iter().map(|&x| x.into()).collect()
    }
}

impl LineCollisioner {
    pub fn filter_horizontal_and_vertical(lines: &[LineI32]) -> Vec<LineI32> {
        lines
            .iter()
            .filter(|&l| l.a.x == l.b.x || l.a.y == l.b.y)
            .cloned()
            .collect()
    }

    pub fn scan_line_intersections(lines: &[LineI32]) -> HashMap<PointI32, u32> {
        let mut intersections = HashMap::new();

        for line in lines {
            for point in line.iter_points() {
                *intersections.entry(point).or_insert(0) += 1;
            }
        }

        intersections
    }

    pub fn count_overlaps(map: HashMap<PointI32, u32>) -> u32 {
        map.iter().filter(|(_k, &v)| v >= 2).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::{LineCollisioner, LineParser};

    const SAMPLE_DATA: &[&str] = &[
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn test_sample() {
        let lines = LineParser::parse_lines(SAMPLE_DATA);
        let lines = LineCollisioner::filter_horizontal_and_vertical(&lines);
        let collisions = LineCollisioner::scan_line_intersections(&lines);
        assert_eq!(LineCollisioner::count_overlaps(collisions), 5);
    }

    #[test]
    fn test_sample_diagonal() {
        let lines = LineParser::parse_lines(SAMPLE_DATA);
        let collisions = LineCollisioner::scan_line_intersections(&lines);
        assert_eq!(LineCollisioner::count_overlaps(collisions), 12);
    }
}
