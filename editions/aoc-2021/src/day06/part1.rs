//! Part 1

use super::{common::FishSchool, INPUT};

pub fn run() -> usize {
    let mut school = FishSchool::from(INPUT);
    school.step_count(80);
    school.count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 396_210)
    }
}
