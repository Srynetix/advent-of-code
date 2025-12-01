//! # --- Day 1: Secret Entrance ---
//!
//! > _Exercise page: <https://adventofcode.com/2025/day/1>_
//!
//! > _Input page: <https://adventofcode.com/2025/day/1/input>_
//!
//! The Elves have good news and bad news.
//!
//! The good news is that they've discovered <a href="https://en.wikipedia.org/wiki/Project_management" target="_blank">project management</a>! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.
//!
//! The bad news is that they've realized they have a <em>different</em> emergency: according to their resource planning, none of them have any time left to decorate the North Pole!
//!
//! To save Christmas, the Elves need <em>you</em> to <em>finish decorating the North Pole by December 12th</em>.
//!
//! Collect stars by solving puzzles.  Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first.  Each puzzle grants <em class="star">one star</em>. Good luck!
//!
//! You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the <em>password</em> seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:
//!
//! "Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."
//!
//! The safe has a dial with only an arrow on it; around the dial are the numbers <code>0</code> through <code>99</code> in order. As you turn the dial, it makes a small <em>click</em> noise as it reaches each number.
//!
//! The attached document (your puzzle input) contains a sequence of <em>rotations</em>, one per line, which tell you how to open the safe. A rotation starts with an <code>L</code> or <code>R</code> which indicates whether the rotation should be to the <em>left</em> (toward lower numbers) or to the <em>right</em> (toward higher numbers). Then, the rotation has a <em>distance</em> value which indicates how many clicks the dial should be rotated in that direction.
//!
//! So, if the dial were pointing at <code>11</code>, a rotation of <code>R8</code> would cause the dial to point at <code>19</code>. After that, a rotation of <code>L19</code> would cause it to point at <code>0</code>.
//!
//! Because the dial is a circle, turning the dial <em>left from <code>0</code></em> one click makes it point at <code>99</code>. Similarly, turning the dial <em>right from <code>99</code></em> one click makes it point at <code>0</code>.
//!
//! So, if the dial were pointing at <code>5</code>, a rotation of <code>L10</code> would cause it to point at <code>95</code>. After that, a rotation of <code>R5</code> could cause it to point at <code>0</code>.
//!
//! The dial starts by pointing at <code>50</code>.
//!
//! You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is <em>the number of times the dial is left pointing at <code>0</code> after any rotation in the sequence</em>.
//!
//! For example, suppose the attached document contained the following rotations:
//!
//! ```text
//! L68
//! L30
//! R48
//! L5
//! R60
//! L55
//! L1
//! L99
//! R14
//! L82
//! ```
//!
//! Following these rotations would cause the dial to move as follows:
//!
//! - The dial starts by pointing at 50.
//! - The dial is rotated L68 to point at 82.
//! - The dial is rotated L30 to point at 52.
//! - The dial is rotated R48 to point at 0.
//! - The dial is rotated L5 to point at 95.
//! - The dial is rotated R60 to point at 55.
//! - The dial is rotated L55 to point at 0.
//! - The dial is rotated L1 to point at 99.
//! - The dial is rotated L99 to point at 0.
//! - The dial is rotated R14 to point at 14.
//! - The dial is rotated L82 to point at 32.
//!
//!
//! Because the dial points at <code>0</code> a total of three times during this process, the password in this example is <code><em>3</em></code>.
//!
//! Analyze the rotations in your attached document. <em>What's the actual password to open the door?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
