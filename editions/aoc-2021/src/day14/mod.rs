//! # --- Day 14: Extended Polymerization ---
//!
//! > _Exercise page: <https://adventofcode.com/2021/day/14>_
//!
//! > _Input page: <https://adventofcode.com/2021/day/14/input>_
//!
//! The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has <a href="https://en.wikipedia.org/wiki/Polymerization" target="_blank">polymerization</a> equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.
//!
//! The submarine manual contains <span title="HO
//!
//! HO -> OH">instructions</span> for finding the optimal polymer formula; specifically, it offers a <em>polymer template</em> and a list of <em>pair insertion</em> rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.
//!
//! For example:
//!
//! ```text
//! NNCB
//!
//! CH -> B
//! HH -> N
//! CB -> H
//! NH -> C
//! HB -> C
//! HC -> B
//! HN -> C
//! NN -> C
//! BH -> H
//! NC -> B
//! NB -> B
//! BN -> B
//! BB -> N
//! BC -> B
//! CC -> N
//! CN -> C
//! ```
//!
//! The first line is the <em>polymer template</em> - this is the starting point of the process.
//!
//! The following section defines the <em>pair insertion</em> rules. A rule like <code>AB -&gt; C</code> means that when elements <code>A</code> and <code>B</code> are immediately adjacent, element <code>C</code> should be inserted between them. These insertions all happen simultaneously.
//!
//! So, starting with the polymer template <code>NNCB</code>, the first step simultaneously considers all three pairs:
//!
//! - The first pair (NN) matches the rule NN -> C, so element C is inserted between the first N and the second N.
//! - The second pair (NC) matches the rule NC -> B, so element B is inserted between the N and the C.
//! - The third pair (CB) matches the rule CB -> H, so element H is inserted between the C and the B.
//!
//!
//! Note that these pairs overlap: the second element of one pair is the first element of the next pair. Also, because all pairs are considered simultaneously, inserted elements are not considered to be part of a pair until the next step.
//!
//! After the first step of this process, the polymer becomes <code>N<em>C</em>N<em>B</em>C<em>H</em>B</code>.
//!
//! Here are the results of a few steps using the above rules:
//!
//! ```text
//! Template:     NNCB
//! After step 1: NCNBCHB
//! After step 2: NBCCNBBBCBHCB
//! After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
//! After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
//! ```
//!
//! This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073. After step 10, <code>B</code> occurs 1749 times, <code>C</code> occurs 298 times, <code>H</code> occurs 161 times, and <code>N</code> occurs 865 times; taking the quantity of the most common element (<code>B</code>, 1749) and subtracting the quantity of the least common element (<code>H</code>, 161) produces <code>1749 - 161 = <em>1588</em></code>.
//!
//! Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result. <em>What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?</em>
//!
//! # --- Part Two ---
//!
//! The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run more steps of the pair insertion process; a total of <em>40 steps</em> should do it.
//!
//! In the above example, the most common element is <code>B</code> (occurring <code>2192039569602</code> times) and the least common element is <code>H</code> (occurring <code>3849876073</code> times); subtracting these produces <code><em>2188189693529</em></code>.
//!
//! Apply <em>40</em> steps of pair insertion to the polymer template and find the most and least common elements in the result. <em>What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
