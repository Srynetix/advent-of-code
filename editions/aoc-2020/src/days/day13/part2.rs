//! Part 2

use super::{common::{extract_schedules, Scheduler}, INPUT};

pub fn run() -> usize {
    let (_, schedule) = extract_schedules(INPUT);
    Scheduler::from_input(schedule).compute_successive_departures_time()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 305_068_317_272_992);
    }
}
