//! # --- Day 6: Wait For It ---
//!
//! > _Exercise page: <https://adventofcode.com/2023/day/6>_
//!
//! > _Input page: <https://adventofcode.com/2023/day/6/input>_
//!
//! The ferry quickly brings you across Island Island. After asking around, you discover that there is indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of water and the small island where the ferry has docked.
//!
//! As you try to figure out what to do next, you notice a poster on a wall near the ferry dock. "Boat races! Open to the public! Grand prize is an all-expenses-paid trip to <em>Desert Island</em>!" That must be where the sand comes from! Best of all, the boat races are starting in just a few minutes.
//!
//! You manage to sign up as a competitor in the boat races just in time. The organizer explains that it's not really a traditional race - instead, you will get a fixed amount of time during which your boat has to travel as far as it can, and you win if your boat goes the farthest.
//!
//! As part of signing up, you get a sheet of paper (your puzzle input) that lists the <em>time</em> allowed for each race and also the best <em>distance</em> ever recorded in that race. To guarantee you win the grand prize, you need to make sure you <em>go farther in each race</em> than the current record holder.
//!
//! The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually <em>toy boats</em>, each with a big button on top. Holding down the button <em>charges the boat</em>, and releasing the button <em>allows the boat to move</em>. Boats move faster if their button was held longer, but time spent holding the button counts against the total race time. You can only hold the button at the start of the race, and boats don't move until the button is released.
//!
//! For example:
//!
//! ```text
//! Time:      7  15   30
//! Distance:  9  40  200
//! ```
//!
//! This document describes three races:
//!
//! - The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
//! - The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
//! - The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.
//!
//!
//! Your toy boat has a starting speed of <em>zero millimeters per millisecond</em>. For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by <em>one millimeter per millisecond</em>.
//!
//! So, because the first race lasts 7 milliseconds, you only have a few options:
//!
//! - Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
//! - Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
//! - Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
//! - Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
//! - Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
//! - Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
//! - Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
//! - Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.
//!
//!
//! Since the current record for this race is <code>9</code> millimeters, there are actually <code><em>4</em></code> different ways you could win: you could hold the button for <code>2</code>, <code>3</code>, <code>4</code>, or <code>5</code> milliseconds at the start of the race.
//!
//! In the second race, you could hold the button for at least <code>4</code> milliseconds and at most <code>11</code> milliseconds and beat the record, a total of <code><em>8</em></code> different ways to win.
//!
//! In the third race, you could hold the button for at least <code>11</code> milliseconds and no more than <code>19</code> milliseconds and still beat the record, a total of <code><em>9</em></code> ways you could win.
//!
//! To see how much margin of error you have, determine the <em>number of ways you can beat the record</em> in each race; in this example, if you multiply these values together, you get <code><em>288</em></code> (<code>4</code> * <code>8</code> * <code>9</code>).
//!
//! Determine the number of ways you could beat the record in each race. <em>What do you get if you multiply these numbers together?</em>
//!
//! # --- Part Two ---
//!
//! As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has <span title="Keming!">very bad</span> <a href="https://en.wikipedia.org/wiki/Kerning" target="_blank">kerning</a>. There's really <em>only one race</em> - ignore the spaces between the numbers on each line.
//!
//! So, the example from before:
//!
//! ```text
//! Time:      7  15   30
//! Distance:  9  40  200
//! ```
//!
//! ...now instead means this:
//!
//! ```text
//! Time:      71530
//! Distance:  940200
//! ```
//!
//! Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for <em><code>71530</code> milliseconds</em> and the record distance you need to beat is <em><code>940200</code> millimeters</em>. You could hold the button anywhere from <code>14</code> to <code>71516</code> milliseconds and beat the record, a total of <code><em>71503</em></code> ways!
//!
//! <em>How many ways can you beat the record in this one much longer race?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
