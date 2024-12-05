//! # --- Day 4: Ceres Search ---
//!
//! > _Exercise page: <https://adventofcode.com/2024/day/4>_
//!
//! > _Input page: <https://adventofcode.com/2024/day/4/input>_
//!
//! "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the <a href="/2019/day/10">Ceres monitoring station</a>!
//!
//! As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her <em>word search</em> (your puzzle input). She only has to find one word: <code>XMAS</code>.
//!
//! This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of <code>XMAS</code> - you need to find <em>all of them</em>. Here are a few ways <code>XMAS</code> might appear, where irrelevant characters have been replaced with <code>.</code>:
//!
//!
//! ```text
//! ..X...
//! .SAMX.
//! .A..A.
//! XMAS.S
//! .X....
//! ```
//!
//! The actual word search will be full of letters instead. For example:
//!
//! ```text
//! MMMSXXMASM
//! MSAMXMSMSA
//! AMXSXMAAMM
//! MSAMASMSMX
//! XMASAMXAMM
//! XXAMMXXAMA
//! SMSMSASXSS
//! SAXAMASAAA
//! MAMMMXMMMM
//! MXMXAXMASX
//! ```
//!
//! In this word search, <code>XMAS</code> occurs a total of <code><em>18</em></code> times; here's the same word search again, but where letters not involved in any <code>XMAS</code> have been replaced with <code>.</code>:
//!
//! ```text
//! ....XXMAS.
//! .SAMXMS...
//! ...S..A...
//! ..A.A.MS.X
//! XMASAMX.MM
//! X.....XA.A
//! S.S.S.S.SS
//! .A.A.A.A.A
//! ..M.M.M.MM
//! .X.X.XMASX
//! ```
//!
//! Take a look at the little Elf's word search. <em>How many times does <code>XMAS</code> appear?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
