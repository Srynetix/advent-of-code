//! # --- Day 18: Operation Order ---
//!
//! > _Exercise page: <https://adventofcode.com/2020/day/18>_
//!
//! > _Input page: <https://adventofcode.com/2020/day/18/input>_
//!
//! As you look out the window and notice a heavily-forested continent slowly appear over the horizon, you are interrupted by the child sitting next to you. They're curious if you could help them with their <span title="Or &quot;maths&quot;, if you have more than one.">math</span> homework.
//!
//! Unfortunately, it seems like this "math" <a href="https://www.youtube.com/watch?v=3QtRK7Y2pPU&amp;t=15" target="_blank">follows different rules</a> than you remember.
//!
//! The homework (your puzzle input) consists of a series of expressions that consist of addition (<code>+</code>), multiplication (<code>*</code>), and parentheses (<code>(...)</code>). Just like normal math, parentheses indicate that the expression inside must be evaluated before it can be used by the surrounding expression. Addition still finds the sum of the numbers on both sides of the operator, and multiplication still finds the product.
//!
//! However, the rules of <em>operator precedence</em> have changed. Rather than evaluating multiplication before addition, the operators have the <em>same precedence</em>, and are evaluated left-to-right regardless of the order in which they appear.
//!
//! For example, the steps to evaluate the expression <code>1 + 2 * 3 + 4 * 5 + 6</code> are as follows:
//!
//! ```text
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!       9   + 4 * 5 + 6
//!          13   * 5 + 6
//!              65   + 6
//!                  71
//! ```
//!
//! Parentheses can override this order; for example, here is what happens if parentheses are added to form <code>1 + (2 * 3) + (4 * (5 + 6))</code>:
//!
//! ```text
//! 1 + (2 * 3) + (4 * (5 + 6))
//! 1 +    6    + (4 * (5 + 6))
//!      7      + (4 * (5 + 6))
//!      7      + (4 *   11   )
//!      7      +     44
//!             51
//! ```
//!
//! Here are a few more examples:
//!
//! - 2 * 3 + (4 * 5) becomes 26.
//! - 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
//! - 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
//! - ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
//!
//!
//! Before you can help with the homework, you need to understand it yourself. <em>Evaluate the expression on each line of the homework; what is the sum of the resulting values?</em>
//!
//! # --- Part Two ---
//!
//! You manage to answer the child's questions and they finish part 1 of their homework, but get stuck when they reach the next section: <em>advanced</em> math.
//!
//! Now, addition and multiplication have <em>different</em> precedence levels, but they're not the ones you're familiar with. Instead, addition is evaluated <em>before</em> multiplication.
//!
//! For example, the steps to evaluate the expression <code>1 + 2 * 3 + 4 * 5 + 6</code> are now as follows:
//!
//! ```text
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!   3   *   7   * 5 + 6
//!   3   *   7   *  11
//!      21       *  11
//!          231
//! ```
//!
//! Here are the other examples from above:
//!
//! - 1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
//! - 2 * 3 + (4 * 5) becomes 46.
//! - 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
//! - 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
//! - ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
//!
//!
//! <em>What do you get if you add up the results of evaluating the homework problems using these new rules?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
