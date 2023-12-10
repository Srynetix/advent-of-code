//! # --- Day 9: Mirage Maintenance ---
//!
//! > _Exercise page: <https://adventofcode.com/2023/day/9>_
//!
//! > _Input page: <https://adventofcode.com/2023/day/9/input>_
//!
//! You ride the camel through the sandstorm and stop where the ghost's maps told you to stop. <span title="The sound of a sandstorm slowly settling.">The sandstorm subsequently subsides, somehow seeing you standing at an <em>oasis</em>!</span>
//!
//! The camel goes to get some water and you stretch your neck. As you look up, you discover what must be yet another giant floating island, this one made of metal! That must be where the <em>parts to fix the sand machines</em> come from.
//!
//! There's even a <a target="_blank" href="https://en.wikipedia.org/wiki/Hang_gliding">hang glider</a> partially buried in the sand here; once the sun rises and heats up the sand, you might be able to use the glider and the hot air to get all the way up to the metal island!
//!
//! While you wait for the sun to rise, you admire the oasis hidden here in the middle of Desert Island. It must have a delicate ecosystem; you might as well take some ecological readings while you wait. Maybe you can report any environmental instabilities you find to someone so the oasis can be around for the next sandstorm-worn traveler.
//!
//! You pull out your handy <em>Oasis And Sand Instability Sensor</em> and analyze your surroundings. The OASIS produces a report of many values and how they are changing over time (your puzzle input). Each line in the report contains the <em>history</em> of a single value. For example:
//!
//! ```text
//! 0 3 6 9 12 15
//! 1 3 6 10 15 21
//! 10 13 16 21 30 45
//! ```
//!
//! To best protect the oasis, your environmental report should include a <em>prediction of the next value</em> in each history. To do this, start by making a new sequence from the <em>difference at each step</em> of your history. If that sequence is <em>not</em> all zeroes, repeat this process, using the sequence you just generated as the input sequence. Once all of the values in your latest sequence are zeroes, you can extrapolate what the next value of the original history should be.
//!
//! In the above dataset, the first history is <code>0 3 6 9 12 15</code>. Because the values increase by <code>3</code> each step, the first sequence of differences that you generate will be <code>3 3 3 3 3</code>. Note that this sequence has one fewer value than the input sequence because at each step it considers two numbers from the input. Since these values aren't <em>all zero</em>, repeat the process: the values differ by <code>0</code> at each step, so the next sequence is <code>0 0 0 0</code>. This means you have enough information to extrapolate the history! Visually, these sequences can be arranged like this:
//!
//! ```text
//! 0   3   6   9  12  15
//!   3   3   3   3   3
//!     0   0   0   0
//! ```
//!
//! To extrapolate, start by adding a new zero to the end of your list of zeroes; because the zeroes represent differences between the two values above them, this also means there is now a placeholder in every sequence above it:
//!
//!
//! ```text
//! 0   3   6   9  12  15   B
//!   3   3   3   3   3   A
//!     0   0   0   0   0
//! ```
//!
//! You can then start filling in placeholders from the bottom up. <code>A</code> needs to be the result of increasing <code>3</code> (the value to its left) by <code>0</code> (the value below it); this means <code>A</code> must be <code><em>3</em></code>:
//!
//! ```text
//! 0   3   6   9  12  15   B
//!   3   3   3   3   3   3
//!     0   0   0   0   0
//! ```
//!
//! Finally, you can fill in <code>B</code>, which needs to be the result of increasing <code>15</code> (the value to its left) by <code>3</code> (the value below it), or <code><em>18</em></code>:
//!
//! ```text
//! 0   3   6   9  12  15  18
//!   3   3   3   3   3   3
//!     0   0   0   0   0
//! ```
//!
//! So, the next value of the first history is <code><em>18</em></code>.
//!
//! Finding all-zero differences for the second history requires an additional sequence:
//!
//! ```text
//! 1   3   6  10  15  21
//!   2   3   4   5   6
//!     1   1   1   1
//!       0   0   0
//! ```
//!
//! Then, following the same process as before, work out the next value in each sequence from the bottom up:
//!
//! ```text
//! 1   3   6  10  15  21  28
//!   2   3   4   5   6   7
//!     1   1   1   1   1
//!       0   0   0   0
//! ```
//!
//! So, the next value of the second history is <code><em>28</em></code>.
//!
//! The third history requires even more sequences, but its next value can be found the same way:
//!
//! ```text
//! 10  13  16  21  30  45  68
//!    3   3   5   9  15  23
//!      0   2   4   6   8
//!        2   2   2   2
//!          0   0   0
//! ```
//!
//! So, the next value of the third history is <code><em>68</em></code>.
//!
//! If you find the next value for each history in this example and add them together, you get <code><em>114</em></code>.
//!
//! Analyze your OASIS report and extrapolate the next value for each history. <em>What is the sum of these extrapolated values?</em>
//!
//! # --- Part Two ---
//!
//! Of course, it would be nice to have <em>even more history</em> included in your report. Surely it's safe to just <em>extrapolate backwards</em> as well, right?
//!
//! For each history, repeat the process of finding differences until the sequence of differences is entirely zero. Then, rather than adding a zero to the end and filling in the next values of each previous sequence, you should instead add a zero to the <em>beginning</em> of your sequence of zeroes, then fill in new <em>first</em> values for each previous sequence.
//!
//! In particular, here is what the third example history looks like when extrapolating back in time:
//!
//! ```text
//! 5  10  13  16  21  30  45
//!   5   3   3   5   9  15
//!    -2   0   2   4   6
//!       2   2   2   2
//!         0   0   0
//! ```
//!
//! Adding the new values on the left side of each sequence from bottom to top eventually reveals the new left-most history value: <code><em>5</em></code>.
//!
//! Doing this for the remaining example data above results in previous values of <code><em>-3</em></code> for the first history and <code><em>0</em></code> for the second history. Adding all three new values together produces <code><em>2</em></code>.
//!
//! Analyze your OASIS report again, this time extrapolating the <em>previous</em> value for each history. <em>What is the sum of these extrapolated values?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
