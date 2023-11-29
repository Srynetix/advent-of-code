//! # --- Day 5: Hydrothermal Venture ---
//!
//! > _Exercise page: <https://adventofcode.com/2021/day/5>_
//!
//! > _Input page: <https://adventofcode.com/2021/day/5/input>_
//!
//! You come across a field of <a target="_blank" href="https://en.wikipedia.org/wiki/Hydrothermal_vent">hydrothermal vents</a> on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
//!
//! They tend to form in <em>lines</em>; the submarine helpfully produces a list of nearby <span title="Maybe they're Bresenham vents.">lines of vents</span> (your puzzle input) for you to review. For example:
//!
//! ```text
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//! ```
//!
//! Each line of vents is given as a line segment in the format <code>x1,y1 -&gt; x2,y2</code> where <code>x1</code>,<code>y1</code> are the coordinates of one end the line segment and <code>x2</code>,<code>y2</code> are the coordinates of the other end. These line segments include the points at both ends. In other words:
//!
//! - An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
//! - An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
//!
//!
//! For now, <em>only consider horizontal and vertical lines</em>: lines where either <code>x1 = x2</code> or <code>y1 = y2</code>.
//!
//! So, the horizontal and vertical lines from the above list would produce the following diagram:
//!
//! ```text
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//! ```
//!
//! In this diagram, the top left corner is <code>0,0</code> and the bottom right corner is <code>9,9</code>. Each position is shown as <em>the number of lines which cover that point</em> or <code>.</code> if no line covers that point. The top-left pair of <code>1</code>s, for example, comes from <code>2,2 -&gt; 2,1</code>; the very bottom row is formed by the overlapping lines <code>0,9 -&gt; 5,9</code> and <code>0,9 -&gt; 2,9</code>.
//!
//! To avoid the most dangerous areas, you need to determine <em>the number of points where at least two lines overlap</em>. In the above example, this is anywhere in the diagram with a <code>2</code> or larger - a total of <code><em>5</em></code> points.
//!
//! Consider only horizontal and vertical lines. <em>At how many points do at least two lines overlap?</em>
//!
//! # --- Part Two ---
//!
//! Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider <em>diagonal lines</em>.
//!
//! Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//!
//! - An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
//! - An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
//!
//!
//! Considering all lines from the above example would now produce the following diagram:
//!
//! ```text
//! 1.1....11.
//! .111...2..
//! ..2.1.111.
//! ...1.2.2..
//! .112313211
//! ...1.2....
//! ..1...1...
//! .1.....1..
//! 1.......1.
//! 222111....
//! ```
//!
//! You still need to determine <em>the number of points where at least two lines overlap</em>. In the above example, this is still anywhere in the diagram with a <code>2</code> or larger - now a total of <code><em>12</em></code> points.
//!
//! Consider all of the lines. <em>At how many points do at least two lines overlap?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
