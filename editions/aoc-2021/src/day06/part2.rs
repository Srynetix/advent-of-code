//! Part 2

use super::{INPUT, common::FishSchool};

pub fn run() -> usize {
    let mut school = FishSchool::from(INPUT);
    school.step_count(256);
    school.count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1_770_823_541_496)
    }
}
