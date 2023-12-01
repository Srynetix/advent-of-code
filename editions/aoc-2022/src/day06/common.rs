//! Common

use std::collections::HashSet;

use aoc_sx::itertools::Itertools;

#[derive(Debug)]
pub struct DataStream<'a>(&'a str);

impl<'a> DataStream<'a> {
    pub fn from_input(input: &'a str) -> Self {
        Self(input)
    }

    pub fn start_of_packet_marker(&self, window_size: usize) -> usize {
        for (index, slice) in self
            .0
            .chars()
            .collect_vec()
            .as_slice()
            .windows(window_size)
            .enumerate()
        {
            if self.slice_contains_all_different_chars(slice) {
                return index + slice.len();
            }
        }

        panic!("Marker not found")
    }

    fn slice_contains_all_different_chars(&self, chars: &[char]) -> bool {
        let mut seen = HashSet::new();
        for c in chars {
            if seen.contains(c) {
                return false;
            }

            seen.insert(c);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::DataStream;

    #[test]
    fn start_of_packet_marker_4() {
        assert_eq!(
            DataStream("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_packet_marker(4),
            7
        );
        assert_eq!(
            DataStream("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_packet_marker(4),
            5
        );
        assert_eq!(
            DataStream("nppdvjthqldpwncqszvftbrmjlhg").start_of_packet_marker(4),
            6
        );
        assert_eq!(
            DataStream("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_packet_marker(4),
            10
        );
        assert_eq!(
            DataStream("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_packet_marker(4),
            11
        );
    }

    #[test]
    fn start_of_packet_marker_14() {
        assert_eq!(
            DataStream("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_packet_marker(14),
            19
        );
        assert_eq!(
            DataStream("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_packet_marker(14),
            23
        );
        assert_eq!(
            DataStream("nppdvjthqldpwncqszvftbrmjlhg").start_of_packet_marker(14),
            23
        );
        assert_eq!(
            DataStream("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_packet_marker(14),
            29
        );
        assert_eq!(
            DataStream("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_packet_marker(14),
            26
        );
    }
}
