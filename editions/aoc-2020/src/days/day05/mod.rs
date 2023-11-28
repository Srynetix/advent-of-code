//! # --- Day 5: Binary Boarding ---
//!
//! > _Exercise page: <https://adventofcode.com/2020/day/5>_
//! > _Input page: <https://adventofcode.com/2020/day/5/input>_
//!
//! You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.
//!
//! You write a <span title="No problem!">quick program</span> to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.
//!
//! Instead of <a href="https://www.youtube.com/watch?v=oAHbLRjF0vo" target="_blank">zones or groups</a>, this airline uses <em>binary space partitioning</em> to seat people. A seat might be specified like <code>FBFBBFFRLR</code>, where <code>F</code> means "front", <code>B</code> means "back", <code>L</code> means "left", and <code>R</code> means "right".
//!
//! The first 7 characters will either be <code>F</code> or <code>B</code>; these specify exactly one of the <em>128 rows</em> on the plane (numbered <code>0</code> through <code>127</code>). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the <em>front</em> (<code>0</code> through <code>63</code>) or the <em>back</em> (<code>64</code> through <code>127</code>). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.
//!
//! For example, consider just the first seven characters of <code>FBFBBFFRLR</code>:
//!
//! - Start by considering the whole range, rows 0 through 127.
//! - F means to take the lower half, keeping rows 0 through 63.
//! - B means to take the upper half, keeping rows 32 through 63.
//! - F means to take the lower half, keeping rows 32 through 47.
//! - B means to take the upper half, keeping rows 40 through 47.
//! - B keeps rows 44 through 47.
//! - F keeps rows 44 through 45.
//! - The final F keeps the lower of the two, row 44.
//!
//!
//! The last three characters will be either <code>L</code> or <code>R</code>; these specify exactly one of the <em>8 columns</em> of seats on the plane (numbered <code>0</code> through <code>7</code>). The same process as above proceeds again, this time with only three steps.  <code>L</code> means to keep the <em>lower half</em>, while <code>R</code> means to keep the <em>upper half</em>.
//!
//! For example, consider just the last 3 characters of <code>FBFBBFFRLR</code>:
//!
//! - Start by considering the whole range, columns 0 through 7.
//! - R means to take the upper half, keeping columns 4 through 7.
//! - L means to take the lower half, keeping columns 4 through 5.
//! - The final R keeps the upper of the two, column 5.
//!
//!
//! So, decoding <code>FBFBBFFRLR</code> reveals that it is the seat at <em>row <code>44</code>, column <code>5</code></em>.
//!
//! Every seat also has a unique <em>seat ID</em>: multiply the row by 8, then add the column. In this example, the seat has ID <code>44 * 8 + 5 = <em>357</em></code>.
//!
//! Here are some other boarding passes:
//!
//! - BFFFBBFRRR: row 70, column 7, seat ID 567.
//! - FFFBBBFRRR: row 14, column 7, seat ID 119.
//! - BBFFBBFRLL: row 102, column 4, seat ID 820.
//!
//!
//! As a sanity check, look through your list of boarding passes. <em>What is the highest seat ID on a boarding pass?</em>
//!
//! # --- Part Two ---
//!
//! <em>Ding!</em> The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding pass in your list.  However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.
//!
//! <em>What is the ID of your seat?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
