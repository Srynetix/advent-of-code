//! Common

use std::{
    collections::{BTreeSet, HashMap},
    convert::Infallible,
    str::FromStr,
};

use aoc_sx::{itertools::Itertools, maplit::btreeset, once_cell::sync::Lazy};

//  0000
// 1    2
// 1    2
//  3333
// 4    5
// 4    5
//  6666

#[derive(Debug)]
pub struct Pattern(pub String);

#[derive(Debug)]
pub struct PatternLine {
    pub signal_patterns: Vec<Pattern>,
    output_patterns: Vec<Pattern>,
}

pub struct PatternCounter;

static DIGIT_MAP: Lazy<HashMap<u8, BTreeSet<u8>>> = Lazy::new(|| {
    let mut map: HashMap<u8, BTreeSet<u8>> = HashMap::new();
    map.insert(0u8, btreeset! { 0, 1, 2, 4, 5, 6 });
    map.insert(1u8, btreeset! { 2, 5 });
    map.insert(2u8, btreeset! { 0, 2, 3, 4, 6 });
    map.insert(3u8, btreeset! { 0, 2, 3, 5, 6 });
    map.insert(4u8, btreeset! { 1, 2, 3, 5 });
    map.insert(5u8, btreeset! { 0, 1, 3, 5, 6 });
    map.insert(6u8, btreeset! { 0, 1, 3, 4, 5, 6 });
    map.insert(7u8, btreeset! { 0, 2, 5 });
    map.insert(8u8, btreeset! { 0, 1, 2, 3, 4, 5, 6 });
    map.insert(9u8, btreeset! { 0, 1, 2, 3, 5, 6 });
    map
});

static INVERTED_DIGIT_MAP: Lazy<HashMap<BTreeSet<u8>, u8>> = Lazy::new(|| {
    let mut map: HashMap<BTreeSet<u8>, u8> = HashMap::new();
    for (k, v) in DIGIT_MAP.iter() {
        map.insert(v.clone(), *k);
    }
    map
});

fn try_match_digit(positions: &BTreeSet<u8>) -> Option<u8> {
    INVERTED_DIGIT_MAP.get(positions).copied()
}

pub fn generate_initial_solutions(sorted_sets: &[BTreeSet<char>]) -> Vec<HashMap<char, u8>> {
    // First is a "one", second is a "seven", third is a "four", and last is a "eight".
    let one_set = &sorted_sets[0];
    let seven_set = &sorted_sets[1];
    let four_set = &sorted_sets[2];
    let eight_set = &sorted_sets[sorted_sets.len() - 1];

    // p0 (top) is 7 - 1
    // p2 & p5 (top right + bottom right) is 1
    // p1 & p3 (top left + middle) is 4 - 1
    // p4 & p6 (bottom left + bottom) is 8 - 4 - 7
    let p0 = (seven_set - one_set).iter().copied().next().unwrap();
    let p2n5: Vec<char> = one_set.iter().copied().collect();
    let p1n3: Vec<char> = (four_set - one_set).iter().copied().collect();
    let p4n6: Vec<char> = (&(eight_set - four_set) - seven_set)
        .iter()
        .copied()
        .collect();

    // Generate each possible combinations
    let solutions = [
        vec![p0, p1n3[0], p2n5[0], p1n3[1], p4n6[0], p2n5[1], p4n6[1]],
        vec![p0, p1n3[0], p2n5[0], p1n3[1], p4n6[1], p2n5[1], p4n6[0]],
        vec![p0, p1n3[0], p2n5[1], p1n3[1], p4n6[0], p2n5[0], p4n6[1]],
        vec![p0, p1n3[0], p2n5[1], p1n3[1], p4n6[1], p2n5[0], p4n6[0]],
        vec![p0, p1n3[1], p2n5[0], p1n3[0], p4n6[0], p2n5[1], p4n6[1]],
        vec![p0, p1n3[1], p2n5[0], p1n3[0], p4n6[1], p2n5[1], p4n6[0]],
        vec![p0, p1n3[1], p2n5[1], p1n3[0], p4n6[0], p2n5[0], p4n6[1]],
        vec![p0, p1n3[1], p2n5[1], p1n3[0], p4n6[1], p2n5[0], p4n6[0]],
    ];

    solutions
        .iter()
        .map(|x| x.iter().enumerate().map(|(a, &b)| (b, a as u8)).collect())
        .collect()
}

