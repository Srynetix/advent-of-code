//! # --- Day 8: Haunted Wasteland ---
//!
//! > _Exercise page: <https://adventofcode.com/2023/day/8>_
//!
//! > _Input page: <https://adventofcode.com/2023/day/8/input>_
//!
//! You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about <em>ghosts</em> a few minutes ago.
//!
//! One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of <em>network</em> of labeled nodes.
//!
//! It seems like you're meant to use the <em>left/right</em> instructions to <em>navigate the network</em>. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!
//!
//! After examining the maps for a bit, two nodes stick out: <code>AAA</code> and <code>ZZZ</code>. You feel like <code>AAA</code> is where you are now, and you have to follow the left/right instructions until you reach <code>ZZZ</code>.
//!
//! This format defines each <em>node</em> of the network individually. For example:
//!
//! ```text
//! RL
//!
//! AAA = (BBB, CCC)
//! BBB = (DDD, EEE)
//! CCC = (ZZZ, GGG)
//! DDD = (DDD, DDD)
//! EEE = (EEE, EEE)
//! GGG = (GGG, GGG)
//! ZZZ = (ZZZ, ZZZ)
//! ```
//!
//! Starting with <code>AAA</code>, you need to <em>look up the next element</em> based on the next left/right instruction in your input. In this example, start with <code>AAA</code> and go <em>right</em> (<code>R</code>) by choosing the right element of <code>AAA</code>, <code><em>CCC</em></code>. Then, <code>L</code> means to choose the <em>left</em> element of <code>CCC</code>, <code><em>ZZZ</em></code>. By following the left/right instructions, you reach <code>ZZZ</code> in <code><em>2</em></code> steps.
//!
//! Of course, you might not find <code>ZZZ</code> right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: <code>RL</code> really means <code>RLRLRLRLRLRLRLRL...</code> and so on. For example, here is a situation that takes <code><em>6</em></code> steps to reach <code>ZZZ</code>:
//!
//! ```text
//! LLR
//!
//! AAA = (BBB, BBB)
//! BBB = (AAA, ZZZ)
//! ZZZ = (ZZZ, ZZZ)
//! ```
//!
//! Starting at <code>AAA</code>, follow the left/right instructions. <em>How many steps are required to reach <code>ZZZ</code>?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
