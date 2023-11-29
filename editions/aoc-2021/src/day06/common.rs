//! Common

pub struct FishSchool {
    index: Vec<usize>,
    buffer: Vec<usize>,
}

impl FishSchool {
    pub fn count(&self) -> usize {
        self.index.iter().sum()
    }

    pub fn step_count(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }

    pub fn step(&mut self) {
        for x in 0..9 {
            let fishes = self.index[x];
            if x == 0 {
                self.buffer[0] -= fishes;
                self.buffer[8] += fishes;
                self.buffer[6] += fishes;
            } else {
                self.buffer[x] -= fishes;
                self.buffer[x - 1] += fishes;
            }
        }

        for x in 0..9 {
            self.index[x] = self.buffer[x];
        }
    }
}

impl From<&str> for FishSchool {
    fn from(s: &str) -> Self {
        let fishes: Vec<usize> = s.trim().split(',').flat_map(|x| x.parse().ok()).collect();
        let mut index = vec![0; 9];

        for f in fishes {
            let idx = f;
            index[idx] += 1;
        }

        Self {
            buffer: index.clone(),
            index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FishSchool;

    const SAMPLE_DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_sample_18() {
        let mut school = FishSchool::from(SAMPLE_DATA);
        school.step_count(18);
        assert_eq!(school.count(), 26);
    }

    #[test]
    fn test_sample_80() {
        let mut school = FishSchool::from(SAMPLE_DATA);
        school.step_count(80);
        assert_eq!(school.count(), 5934);
    }

    #[test]
    fn test_sample_256() {
        let mut school = FishSchool::from(SAMPLE_DATA);
        school.step_count(256);
        assert_eq!(school.count(), 26_984_457_539);
    }
}
