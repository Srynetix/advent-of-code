//! Part 1

use super::{
    INPUT,
    common::{Scheduler, extract_schedules},
};

pub fn run() -> usize {
    let (target, schedule) = extract_schedules(INPUT);
    let scheduler = Scheduler::from_input(schedule);
    let (wait_time, bus_id) = scheduler.scan_buses_for_target_time(target);

    wait_time * bus_id
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 136);
    }
}
