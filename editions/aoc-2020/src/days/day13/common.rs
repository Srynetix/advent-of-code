//! Common

/// Extract schedules from input string.
///
/// # Arguments
///
/// * `input` - Input string
pub fn extract_schedules(input: &str) -> (usize, &str) {
    let mut lines = input.lines();
    (
        lines.next().and_then(|x| x.parse::<usize>().ok()).unwrap(),
        lines.next().unwrap(),
    )
}

/// Bus scheduler
pub struct Scheduler {
    data: Vec<Option<usize>>,
}

impl Scheduler {
    /// Creates new scheduler from input.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn from_input(input: &str) -> Self {
        Self {
            data: input.trim().split(',').map(|x| x.parse().ok()).collect(),
        }
    }

    /// Creates new scheduler from vec.
    ///
    /// # Arguments
    ///
    /// * `data` - Vec
    pub fn from_vec(data: Vec<Option<usize>>) -> Self {
        Self { data }
    }

    /// Scan bus for target time.
    /// Returns a `(wait_time, bus_id)` tuple.
    ///
    /// # Arguments
    ///
    /// * `target` - Target time
    pub fn scan_buses_for_target_time(&self, target: usize) -> (usize, usize) {
        self.data
            .iter()
            .filter_map(|b| b.and_then(|b| Some((b - target.rem_euclid(b), b))))
            .min()
            .unwrap()
    }

    /// Compute successive departures time.
    pub fn compute_successive_departures_time(&self) -> usize {
        let nums = self.get_bus_numbers();
        let offsets = self.get_offsets();

        let mut t = 0;
        let mut last_lcm = 1;
        let mut found = false;

        while !found {
            found = true;

            for idx in 1..nums.len() {
                let n_a = nums[idx - 1];
                let n_b = nums[idx];
                let offset_a = offsets[idx - 1];
                let offset_b = offsets[idx];

                let (nt, _, _) = Self::calculate_x(n_a, n_b, offset_a, offset_b, t, last_lcm);

                t = nt;
                last_lcm = nums[0..idx].iter().product();

                if !Self::validate_t(&nums[0..idx], &offsets[0..idx], t) {
                    // Try again
                    found = false;
                    break;
                }
            }
        }

        t
    }

    const fn calculate_x(
        n_a: usize,
        n_b: usize,
        offset_a: usize,
        offset_b: usize,
        t_initial: usize,
        t_increment: usize,
    ) -> (usize, usize, usize) {
        let mut t = t_initial;
        let mut x;
        let mut y;

        loop {
            x = (t + offset_a) / n_a;
            y = (t + offset_b) / n_b;

            // Try to validate equations
            if y * n_b == t + offset_b && x * n_a == t + offset_a {
                return (t, x, y);
            }

            t += t_increment;
        }
    }

    fn validate_t(n: &[usize], a: &[usize], t: usize) -> bool {
        for i in 0..n.len() {
            if (t + a[i]) % n[i] != 0 {
                return false;
            }
        }

        true
    }

    fn get_offsets(&self) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, n)| n.map(|_| idx))
            .collect()
    }

    fn get_bus_numbers(&self) -> Vec<usize> {
        self.data.iter().filter_map(|n| *n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"939
    7,13,x,x,59,x,31,19"#;

    #[test]
    fn test_scheduler_parse() {
        let (_, schedule_line) = extract_schedules(SAMPLE);
        let scheduler = Scheduler::from_input(schedule_line);
        assert_eq!(scheduler.get_bus_numbers(), vec![7, 13, 59, 31, 19]);
    }

    #[test]
    fn test_scan_buses_for_target_time() {
        let (target, schedule_line) = extract_schedules(SAMPLE);
        let scheduler = Scheduler::from_input(schedule_line);
        let result = scheduler.scan_buses_for_target_time(target);

        assert_eq!(result, (5, 59));
    }

    #[test]
    fn test_compute_successive_departures_time() {
        assert_eq!(
            Scheduler::from_vec(vec![Some(17), None, Some(13), Some(19)])
                .compute_successive_departures_time(),
            3417
        );
        assert_eq!(
            Scheduler::from_vec(vec![Some(67), Some(7), Some(59), Some(61)])
                .compute_successive_departures_time(),
            754_018
        );
        assert_eq!(
            Scheduler::from_vec(vec![Some(67), None, Some(7), Some(59), Some(61)])
                .compute_successive_departures_time(),
            779_210
        );
        assert_eq!(
            Scheduler::from_vec(vec![Some(67), Some(7), None, Some(59), Some(61)])
                .compute_successive_departures_time(),
            1_261_476
        );
        assert_eq!(
            Scheduler::from_vec(vec![Some(1789), Some(37), Some(47), Some(1889)])
                .compute_successive_departures_time(),
            1_202_161_486
        );
    }
}
