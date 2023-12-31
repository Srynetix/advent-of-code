//! # --- Day 11: Cosmic Expansion ---
//!
//! > _Exercise page: <https://adventofcode.com/2023/day/11>_
//!
//! > _Input page: <https://adventofcode.com/2023/day/11/input>_
//!
//! You continue following signs for "Hot Springs" and eventually come across an <a target="_blank" href="https://en.wikipedia.org/wiki/Observatory">observatory</a>. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.
//!
//! He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.
//!
//! Maybe you can help him with the analysis to speed things up?
//!
//! The researcher has collected a bunch of data and compiled the data into a single giant <em>image</em> (your puzzle input). The image includes <em>empty space</em> (<code>.</code>) and <em>galaxies</em> (<code>#</code>). For example:
//!
//! ```text
//! ...#......
//! .......#..
//! #.........
//! ..........
//! ......#...
//! .#........
//! .........#
//! ..........
//! .......#..
//! #...#.....
//! ```
//!
//! The researcher is trying to figure out the sum of the lengths of the <em>shortest path between every pair of galaxies</em>. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.
//!
//! Due to something involving gravitational effects, <em>only some space expands</em>. In fact, the result is that <em>any rows or columns that contain no galaxies</em> should all actually be twice as big.
//!
//! In the above example, three columns and two rows contain no galaxies:
//!
//! ```text
//!    v  v  v
//!  ...#......
//!  .......#..
//!  #.........
//! >..........<
//!  ......#...
//!  .#........
//!  .........#
//! >..........<
//!  .......#..
//!  #...#.....
//!    ^  ^  ^
//! ```
//!
//! These rows and columns need to be <em>twice as big</em>; the result of cosmic expansion therefore looks like this:
//!
//! ```text
//! ....#........
//! .........#...
//! #............
//! .............
//! .............
//! ........#....
//! .#...........
//! ............#
//! .............
//! .............
//! .........#...
//! #....#.......
//! ```
//!
//! Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:
//!
//! ```text
//! ....1........
//! .........2...
//! 3............
//! .............
//! .............
//! ........4....
//! .5...........
//! ............6
//! .............
//! .............
//! .........7...
//! 8....9.......
//! ```
//!
//! In these 9 galaxies, there are <em>36 pairs</em>. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one <code>.</code> or <code>#</code> at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)
//!
//! For example, here is one of the shortest paths between galaxies <code>5</code> and <code>9</code>:
//!
//! ```text
//! ....1........
//! .........2...
//! 3............
//! .............
//! .............
//! ........4....
//! .5...........
//! .##.........6
//! ..##.........
//! ...##........
//! ....##...7...
//! 8....9.......
//! ```
//!
//! This path has length <code><em>9</em></code> because it takes a minimum of <em>nine steps</em> to get from galaxy <code>5</code> to galaxy <code>9</code> (the eight locations marked <code>#</code> plus the step onto galaxy <code>9</code> itself). Here are some other example shortest path lengths:
//!
//! - Between galaxy 1 and galaxy 7: 15
//! - Between galaxy 3 and galaxy 6: 17
//! - Between galaxy 8 and galaxy 9: 5
//!
//!
//! In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is <code><em>374</em></code>.
//!
//! Expand the universe, then find the length of the shortest path between every pair of galaxies. <em>What is the sum of these lengths?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
