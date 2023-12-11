//! # --- Day 10: Pipe Maze ---
//!
//! > _Exercise page: <https://adventofcode.com/2023/day/10>_
//!
//! > _Input page: <https://adventofcode.com/2023/day/10/input>_
//!
//! You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.
//!
//! You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "<a target="_blank" href="https://en.wikipedia.org/wiki/Hot_spring">Hot Springs</a>" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.
//!
//! The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.
//!
//! Scanning the area, you discover that the entire field you're standing on is <span title="Manufactured by Hamilton and Hilbert Pipe Company">densely packed with pipes</span>; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).
//!
//! The pipes are arranged in a two-dimensional grid of <em>tiles</em>:
//!
//! - | is a vertical pipe connecting north and south.
//! - - is a horizontal pipe connecting east and west.
//! - L is a 90-degree bend connecting north and east.
//! - J is a 90-degree bend connecting north and west.
//! - 7 is a 90-degree bend connecting south and west.
//! - F is a 90-degree bend connecting south and east.
//! - . is ground; there is no pipe in this tile.
//! - S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
//!
//!
//! Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is <em>one large, continuous loop</em>.
//!
//! For example, here is a square loop of pipe:
//!
//! ```text
//! .....
//! .F-7.
//! .|.|.
//! .L-J.
//! .....
//! ```
//!
//! If the animal had entered this loop in the northwest corner, the sketch would instead look like this:
//!
//! ```text
//! .....
//! .S-7.
//! .|.|.
//! .L-J.
//! .....
//! ```
//!
//! In the above diagram, the <code>S</code> tile is still a 90-degree <code>F</code> bend: you can tell because of how the adjacent pipes connect to it.
//!
//! Unfortunately, there are also many pipes that <em>aren't connected to the loop</em>! This sketch shows the same loop as above:
//!
//! ```text
//! -L|F7
//! 7S-7|
//! L|7||
//! -L-J|
//! L|-JF
//! ```
//!
//! In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to <code>S</code>, pipes those pipes connect to, pipes <em>those</em> pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including <code>S</code>, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).
//!
//! Here is a sketch that contains a slightly more complex main loop:
//!
//! ```text
//! ..F7.
//! .FJ|.
//! SJ.L7
//! |F--J
//! LJ...
//! ```
//!
//! Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:
//!
//! ```text
//! 7-F7-
//! .FJ|7
//! SJLL7
//! |F--J
//! LJ.LJ
//! ```
//!
//! If you want to <em>get out ahead of the animal</em>, you should find the tile in the loop that is <em>farthest</em> from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps <em>along the loop</em> to reach from the starting point - regardless of which way around the loop the animal went.
//!
//! In the first example with the square loop:
//!
//! ```text
//! .....
//! .S-7.
//! .|.|.
//! .L-J.
//! .....
//! ```
//!
//! You can count the distance each tile in the loop is from the starting point like this:
//!
//! ```text
//! .....
//! .012.
//! .1.3.
//! .234.
//! .....
//! ```
//!
//! In this example, the farthest point from the start is <code><em>4</em></code> steps away.
//!
//! Here's the more complex loop again:
//!
//! ```text
//! ..F7.
//! .FJ|.
//! SJ.L7
//! |F--J
//! LJ...
//! ```
//!
//! Here are the distances for each tile on that loop:
//!
//! ```text
//! ..45.
//! .236.
//! 01.78
//! 14567
//! 23...
//! ```
//!
//! Find the single giant loop starting at <code>S</code>. <em>How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?</em>
//!
//! # --- Part Two ---
//!
//! You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is <em>within the area enclosed by the loop</em>?
//!
//! To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:
//!
//! ```text
//! ...........
//! .S-------7.
//! .|F-----7|.
//! .||.....||.
//! .||.....||.
//! .|L-7.F-J|.
//! .|..|.|..|.
//! .L--J.L--J.
//! ...........
//! ```
//!
//! The above loop encloses merely <em>four tiles</em> - the two pairs of <code>.</code> in the southwest and southeast (marked <code>I</code> below). The middle <code>.</code> tiles (marked <code>O</code> below) are <em>not</em> in the loop. Here is the same loop again with those regions marked:
//!
//! ```text
//! ...........
//! .S-------7.
//! .|F-----7|.
//! .||OOOOO||.
//! .||OOOOO||.
//! .|L-7OF-J|.
//! .|II|O|II|.
//! .L--JOL--J.
//! .....O.....
//! ```
//!
//! In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, <code>I</code> is still within the loop and <code>O</code> is still outside the loop:
//!
//! ```text
//! ..........
//! .S------7.
//! .|F----7|.
//! .||OOOO||.
//! .||OOOO||.
//! .|L-7F-J|.
//! .|II||II|.
//! .L--JL--J.
//! ..........
//! ```
//!
//! In both of the above examples, <code><em>4</em></code> tiles are enclosed by the loop.
//!
//! Here's a larger example:
//!
//! ```text
//! .F----7F7F7F7F-7....
//! .|F--7||||||||FJ....
//! .||.FJ||||||||L7....
//! FJL7L7LJLJ||LJ.L-7..
//! L--J.L7...LJS7F-7L7.
//! ....F-J..F7FJ|L7L7L7
//! ....L7.F7||L7|.L7L7|
//! .....|FJLJ|FJ|F7|.LJ
//! ....FJL-7.||.||||...
//! ....L---J.LJ.LJLJ...
//! ```
//!
//! The above sketch has many random bits of ground, some of which are in the loop (<code>I</code>) and some of which are outside it (<code>O</code>):
//!
//! ```text
//! OF----7F7F7F7F-7OOOO
//! O|F--7||||||||FJOOOO
//! O||OFJ||||||||L7OOOO
//! FJL7L7LJLJ||LJIL-7OO
//! L--JOL7IIILJS7F-7L7O
//! OOOOF-JIIF7FJ|L7L7L7
//! OOOOL7IF7||L7|IL7L7|
//! OOOOO|FJLJ|FJ|F7|OLJ
//! OOOOFJL-7O||O||||OOO
//! OOOOL---JOLJOLJLJOOO
//! ```
//!
//! In this larger example, <code><em>8</em></code> tiles are enclosed by the loop.
//!
//! Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:
//!
//! ```text
//! FF7FSF7F7F7F7F7F---7
//! L|LJ||||||||||||F--J
//! FL-7LJLJ||||||LJL-77
//! F--JF--7||LJLJ7F7FJ-
//! L---JF-JLJ.||-FJLJJ7
//! |F|F-JF---7F7-L7L|7|
//! |FFJF7L7F-JF7|JL---7
//! 7-L-JL7||F7|L7F-7F7|
//! L.L7LFJ|||||FJL7||LJ
//! L7JLJL-JLJLJL--JLJ.L
//! ```
//!
//! Here are just the tiles that are <em>enclosed by the loop</em> marked with <code>I</code>:
//!
//! ```text
//! FF7FSF7F7F7F7F7F---7
//! L|LJ||||||||||||F--J
//! FL-7LJLJ||||||LJL-77
//! F--JF--7||LJLJIF7FJ-
//! L---JF-JLJIIIIFJLJJ7
//! |F|F-JF---7IIIL7L|7|
//! |FFJF7L7F-JF7IIL---7
//! 7-L-JL7||F7|L7F-7F7|
//! L.L7LFJ|||||FJL7||LJ
//! L7JLJL-JLJLJL--JLJ.L
//! ```
//!
//! In this last example, <code><em>10</em></code> tiles are enclosed by the loop.
//!
//! Figure out whether you have time to search for the nest by calculating the area within the loop. <em>How many tiles are enclosed by the loop?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
