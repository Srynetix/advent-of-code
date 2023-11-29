//! # --- Day 8: Handheld Halting ---
//!
//! > _Exercise page: <https://adventofcode.com/2020/day/8>_
//!
//! > _Input page: <https://adventofcode.com/2020/day/8/input>_
//!
//! Your flight to the major airline hub reaches cruising altitude without incident.  While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.
//!
//! Their <a target="_blank" href="https://en.wikipedia.org/wiki/Handheld_game_console">handheld game console</a> won't turn on! They ask if you can take a look.
//!
//! You narrow the problem down to a strange <em>infinite loop</em> in the <span title="A trendy new line of encrypted footwear?">boot code</span> (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.
//!
//! The boot code is represented as a text file with one <em>instruction</em> per line of text. Each instruction consists of an <em>operation</em> (<code>acc</code>, <code>jmp</code>, or <code>nop</code>) and an <em>argument</em> (a signed number like <code>+4</code> or <code>-20</code>).
//!
//! - acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
//! - jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
//! - nop stands for No OPeration - it does nothing.  The instruction immediately below it is executed next.
//!
//!
//! For example, consider the following program:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! These instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2, 8(!)
//! jmp +4  | 3
//! acc +3  | 6
//! jmp -3  | 7
//! acc -99 |
//! acc +1  | 4
//! jmp -4  | 5
//! acc +6  |
//! ```
//!
//! First, the <code>nop +0</code> does nothing. Then, the accumulator is increased from 0 to 1 (<code>acc +1</code>) and <code>jmp +4</code> sets the next instruction to the other <code>acc +1</code> near the bottom. After it increases the accumulator from 1 to 2, <code>jmp -4</code> executes, setting the next instruction to the only <code>acc +3</code>. It sets the accumulator to 5, and <code>jmp -3</code> causes the program to continue back at the first <code>acc +1</code>.
//!
//! This is an <em>infinite loop</em>: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.
//!
//! Immediately <em>before</em> the program would run an instruction a second time, the value in the accumulator is <em><code>5</code></em>.
//!
//! Run your copy of the boot code. Immediately before any instruction is executed a second time, <em>what value is in the accumulator?</em>
//!
//! # --- Part Two ---
//!
//! After some careful analysis, you believe that <em>exactly one instruction is corrupted</em>.
//!
//! Somewhere in the program, <em>either</em> a <code>jmp</code> is supposed to be a <code>nop</code>, <em>or</em> a <code>nop</code> is supposed to be a <code>jmp</code>. (No <code>acc</code> instructions were harmed in the corruption of this boot code.)
//!
//! The program is supposed to terminate by <em>attempting to execute an instruction immediately after the last instruction in the file</em>. By changing exactly one <code>jmp</code> or <code>nop</code>, you can repair the boot code and make it terminate correctly.
//!
//! For example, consider the same program from above:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! If you change the first instruction from <code>nop +0</code> to <code>jmp +0</code>, it would create a single-instruction infinite loop, never leaving that instruction.  If you change almost any of the <code>jmp</code> instructions, the program will still eventually find another <code>jmp</code> instruction and loop forever.
//!
//! However, if you change the second-to-last instruction (from <code>jmp -4</code> to <code>nop -4</code>), the program terminates! The instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2
//! jmp +4  | 3
//! acc +3  |
//! jmp -3  |
//! acc -99 |
//! acc +1  | 4
//! nop -4  | 5
//! acc +6  | 6
//! ```
//!
//! After the last instruction (<code>acc +6</code>), the program terminates by attempting to run the instruction below the last instruction in the file.  With this change, after the program terminates, the accumulator contains the value <em><code>8</code></em> (<code>acc +1</code>, <code>acc +1</code>, <code>acc +6</code>).
//!
//! Fix the program so that it terminates normally by changing exactly one <code>jmp</code> (to <code>nop</code>) or <code>nop</code> (to <code>jmp</code>). <em>What is the value of the accumulator after the program terminates?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
