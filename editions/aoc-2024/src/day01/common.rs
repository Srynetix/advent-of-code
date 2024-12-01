//! Common

pub struct Sorter;

impl Sorter {
    pub fn from_input(input: &str) -> (Vec<u32>, Vec<u32>) {
        let mut v1 = vec![];
        let mut v2 = vec![];

        for line in input.lines() {
            let mut numbers = line.split_whitespace();
            let num1 = numbers.next().and_then(|n| n.parse::<u32>().ok()).unwrap();
            let num2 = numbers.next().and_then(|n| n.parse::<u32>().ok()).unwrap();
            v1.push(num1);
            v2.push(num2);
        }

        (v1, v2)
    }

    pub fn sort_lists(v1: &mut [u32], v2: &mut [u32]) {
        v1.sort();
        v2.sort();
    }

    pub fn distances(v1: &[u32], v2: &[u32]) -> Vec<u32> {
        let mut res = vec![];
        for (e1, e2) in v1.iter().zip(v2.iter()) {
            let i1 = *e1 as i32;
            let i2 = *e2 as i32;
            res.push((i1 - i2).unsigned_abs());
        }
        res
    }

    pub fn sum_distances(v: Vec<u32>) -> u32 {
        v.into_iter().sum()
    }

    pub fn similarity_score(value: u32, v: &[u32]) -> u32 {
        v.iter().filter(|v| **v == value).count() as u32
    }

    pub fn total_similarity_score(v1: &[u32], v2: &[u32]) -> u32 {
        v1.iter()
            .map(|v| Self::similarity_score(*v, v2) * *v)
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::Sorter;

    const SAMPLE: &str = indoc! {r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#};

    #[test]
    fn sample() {
        let (mut v1, mut v2) = Sorter::from_input(SAMPLE);
        Sorter::sort_lists(&mut v1, &mut v2);

        let d = Sorter::distances(&v1, &v2);
        assert_eq!(d, &[2, 1, 0, 1, 2, 5]);
        assert_eq!(Sorter::sum_distances(d), 11);
    }

    #[test]
    fn sample_2() {
        let (v1, v2) = Sorter::from_input(SAMPLE);
        assert_eq!(Sorter::total_similarity_score(&v1, &v2), 31);
    }
}
