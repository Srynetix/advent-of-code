//! # --- Day 7: Handy Haversacks ---
//!
//! > _Exercise page: <https://adventofcode.com/2020/day/7>_
//!
//! > _Input page: <https://adventofcode.com/2020/day/7/input>_
//!
//! You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to <em>issues in luggage processing</em>.
//!
//! Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!
//!
//! For example, consider the following rules:
//!
//! ```text
//! light red bags contain 1 bright white bag, 2 muted yellow bags.
//! dark orange bags contain 3 bright white bags, 4 muted yellow bags.
//! bright white bags contain 1 shiny gold bag.
//! muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
//! shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
//! dark olive bags contain 3 faded blue bags, 4 dotted black bags.
//! vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
//! faded blue bags contain no other bags.
//! dotted black bags contain no other bags.
//! ```
//!
//! These rules specify the required contents for 9 bag types. In this example, every <code>faded blue</code> bag is empty, every <code>vibrant plum</code> bag contains 11 bags (5 <code>faded blue</code> and 6 <code>dotted black</code>), and so on.
//!
//! You have a <code><em>shiny gold</em></code> bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one <code>shiny gold</code> bag?)
//!
//! In the above rules, the following options would be available to you:
//!
//! - A bright white bag, which can hold your shiny gold bag directly.
//! - A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
//! - A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//! - A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//!
//!
//! So, in this example, the number of bag colors that can eventually contain at least one <code>shiny gold</code> bag is <code><em>4</em></code>.
//!
//! <em>How many bag colors can eventually contain at least one <code>shiny gold</code> bag?</em> (The list of rules is quite long; make sure you get all of it.)
//!
//! # --- Part Two ---
//!
//! It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!
//!
//! Consider again your <code>shiny gold</code> bag and the rules from the above example:
//!
//! - faded blue bags contain 0 other bags.
//! - dotted black bags contain 0 other bags.
//! - vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
//! - dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
//!
//!
//! So, a single <code>shiny gold</code> bag must contain 1 <code>dark olive</code> bag (and the 7 bags within it) plus 2 <code>vibrant plum</code> bags (and the 11 bags within <em>each</em> of those): <code>1 + 1*7 + 2 + 2*11</code> = <code><em>32</em></code> bags!
//!
//! Of course, the actual rules have a <span title="100%">small</span> chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!
//!
//! Here's another example:
//!
//! ```text
//! shiny gold bags contain 2 dark red bags.
//! dark red bags contain 2 dark orange bags.
//! dark orange bags contain 2 dark yellow bags.
//! dark yellow bags contain 2 dark green bags.
//! dark green bags contain 2 dark blue bags.
//! dark blue bags contain 2 dark violet bags.
//! dark violet bags contain no other bags.
//! ```
//!
//! In this example, a single <code>shiny gold</code> bag must contain <code><em>126</em></code> other bags.
//!
//! <em>How many individual bags are required inside your single <code>shiny gold</code> bag?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
