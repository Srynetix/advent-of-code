//! # --- Day 13: Shuttle Search ---
//!
//! > _Exercise page: <https://adventofcode.com/2020/day/13>_
//!
//! > _Input page: <https://adventofcode.com/2020/day/13/input>_
//!
//! Your ferry can make it safely to a nearby port, but it won't get much further. When you call to book another ship, you discover that no ships embark from that port to your vacation island. You'll need to get from the port to the nearest airport.
//!
//! Fortunately, a shuttle bus service is available to bring you from the sea port to the airport!  Each bus has an ID number that also indicates <em>how often the bus leaves for the airport</em>.
//!
//! Bus schedules are defined based on a <em>timestamp</em> that measures the <em>number of minutes</em> since some fixed reference point in the past. At timestamp <code>0</code>, every bus simultaneously departed from the sea port. After that, each bus travels to the airport, then various other locations, and finally returns to the sea port to repeat its journey forever.
//!
//! The time this loop takes a particular bus is also its ID number: the bus with ID <code>5</code> departs from the sea port at timestamps <code>0</code>, <code>5</code>, <code>10</code>, <code>15</code>, and so on. The bus with ID <code>11</code> departs at <code>0</code>, <code>11</code>, <code>22</code>, <code>33</code>, and so on. If you are there when the bus departs, you can ride that bus to the airport!
//!
//! Your notes (your puzzle input) consist of two lines.  The first line is your estimate of the <em>earliest timestamp you could depart on a bus</em>. The second line lists the bus IDs that are in service according to the shuttle company; entries that show <code>x</code> must be out of service, so you decide to ignore them.
//!
//! To save time once you arrive, your goal is to figure out <em>the earliest bus you can take to the airport</em>. (There will be exactly one such bus.)
//!
//! For example, suppose you have the following notes:
//!
//! ```text
//! 939
//! 7,13,x,x,59,x,31,19
//! ```
//!
//! Here, the earliest timestamp you could depart is <code>939</code>, and the bus IDs in service are <code>7</code>, <code>13</code>, <code>59</code>, <code>31</code>, and <code>19</code>. Near timestamp <code>939</code>, these bus IDs depart at the times marked <code>D</code>:
//!
//! ```text
//! time   bus 7   bus 13  bus 59  bus 31  bus 19
//! 929      .       .       .       .       .
//! 930      .       .       .       D       .
//! 931      D       .       .       .       D
//! 932      .       .       .       .       .
//! 933      .       .       .       .       .
//! 934      .       .       .       .       .
//! 935      .       .       .       .       .
//! 936      .       D       .       .       .
//! 937      .       .       .       .       .
//! 938      D       .       .       .       .
//! 939      .       .       .       .       .
//! 940      .       .       .       .       .
//! 941      .       .       .       .       .
//! 942      .       .       .       .       .
//! 943      .       .       .       .       .
//! 944      .       .       D       .       .
//! 945      D       .       .       .       .
//! 946      .       .       .       .       .
//! 947      .       .       .       .       .
//! 948      .       .       .       .       .
//! 949      .       D       .       .       .
//! ```
//!
//! The earliest bus you could take is bus ID <code>59</code>. It doesn't depart until timestamp <code>944</code>, so you would need to wait <code>944 - 939 = 5</code> minutes before it departs. Multiplying the bus ID by the number of minutes you'd need to wait gives <em><code>295</code></em>.
//!
//! <em>What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?</em>
//!
//! # --- Part Two ---
//!
//! The shuttle company is running a <span title="This is why you should never let me design a contest for a shuttle company.">contest</span>: one gold coin for anyone that can find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs at that subsequent minute. (The first line in your input is no longer relevant.)
//!
//! For example, suppose you have the same list of bus IDs as above:
//!
//! ```text
//! 7,13,x,x,59,x,31,19```
//!
//! An <code>x</code> in the schedule means there are no constraints on what bus IDs must depart at that time.
//!
//! This means you are looking for the earliest timestamp (called <code>t</code>) such that:
//!
//! - Bus ID 7 departs at timestamp t.
//!
//! - Bus ID 13 departs one minute after timestamp t.
//! - There are no requirements or restrictions on departures at two or three minutes after timestamp t.
//! - Bus ID 59 departs four minutes after timestamp t.
//! - There are no requirements or restrictions on departures at five minutes after timestamp t.
//! - Bus ID 31 departs six minutes after timestamp t.
//! - Bus ID 19 departs seven minutes after timestamp t.
//!
//!
//! The only bus departures that matter are the listed bus IDs at their specific offsets from <code>t</code>. Those bus IDs can depart at other times, and other bus IDs can depart at those times.  For example, in the list above, because bus ID <code>19</code> must depart seven minutes after the timestamp at which bus ID <code>7</code> departs, bus ID <code>7</code> will always <em>also</em> be departing with bus ID <code>19</code> at seven minutes after timestamp <code>t</code>.
//!
//! In this example, the earliest timestamp at which this occurs is <em><code>1068781</code></em>:
//!
//! ```text
//! time     bus 7   bus 13  bus 59  bus 31  bus 19
//! 1068773    .       .       .       .       .
//! 1068774    D       .       .       .       .
//! 1068775    .       .       .       .       .
//! 1068776    .       .       .       .       .
//! 1068777    .       .       .       .       .
//! 1068778    .       .       .       .       .
//! 1068779    .       .       .       .       .
//! 1068780    .       .       .       .       .
//! 1068781    D       .       .       .       .
//! 1068782    .       D       .       .       .
//! 1068783    .       .       .       .       .
//! 1068784    .       .       .       .       .
//! 1068785    .       .       D       .       .
//! 1068786    .       .       .       .       .
//! 1068787    .       .       .       D       .
//! 1068788    D       .       .       .       D
//! 1068789    .       .       .       .       .
//! 1068790    .       .       .       .       .
//! 1068791    .       .       .       .       .
//! 1068792    .       .       .       .       .
//! 1068793    .       .       .       .       .
//! 1068794    .       .       .       .       .
//! 1068795    D       D       .       .       .
//! 1068796    .       .       .       .       .
//! 1068797    .       .       .       .       .
//! ```
//!
//! In the above example, bus ID <code>7</code> departs at timestamp <code>1068788</code> (seven minutes after <code>t</code>). This is fine; the only requirement on that minute is that bus ID <code>19</code> departs then, and it does.
//!
//! Here are some other examples:
//!
//! - The earliest timestamp that matches the list 17,x,13,19 is 3417.
//! - 67,7,59,61 first occurs at timestamp 754018.
//! - 67,x,7,59,61 first occurs at timestamp 779210.
//! - 67,7,x,59,61 first occurs at timestamp 1261476.
//! - 1789,37,47,1889 first occurs at timestamp 1202161486.
//!
//!
//! However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than <code>100000000000000</code>!
//!
//! <em>What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
