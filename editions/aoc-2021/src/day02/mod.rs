//! # --- Day 2: Dive! ---
//!
//! > _Exercise page: <https://adventofcode.com/2021/day/2>_
//!
//! > _Input page: <https://adventofcode.com/2021/day/2/input>_
//!
//! Now, you need to figure out how to <span title="Tank, I need a pilot program for a B212 helicopter.">pilot this thing</span>.
//!
//! It seems like the submarine can take a series of commands like <code>forward 1</code>, <code>down 2</code>, or <code>up 3</code>:
//!
//! - forward X increases the horizontal position by X units.
//! - down X increases the depth by X units.
//! - up X decreases the depth by X units.
//!
//!
//! Note that since you're on a submarine, <code>down</code> and <code>up</code> affect your <em>depth</em>, and so they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
//!
//! ```text
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//!
//! Your horizontal position and depth both start at <code>0</code>. The steps above would then modify them as follows:
//!
//! - forward 5 adds 5 to your horizontal position, a total of 5.
//! - down 5 adds 5 to your depth, resulting in a value of 5.
//! - forward 8 adds 8 to your horizontal position, a total of 13.
//! - up 3 decreases your depth by 3, resulting in a value of 2.
//! - down 8 adds 8 to your depth, resulting in a value of 10.
//! - forward 2 adds 2 to your horizontal position, a total of 15.
//!
//!
//! After following these instructions, you would have a horizontal position of <code>15</code> and a depth of <code>10</code>. (Multiplying these together produces <code><em>150</em></code>.)
//!
//! Calculate the horizontal position and depth you would have after following the planned course. <em>What do you get if you multiply your final horizontal position by your final depth?</em>
//!
//! # --- Part Two ---
//!
//! Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a third value, <em>aim</em>, which also starts at <code>0</code>. The commands also mean something entirely different than you first thought:
//!
//! - down X increases your aim by X units.
//! - up X decreases your aim by X units.
//! - forward X does two things:
//!   It increases your horizontal position by X units.
//!   It increases your depth by your aim multiplied by X.
//!
//!
//!
//! Again note that since you're on a submarine, <code>down</code> and <code>up</code> do the opposite of what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! - forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
//! - down 5 adds 5 to your aim, resulting in a value of 5.
//! - forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
//! - up 3 decreases your aim by 3, resulting in a value of 2.
//! - down 8 adds 8 to your aim, resulting in a value of 10.
//! - forward 2 adds 2 to your horizontal position, a total of 15.  Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
//!
//!
//! After following these new instructions, you would have a horizontal position of <code>15</code> and a depth of <code>60</code>. (Multiplying these produces <code><em>900</em></code>.)
//!
//! Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. <em>What do you get if you multiply your final horizontal position by your final depth?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
