//! Common

pub fn count_increments(input: &[u32]) -> usize {
    input
        .windows(2)
        .flat_map(<&[u32; 2]>::try_from)
        .fold(0, |acc, [a, b]| acc + (b > a) as usize)
}

pub fn count_increments_three(input: &[u32]) -> usize {
    let input: Vec<_> = input.windows(3).map(|slice| slice.iter().sum()).collect();
    count_increments(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_count_increments_sample() {
        assert_eq!(count_increments(&SAMPLE_DATA), 7);
    }

    #[test]
    fn test_count_increments_three_sample() {
        assert_eq!(count_increments_three(&SAMPLE_DATA), 5);
    }
}
