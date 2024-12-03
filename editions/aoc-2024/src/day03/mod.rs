//! # --- Day 3: Mull It Over ---
//!
//! > _Exercise page: <https://adventofcode.com/2024/day/3>_
//!
//! > _Input page: <https://adventofcode.com/2024/day/3/input>_
//!
//! "Our computers are having issues, so I have no idea if we have any Chief Historians <span title="There's a spot reserved for Chief Historians between the green toboggans and the red toboggans. They've never actually had any Chief Historians in stock, but it's best to be prepared.">in stock</span>! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the <a href="/2020/day/2">North Pole Toboggan Rental Shop</a>. The Historians head out to take a look.
//!
//! The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"
//!
//! The computer appears to be trying to run a program, but its memory (your puzzle input) is <em>corrupted</em>. All of the instructions have been jumbled up!
//!
//! It seems like the goal of the program is just to <em>multiply some numbers</em>. It does that with instructions like <code>mul(X,Y)</code>, where <code>X</code> and <code>Y</code> are each 1-3 digit numbers. For instance, <code>mul(44,46)</code> multiplies <code>44</code> by <code>46</code> to get a result of <code>2024</code>. Similarly, <code>mul(123,4)</code> would multiply <code>123</code> by <code>4</code>.
//!
//! However, because the program's memory has been corrupted, there are also many invalid characters that should be <em>ignored</em>, even if they look like part of a <code>mul</code> instruction. Sequences like <code>mul(4*</code>, <code>mul(6,9!</code>, <code>?(12,34)</code>, or <code>mul ( 2 , 4 )</code> do <em>nothing</em>.
//!
//! For example, consider the following section of corrupted memory:
//!
//! ```text
//! xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))```
//!
//! Only the four highlighted sections are real <code>mul</code> instructions. Adding up the result of each instruction produces <code><em>161</em></code> (<code>2*4 + 5*5 + 11*8 + 8*5</code>).
//!
//! Scan the corrupted memory for uncorrupted <code>mul</code> instructions. <em>What do you get if you add up all of the results of the multiplications?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