pub fn convert_patterns_to_sets(input: &[&str]) -> Vec<BTreeSet<char>> {
    input
        .iter()
        .sorted_by(|&&x, &&y| x.len().cmp(&y.len()))
        .map(|&x| x.chars().collect())
        .collect()
}

pub fn try_solution<'a>(
    solution: &HashMap<char, u8>,
    sorted_input: &[&'a str],
) -> Option<HashMap<&'a str, u8>> {
    let mut output = HashMap::new();

    for &input in sorted_input {
        let positions: BTreeSet<_> = input
            .chars()
            .map(|x| *solution.get(&x).unwrap())
            .sorted()
            .collect();
        match try_match_digit(&positions) {
            Some(k) => {
                output.insert(input, k);
            }
            None => return None,
        }
    }

    Some(output)
}

pub fn find_solution<'a>(input: &[&'a str]) -> HashMap<&'a str, u8> {
    let sets = convert_patterns_to_sets(input);
    let solutions = generate_initial_solutions(&sets);

    for solution in solutions {
        match try_solution(&solution, input) {
            Some(s) => return s,
            None => continue,
        }
    }

    panic!("Could not find solution");
}

impl PatternCounter {
    pub fn count_unambiguous_output_patterns(lines: &[PatternLine]) -> usize {
        lines
            .iter()
            .map(|x| x.count_unambiguous_output_patterns())
            .sum()
    }
}

impl Pattern {
    fn guess_digit(&self) -> Option<u8> {
        match self.0.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }
}

impl PatternLine {
    pub fn count_unambiguous_output_patterns(&self) -> usize {
        self.output_patterns
            .iter()
            .flat_map(|x| x.guess_digit())
            .count()
    }

    pub fn decode_output_with_mapping(&self, mapping: &HashMap<&str, u8>) -> String {
        self.output_patterns
            .iter()
            .map(|x| mapping.get(&x.0[..]).unwrap())
            .join("")
    }
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Self {
        Self(s.chars().sorted().collect())
    }
}

impl From<&str> for PatternLine {
    fn from(s: &str) -> Self {
        let mut spl = s.split(" | ");
        let signals: Vec<Pattern> = spl
            .next()
            .expect("Could not extract signals from input")
            .split_whitespace()
            .map(Into::into)
            .collect();
        let output: Vec<Pattern> = spl
            .next()
            .expect("Could not extract output from input")
            .split_whitespace()
            .map(Into::into)
            .collect();

        Self {
            signal_patterns: signals,
            output_patterns: output,
        }
    }
}

impl FromStr for PatternLine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::maplit::hashmap;

    use super::{find_solution, PatternCounter, PatternLine};

    const SAMPLE_DATA: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const LARGER_SAMPLE_DATA: &[&str] = &[
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn test_sample() {
        let lines = PatternLine::from(SAMPLE_DATA);
        assert_eq!(
            PatternCounter::count_unambiguous_output_patterns(&[lines]),
            0
        );
    }

    #[test]
    fn test_larger_sample() {
        let lines: Vec<PatternLine> = LARGER_SAMPLE_DATA.iter().map(|&x| x.into()).collect();
        assert_eq!(
            PatternCounter::count_unambiguous_output_patterns(&lines),
            26
        );
    }

    #[test]
    fn test_find_solution_sample() {
        let line = PatternLine::from(SAMPLE_DATA);
        let input = line
            .signal_patterns
            .iter()
            .map(|x| &x.0[..])
            .collect::<Vec<_>>();
        let mapping = find_solution(&input);
        assert_eq!(
            mapping,
            hashmap! {
                "abcdeg" => 0,
                "ab" => 1,
                "acdfg" => 2,
                "abcdf" => 3,
                "abef" => 4,
                "bcdef" => 5,
                "bcdefg" => 6,
                "abd" => 7,
                "abcdefg" => 8,
                "abcdef" => 9,
            }
        )
    }

    #[test]
    fn test_find_solution_large() {
        let lines: Vec<PatternLine> = LARGER_SAMPLE_DATA.iter().map(|&x| x.into()).collect();
        let output: Vec<String> = lines
            .iter()
            .map(|line| {
                let input: Vec<_> = line.signal_patterns.iter().map(|x| &x.0[..]).collect();
                let mapping = find_solution(&input);
                line.decode_output_with_mapping(&mapping)
            })
            .collect();

        assert_eq!(
            output,
            ["8394", "9781", "1197", "9361", "4873", "8418", "4548", "1625", "8717", "4315",]
        );
    }
}
