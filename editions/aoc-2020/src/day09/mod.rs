//! # --- Day 9: Encoding Error ---
//!
//! > _Exercise page: <https://adventofcode.com/2020/day/9>_
//!
//! > _Input page: <https://adventofcode.com/2020/day/9/input>_
//!
//! With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.
//!
//! Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).
//!
//! The data appears to be encrypted with the eXchange-Masking Addition System (<span title="No relation.">XMAS</span>) which, conveniently for you, is an old cypher with an important weakness.
//!
//! XMAS starts by transmitting a <em>preamble</em> of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one such pair.
//!
//! For example, suppose your preamble consists of the numbers <code>1</code> through <code>25</code> in a random order. To be valid, the next number must be the sum of two of those numbers:
//!
//! - 26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
//! - 49 would be a valid next number, as it is the sum of 24 and 25.
//! - 100 would not be valid; no two of the previous 25 numbers sum to 100.
//! - 50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.
//!
//!
//! Suppose the 26th number is <code>45</code>, and the first number (no longer an option, as it is more than 25 numbers ago) was <code>20</code>. Now, for the next number to be valid, there needs to be some pair of numbers among <code>1</code>-<code>19</code>, <code>21</code>-<code>25</code>, or <code>45</code> that add up to it:
//!
//! - 26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
//! - 65 would not be valid, as no two of the available numbers sum to it.
//! - 64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
//!
//!
//! Here is a larger example which only considers the previous <em>5</em> numbers (and has a preamble of length 5):
//!
//! ```text
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only number that does not follow this rule is <em><code>127</code></em>.
//!
//! The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is <em>not</em> the sum of two of the 25 numbers before it. <em>What is the first number that does not have this property?</em>
//!
//! # --- Part Two ---
//!
//! The final step in breaking the XMAS encryption relies on the invalid number you just found: you must <em>find a contiguous set of at least two numbers</em> in your list which sum to the invalid number from step 1.
//!
//! Again consider the above example:
//!
//! ```text
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this list, adding up all of the numbers from <code>15</code> through <code>40</code> produces the invalid number from step 1, <code>127</code>. (Of course, the contiguous set of numbers in your actual list might be much longer.)
//!
//! To find the <em>encryption weakness</em>, add together the <em>smallest</em> and <em>largest</em> number in this contiguous range; in this example, these are <code>15</code> and <code>47</code>, producing <em><code>62</code></em>.
//!
//! <em>What is the encryption weakness in your XMAS-encrypted list of numbers?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
