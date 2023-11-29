//! # --- Day 1: Calorie Counting ---
//!
//! > _Exercise page: <https://adventofcode.com/2022/day/1>_
//!
//! > _Input page: <https://adventofcode.com/2022/day/1/input>_
//!
//! Santa's reindeer typically eat regular reindeer food, but they need a lot of <a href="/2018/day/25">magical energy</a> to deliver presents on Christmas. For that, their favorite snack is a special type of <em class="star">star</em> fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.
//!
//! To supply enough magical energy, the expedition needs to retrieve a minimum of <em class="star">fifty stars</em> by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
//!
//! Collect stars by solving puzzles.  Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first.  Each puzzle grants <em class="star">one star</em>. Good luck!
//!
//! The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of <em>Calories</em> each Elf is carrying (your puzzle input).
//!
//! The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, <span title="By &quot;etc&quot;, you're pretty sure they just mean &quot;more snacks&quot;.">etc.</span> that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
//!
//! For example, suppose the Elves finish writing their items' Calories and end up with the following list:
//!
//! ```text
//! 1000
//! 2000
//! 3000
//!
//! 4000
//!
//! 5000
//! 6000
//!
//! 7000
//! 8000
//! 9000
//!
//! 10000
//! ```
//!
//! This list represents the Calories of the food carried by five Elves:
//!
//! - The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
//! - The second Elf is carrying one food item with 4000 Calories.
//! - The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
//! - The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
//! - The fifth Elf is carrying one food item with 10000 Calories.
//!
//!
//! In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the <em>most</em> Calories. In the example above, this is <em><code>24000</code></em> (carried by the fourth Elf).
//!
//! Find the Elf carrying the most Calories. <em>How many total Calories is that Elf carrying?</em>
//!
//! # --- Part Two ---
//!
//! By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually <em>run out of snacks</em>.
//!
//! To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the <em>top three</em> Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.
//!
//! In the example above, the top three Elves are the fourth Elf (with <code>24000</code> Calories), then the third Elf (with <code>11000</code> Calories), then the fifth Elf (with <code>10000</code> Calories). The sum of the Calories carried by these three elves is <code><em>45000</em></code>.
//!
//! Find the top three Elves carrying the most Calories. <em>How many Calories are those Elves carrying in total?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
