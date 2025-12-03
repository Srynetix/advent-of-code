//! # --- Day 2: Gift Shop ---
//!
//! > _Exercise page: <https://adventofcode.com/2025/day/2>_
//!
//! > _Input page: <https://adventofcode.com/2025/day/2/input>_
//!
//! You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.
//!
//! As you make your way through the <span title="They even sell lunch boxes and blue tents!">surprisingly extensive</span> selection, one of the clerks recognizes you and asks for your help.
//!
//! As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?
//!
//! They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:
//!
//! ```text
//! 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
//! 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
//! 824824821-824824827,2121212118-2121212124```
//!
//! (The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)
//!
//! The ranges are separated by commas (<code>,</code>); each range gives its <em>first ID</em> and <em>last ID</em> separated by a dash (<code>-</code>).
//!
//! Since the young Elf was just doing silly patterns, you can find the <em>invalid IDs</em> by looking for any ID which is made only of some sequence of digits repeated twice. So, <code>55</code> (<code>5</code> twice), <code>6464</code> (<code>64</code> twice), and <code>123123</code> (<code>123</code> twice) would all be invalid IDs.
//!
//! None of the numbers have leading zeroes; <code>0101</code> isn't an ID at all. (<code>101</code> is a <em>valid</em> ID that you would ignore.)
//!
//! Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:
//!
//! - 11-22 has two invalid IDs, 11 and 22.
//! - 95-115 has one invalid ID, 99.
//! - 998-1012 has one invalid ID, 1010.
//! - 1188511880-1188511890 has one invalid ID, 1188511885.
//! - 222220-222224 has one invalid ID, 222222.
//! - 1698522-1698528 contains no invalid IDs.
//! - 446443-446449 has one invalid ID, 446446.
//! - 38593856-38593862 has one invalid ID, 38593859.
//! - The rest of the ranges contain no invalid IDs.
//!
//!
//! Adding up all the invalid IDs in this example produces <code><em>1227775554</em></code>.
//!
//! <em>What do you get if you add up all of the invalid IDs?</em>
//!
//! # --- Part Two ---
//!
//! The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?
//!
//! Now, an ID is invalid if it is made only of some sequence of digits repeated <em>at least</em> twice. So, <code>12341234</code> (<code>1234</code> two times), <code>123123123</code> (<code>123</code> three times), <code>1212121212</code> (<code>12</code> five times), and <code>1111111</code> (<code>1</code> seven times) are all invalid IDs.
//!
//! From the same example as before:
//!
//! - 11-22 still has two invalid IDs, 11 and 22.
//! - 95-115 now has two invalid IDs, 99 and 111.
//! - 998-1012 now has two invalid IDs, 999 and 1010.
//! - 1188511880-1188511890 still has one invalid ID, 1188511885.
//! - 222220-222224 still has one invalid ID, 222222.
//! - 1698522-1698528 still contains no invalid IDs.
//! - 446443-446449 still has one invalid ID, 446446.
//! - 38593856-38593862 still has one invalid ID, 38593859.
//! - 565653-565659 now has one invalid ID, 565656.
//! - 824824821-824824827 now has one invalid ID, 824824824.
//! - 2121212118-2121212124 now has one invalid ID, 2121212121.
//!
//!
//! Adding up all the invalid IDs in this example produces <code><em>4174379265</em></code>.
//!
//! <em>What do you get if you add up all of the invalid IDs using these new rules?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
