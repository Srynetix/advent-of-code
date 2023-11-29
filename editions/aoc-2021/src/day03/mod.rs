//! # --- Day 3: Binary Diagnostic ---
//!
//! > _Exercise page: <https://adventofcode.com/2021/day/3>_
//!
//! > _Input page: <https://adventofcode.com/2021/day/3/input>_
//!
//! The submarine has been making some <span title="Turns out oceans are heavy.">odd creaking noises</span>, so you ask it to produce a diagnostic report just in case.
//!
//! The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell you many useful things about the conditions of the submarine. The first parameter to check is the <em>power consumption</em>.
//!
//! You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the <em>gamma rate</em> and the <em>epsilon rate</em>). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.
//!
//! Each bit in the gamma rate can be determined by finding the <em>most common bit in the corresponding position</em> of all numbers in the diagnostic report. For example, given the following diagnostic report:
//!
//! ```text
//! 00100
//! 11110
//! 10110
//! 10111
//! 10101
//! 01111
//! 00111
//! 11100
//! 10000
//! 11001
//! 00010
//! 01010
//! ```
//!
//! Considering only the first bit of each number, there are five <code>0</code> bits and seven <code>1</code> bits. Since the most common bit is <code>1</code>, the first bit of the gamma rate is <code>1</code>.
//!
//! The most common second bit of the numbers in the diagnostic report is <code>0</code>, so the second bit of the gamma rate is <code>0</code>.
//!
//! The most common value of the third, fourth, and fifth bits are <code>1</code>, <code>1</code>, and <code>0</code>, respectively, and so the final three bits of the gamma rate are <code>110</code>.
//!
//! So, the gamma rate is the binary number <code>10110</code>, or <code><em>22</em></code> in decimal.
//!
//! The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is <code>01001</code>, or <code><em>9</em></code> in decimal. Multiplying the gamma rate (<code>22</code>) by the epsilon rate (<code>9</code>) produces the power consumption, <code><em>198</em></code>.
//!
//! Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. <em>What is the power consumption of the submarine?</em> (Be sure to represent your answer in decimal, not binary.)
//!
//! # --- Part Two ---
//!
//! Next, you should verify the <em>life support rating</em>, which can be determined by multiplying the <em>oxygen generator rating</em> by the <em>CO2 scrubber rating</em>.
//!
//! Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and <em>consider just the first bit</em> of those numbers. Then:
//!
//! - Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
//! - If you only have one number left, stop; this is the rating value for which you are searching.
//! - Otherwise, repeat the process, considering the next bit to the right.
//!
//!
//! The <em>bit criteria</em> depends on which type of rating value you want to find:
//!
//! - To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
//! - To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.
//!
//!
//! For example, to determine the <em>oxygen generator rating</em> value using the same example diagnostic report from above:
//!
//! - Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
//! - Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
//! - In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
//! - In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
//! - In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
//! - As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.
//!
//!
//! Then, to determine the <em>CO2 scrubber rating</em> value from the same example above:
//!
//! - Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
//! - Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
//! - In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
//! - As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
//!
//!
//! Finally, to find the life support rating, multiply the oxygen generator rating (<code>23</code>) by the CO2 scrubber rating (<code>10</code>) to get <code><em>230</em></code>.
//!
//! Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. <em>What is the life support rating of the submarine?</em> (Be sure to represent your answer in decimal, not binary.)

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
