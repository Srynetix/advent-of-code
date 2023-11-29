//! Common

pub struct CrabSwarm {
    positions: Vec<u32>,
}

impl CrabSwarm {
    pub fn fuel_cost_for_position(&self, pos: u32) -> u32 {
        let mut sum: i32 = 0;

        for &p in &self.positions {
            let diff = if p > pos {
                p as i32 - pos as i32
            } else {
                pos as i32 - p as i32
            };

            sum += diff;
        }

        sum as u32
    }

    pub fn fuel_cost_for_position_with_sum(&self, pos: u32) -> u32 {
        let mut sum: i32 = 0;

        for &p in &self.positions {
            let diff = if p > pos {
                p as i32 - pos as i32
            } else {
                pos as i32 - p as i32
            };

            let local_sum = Self::sum(diff as usize, 1, diff) as i32;
            sum += local_sum;
        }

        sum as u32
    }

    pub fn sum(n: usize, min: i32, max: i32) -> u32 {
        ((n as f32 * (min + max) as f32) / 2.0) as u32
    }

    pub fn min_cost_for_alignment(&self) -> (u32, u32) {
        let mut min_pos = u32::MAX;
        let mut min_sum = u32::MAX;
        let max_p = *self
            .positions
            .iter()
            .max()
            .expect("Should have more than one value");
        for i in 0..max_p {
            let sum = self.fuel_cost_for_position(i);
            if sum < min_sum {
                min_sum = sum;
                min_pos = i;
            }
        }

        (min_pos, min_sum)
    }

    pub fn min_cost_for_alignment_with_sum(&self) -> (u32, u32) {
        let mut min_pos = u32::MAX;
        let mut min_sum = u32::MAX;
        let max_p = *self
            .positions
            .iter()
            .max()
            .expect("Should have more than one value");
        for i in 0..max_p {
            let sum = self.fuel_cost_for_position_with_sum(i);
            if sum < min_sum {
                min_sum = sum;
                min_pos = i;
            }
        }

        (min_pos, min_sum)
    }
}

impl From<&str> for CrabSwarm {
    fn from(s: &str) -> Self {
        let positions = s.trim().split(',').flat_map(|x| x.parse().ok()).collect();

        Self { positions }
    }
}

#[cfg(test)]
mod tests {
    use super::CrabSwarm;

    const SAMPLE_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_sample() {
        let swarm = CrabSwarm::from(SAMPLE_DATA);
        assert_eq!(swarm.min_cost_for_alignment(), (2, 37));
    }

    #[test]
    fn test_sample_sum() {
        let swarm = CrabSwarm::from(SAMPLE_DATA);
        assert_eq!(swarm.min_cost_for_alignment_with_sum(), (5, 168));
    }
}
