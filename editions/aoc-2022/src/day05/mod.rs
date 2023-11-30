//! # --- Day 5: Supply Stacks ---
//!
//! > _Exercise page: <https://adventofcode.com/2022/day/5>_
//!
//! > _Input page: <https://adventofcode.com/2022/day/5/input>_
//!
//! The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked <em>crates</em>, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
//!
//! The ship has a <em>giant cargo crane</em> capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her <em>which</em> crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates <em>and</em> the rearrangement procedure (your puzzle input). For example:
//!
//! ```text
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//! ```
//!
//! In this example, there are three stacks of crates. Stack 1 contains two crates: crate <code>Z</code> is on the bottom, and crate <code>N</code> is on top. Stack 2 contains three crates; from bottom to top, they are crates <code>M</code>, <code>C</code>, and <code>D</code>. Finally, stack 3 contains a single crate, <code>P</code>.
//!
//! Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//!
//! ```text
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//! ```
//!
//! In the second step, three crates are moved from stack 1 to stack 3. Crates are moved <em>one at a time</em>, so the first crate to be moved (<code>D</code>) ends up below the second and third crates:
//!
//! ```text
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//! ```
//!
//! Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved <em>one at a time</em>, crate <code>C</code> ends up below crate <code>M</code>:
//!
//! ```text
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//! ```
//!
//! Finally, one crate is moved from stack 1 to stack 2:
//!
//! ```text
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//! ```
//!
//! The Elves just need to know <em>which crate will end up on top of each stack</em>; in this example, the top crates are <code>C</code> in stack 1, <code>M</code> in stack 2, and <code>Z</code> in stack 3, so you should combine these together and give the Elves the message <code><em>CMZ</em></code>.
//!
//! <em>After the rearrangement procedure completes, what crate ends up on top of each stack?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
