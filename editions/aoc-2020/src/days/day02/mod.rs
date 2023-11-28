//! # --- Day 2: Password Philosophy ---
//!
//! > _Exercise page: <https://adventofcode.com/2020/day/2>_  
//! > _Input page: <https://adventofcode.com/2020/day/2/input>_
//!
//! Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via <a href="https://en.wikipedia.org/wiki/Toboggan" target="_blank">toboggan</a>.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the <span title="To ensure your safety, your password must be the following string...">Official Toboggan Corporate Policy</span> that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input) of <em>passwords</em> (according to the corrupted database) and <em>the corporate policy when that password was set</em>.
//!
//! For example, suppose you have the following list:
//!
//! ```
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//!
//! Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, <code>1-3 a</code> means that the password must contain <code>a</code> at least <code>1</code> time and at most <code>3</code> times.
//!
//! In the above example, <code><em>2</em></code> passwords are valid. The middle password, <code>cdefg</code>, is not; it contains no instances of <code>b</code>, but needs at least <code>1</code>. The first and third passwords are valid: they contain one <code>a</code> or nine <code>c</code>, both within the limits of their respective policies.
//!
//! <em>How many passwords are valid</em> according to their policies?
//!
//! # --- Part Two ---
//!
//! While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
//!
//! Each policy actually describes two <em>positions in the password</em>, where <code>1</code> means the first character, <code>2</code> means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) <em>Exactly one of these positions</em> must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//! - 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//! - 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//! - 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//!
//!
//! <em>How many passwords are valid</em> according to the new interpretation of the policies?

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
