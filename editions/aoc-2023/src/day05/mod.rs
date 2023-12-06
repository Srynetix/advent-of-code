//! # --- Day 5: If You Give A Seed A Fertilizer ---
//!
//! > _Exercise page: <https://adventofcode.com/2023/day/5>_
//!
//! > _Input page: <https://adventofcode.com/2023/day/5/input>_
//!
//! You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.
//!
//! "A water source? Island Island <em>is</em> the water source!" You point out that Snow Island isn't receiving any water.
//!
//! "Oh, we had to stop the water because we <em>ran out of sand</em> to <a target="_blank" href="https://en.wikipedia.org/wiki/Sand_filter">filter</a> it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.
//!
//! "I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"
//!
//! You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our <em>food production problem</em>. The latest Island Island <a href="https://en.wikipedia.org/wiki/Almanac" target="_blank">Almanac</a> just arrived and we're having trouble making sense of it."
//!
//! The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil <code>123</code> and fertilizer <code>123</code> aren't necessarily related to each other.
//!
//! For example:
//!
//! ```text
//! seeds: 79 14 55 13
//!
//! seed-to-soil map:
//! 50 98 2
//! 52 50 48
//!
//! soil-to-fertilizer map:
//! 0 15 37
//! 37 52 2
//! 39 0 15
//!
//! fertilizer-to-water map:
//! 49 53 8
//! 0 11 42
//! 42 0 7
//! 57 7 4
//!
//! water-to-light map:
//! 88 18 7
//! 18 25 70
//!
//! light-to-temperature map:
//! 45 77 23
//! 81 45 19
//! 68 64 13
//!
//! temperature-to-humidity map:
//! 0 69 1
//! 1 0 69
//!
//! humidity-to-location map:
//! 60 56 37
//! 56 93 4
//! ```
//!
//! The almanac starts by listing which seeds need to be planted: seeds <code>79</code>, <code>14</code>, <code>55</code>, and <code>13</code>.
//!
//! The rest of the almanac contains a list of <em>maps</em> which describe how to convert numbers from a <em>source category</em> into numbers in a <em>destination category</em>. That is, the section that starts with <code>seed-to-soil map:</code> describes how to convert a <em>seed number</em> (the source) to a <em>soil number</em> (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.
//!
//! Rather than list every source number and its corresponding destination number one by one, the maps describe entire <em>ranges</em> of numbers that can be converted. Each line within a map contains <span title="Don't blame me for the weird order. Blame LXC container.conf UID mappings.">three numbers</span>: the <em>destination range start</em>, the <em>source range start</em>, and the <em>range length</em>.
//!
//! Consider again the example <code>seed-to-soil map</code>:
//!
//! ```text
//! 50 98 2
//! 52 50 48
//! ```
//!
//! The first line has a <em>destination range start</em> of <code>50</code>, a <em>source range start</em> of <code>98</code>, and a <em>range length</em> of <code>2</code>. This line means that the source range starts at <code>98</code> and contains two values: <code>98</code> and <code>99</code>. The destination range is the same length, but it starts at <code>50</code>, so its two values are <code>50</code> and <code>51</code>. With this information, you know that seed number <code>98</code> corresponds to soil number <code>50</code> and that seed number <code>99</code> corresponds to soil number <code>51</code>.
//!
//! The second line means that the source range starts at <code>50</code> and contains <code>48</code> values: <code>50</code>, <code>51</code>, ..., <code>96</code>, <code>97</code>. This corresponds to a destination range starting at <code>52</code> and also containing <code>48</code> values: <code>52</code>, <code>53</code>, ..., <code>98</code>, <code>99</code>. So, seed number <code>53</code> corresponds to soil number <code>55</code>.
//!
//! Any source numbers that <em>aren't mapped</em> correspond to the <em>same</em> destination number. So, seed number <code>10</code> corresponds to soil number <code>10</code>.
//!
//! So, the entire list of seed numbers and their corresponding soil numbers looks like this:
//!
//! ```text
//! seed  soil
//! 0     0
//! 1     1
//! ...   ...
//! 48    48
//! 49    49
//! 50    52
//! 51    53
//! ...   ...
//! 96    98
//! 97    99
//! 98    50
//! 99    51
//! ```
//!
//! With this map, you can look up the soil number required for each initial seed number:
//!
//! - Seed number 79 corresponds to soil number 81.
//! - Seed number 14 corresponds to soil number 14.
//! - Seed number 55 corresponds to soil number 57.
//! - Seed number 13 corresponds to soil number 13.
//!
//!
//! The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find <em>the lowest location number that corresponds to any of the initial seeds</em>. To do this, you'll need to convert each seed number through other categories until you can find its corresponding <em>location number</em>. In this example, the corresponding types are:
//!
//! - Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
//! - Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
//! - Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
//! - Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
//!
//!
//! So, the lowest location number in this example is <code><em>35</em></code>.
//!
//! <em>What is the lowest location number that corresponds to any of the initial seed numbers?</em>
//!
//! # --- Part Two ---
//!
//! Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the <code>seeds:</code> line actually describes <em>ranges of seed numbers</em>.
//!
//! The values on the initial <code>seeds:</code> line come in pairs. Within each pair, the first value is the <em>start</em> of the range and the second value is the <em>length</em> of the range. So, in the first line of the example above:
//!
//! ```text
//! seeds: 79 14 55 13```
//!
//! This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number <code>79</code> and contains <code>14</code> values: <code>79</code>, <code>80</code>, ..., <code>91</code>, <code>92</code>. The second range starts with seed number <code>55</code> and contains <code>13</code> values: <code>55</code>, <code>56</code>, ..., <code>66</code>, <code>67</code>.
//!
//! Now, rather than considering four seed numbers, you need to consider a total of <em>27</em> seed numbers.
//!
//! In the above example, the lowest location number can be obtained from seed number <code>82</code>, which corresponds to soil <code>84</code>, fertilizer <code>84</code>, water <code>84</code>, light <code>77</code>, temperature <code>45</code>, humidity <code>46</code>, and <em>location <code>46</code></em>. So, the lowest location number is <code><em>46</em></code>.
//!
//! Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. <em>What is the lowest location number that corresponds to any of the initial seed numbers?</em>

pub mod common;
pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");
