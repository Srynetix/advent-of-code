//! # --- Day 1: Historian Hysteria ---
//!
//! > _Exercise page: <https://adventofcode.com/2024/day/1>_
//!
//! > _Input page: <https://adventofcode.com/2024/day/1/input>_
//!
//! The <em>Chief Historian</em> is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last anyone heard, he was visiting locations that are historically significant to the North Pole; a group of Senior Historians has asked you to accompany them as they check the places they think he was most likely to visit.
//!
//! As each location is checked, they will mark it on their list with a <em class="star">star</em>. They figure the Chief Historian <em>must</em> be in one of the first fifty places they'll look, so in order to save Christmas, you need to help them get <em class="star">fifty stars</em> on their list before Santa takes off on December 25th.
//!
//! Collect stars by solving puzzles.  Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first.  Each puzzle grants <em class="star">one star</em>. Good luck!
//!
//! You haven't even left yet and the group of Elvish Senior Historians has already hit a problem: their list of locations to check is currently <em>empty</em>. Eventually, someone decides that the best place to check first would be the Chief Historian's office.
//!
//! Upon pouring into the office, everyone confirms that the Chief Historian is indeed nowhere to be found. Instead, the Elves discover an assortment of notes and lists of historically significant locations! This seems to be the planning the Chief Historian was doing before he left. Perhaps these notes can be used to determine which locations to search?
//!
//! Throughout the Chief's office, the historically significant locations are listed not by name but by a unique number called the <em>location ID</em>. To make sure they don't miss anything, The Historians split into two groups, each searching the office and trying to create their own complete list of location IDs.
//!
//! There's just one problem: by holding the two lists up <em>side by side</em> (your puzzle input), it quickly becomes clear that the lists aren't very similar. Maybe you can help The Historians reconcile their lists?
//!
//! For example:
//!
//! ```text
//! 3   4
//! 4   3
//! 2   5
//! 1   3
//! 3   9
//! 3   3
//! ```
//!
//! Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are. Pair up the <em>smallest number in the left list</em> with the <em>smallest number in the right list</em>, then the <em>second-smallest left number</em> with the <em>second-smallest right number</em>, and so on.
//!
//! Within each pair, figure out <em>how far apart</em> the two numbers are; you'll need to <em>add up all of those distances</em>. For example, if you pair up a <code>3</code> from the left list with a <code>7</code> from the right list, the distance apart is <code>4</code>; if you pair up a <code>9</code> with a <code>3</code>, the distance apart is <code>6</code>.
//!
//! In the example list above, the pairs and distances would be as follows:
//!
//! - The smallest number in the left list is 1, and the smallest number in the right list is 3. The distance between them is 2.
//! - The second-smallest number in the left list is 2, and the second-smallest number in the right list is another 3. The distance between them is 1.
//! - The third-smallest number in both lists is 3, so the distance between them is 0.
//! - The next numbers to pair up are 3 and 4, a distance of 1.
//! - The fifth-smallest numbers in each list are 3 and 5, a distance of 2.
//! - Finally, the largest number in the left list is 4, while the largest number in the right list is 9; these are a distance 5 apart.
//!
//!
//! To find the <em>total distance</em> between the left list and the right list, add up the distances between all of the pairs you found. In the example above, this is <code>2 + 1 + 0 + 1 + 2 + 5</code>, a total distance of <code><em>11</em></code>!
//!
//! Your actual left and right lists contain many location IDs. <em>What is the total distance between your lists?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
