//! Common

pub struct BitAnalyzer;

pub enum RatingComputationMethod {
    LeastCommon,
    MostCommon,
}

impl BitAnalyzer {
    fn compute_binary_sum(data: &[&str]) -> Vec<u32> {
        let line_length = data[0].len();

        data.iter()
            .map(|&l| l.chars().flat_map(|n| n.to_digit(10)).collect::<Vec<_>>())
            .fold(vec![0; line_length], |mut acc, l| {
                for (idx, x) in l.iter().enumerate() {
                    acc[idx] += x;
                }

                acc
            })
    }

    pub fn compute_gamma_and_epsilon(data: &[&str]) -> (u32, u32) {
        let half_length = data.len() / 2;
        let sum = Self::compute_binary_sum(data);

        let temp: Vec<bool> = sum.iter().map(|&n| ((n as usize) > half_length)).collect();
        let epsilon: String = temp.iter().map(|&n| (!n as u32).to_string()).collect();
        let gamma: String = temp.iter().map(|&n| (n as u32).to_string()).collect();

        (
            u32::from_str_radix(&gamma, 2).expect("Bad gamma binary value"),
            u32::from_str_radix(&epsilon, 2).expect("Bad epsilon binary value"),
        )
    }

    pub fn compute_oxygen_generator_rating(data: &[&str]) -> u32 {
        Self::compute_rating(RatingComputationMethod::MostCommon, data)
    }

    pub fn compute_co2_scrubber_rating(data: &[&str]) -> u32 {
        Self::compute_rating(RatingComputationMethod::LeastCommon, data)
    }

    fn compute_rating(method: RatingComputationMethod, data: &[&str]) -> u32 {
        let line_length = data[0].len();
        let mut remaining = data.to_owned();
        let mut result = 0;

        for index in 0..line_length {
            let sum_vec = Self::compute_binary_sum(&remaining);
            let length = remaining.len();
            let bit = match method {
                RatingComputationMethod::LeastCommon => {
                    Self::find_least_common_bit(index, length, &sum_vec)
                }
                RatingComputationMethod::MostCommon => {
                    Self::find_most_common_bit(index, length, &sum_vec)
                }
            };
            remaining = Self::filter_values_with_bit(&remaining, index, bit);

            if remaining.len() == 1 {
                result = u32::from_str_radix(remaining[0], 2).expect("Should be a valid binary");
                break;
            }
        }

        result
    }

    fn find_most_common_bit(index: usize, length: usize, sum_vec: &[u32]) -> u8 {
        let ones = sum_vec[index];
        let zeroes = length as u32 - ones;
        (ones >= zeroes) as u8
    }

    fn find_least_common_bit(index: usize, length: usize, sum_vec: &[u32]) -> u8 {
        let ones = sum_vec[index];
        let zeroes = length as u32 - ones;
        (ones < zeroes) as u8
    }

    fn filter_values_with_bit<'a>(values: &[&'a str], index: usize, bit: u8) -> Vec<&'a str> {
        values
            .iter()
            .filter(|&&l| {
                let c = l.chars().nth(index).expect("Index out of bounds");
                c.to_digit(2).expect("Invalid char") as u8 == bit
            })
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::BitAnalyzer;

    const SAMPLE_DATA: &[&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_compute_binary_sum() {
        assert_eq!(
            BitAnalyzer::compute_binary_sum(SAMPLE_DATA),
            vec![7, 5, 8, 7, 5]
        );
    }

    #[test]
    fn test_compute_gamma_and_epsilon() {
        assert_eq!(BitAnalyzer::compute_gamma_and_epsilon(SAMPLE_DATA), (22, 9));
    }

    #[test]
    fn test_compute_oxygen_generator_rating() {
        assert_eq!(
            BitAnalyzer::compute_oxygen_generator_rating(SAMPLE_DATA),
            23
        );
    }

    #[test]
    fn test_compute_co2_scrubber_rating() {
        assert_eq!(BitAnalyzer::compute_co2_scrubber_rating(SAMPLE_DATA), 10);
    }
}
