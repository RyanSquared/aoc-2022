use std::collections::HashSet;

use aoc_runner_derive::*;
// use anyhow::{anyhow, Error, Result};

/// Given two strs, find all common chars between them.
///
/// # Example
///
/// ```rust
/// # use aoc::day3::*;
/// let lines = "vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw";
/// let compartments = input_generator_part1(lines);
/// let similar_chars = Vec::from_iter(
///     compartments
///         .iter()
///         .filter_map(|(c1, c2)| common_chars(c1.as_str(), c2.as_str()).chars().next())
/// );
/// let expected = vec!['p', 'L', 'P', 'v', 't', 's'];
/// assert_eq!(similar_chars, expected);
///
/// let rucksacks = input_generator_part2(lines);
/// let similar_chars = Vec::from_iter(
///     rucksacks
///         .iter()
///         .map(|(a, b, c)| common_chars(a, common_chars(b, c).as_str()))
///         .filter_map(|s| s.chars().next())
/// );
/// assert_eq!(similar_chars, vec!['r', 'Z']);
pub fn common_chars(c1: &str, c2: &str) -> String {
    let hsa: HashSet<char> = HashSet::from_iter(c1.chars());
    let hsb: HashSet<char> = HashSet::from_iter(c2.chars());
    hsa.intersection(&hsb).collect()
}

/// Generate a Priority from a char
///
/// # Example
///
/// ```rust
/// # use aoc::day3::*;
/// let lines = "vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw";
/// let compartments = input_generator_part1(lines);
/// let priorities = Vec::from_iter(
///     compartments
///         .iter()
///         .filter_map(|(c1, c2)| common_chars(c1.as_str(), c2.as_str()).chars().next())
///         .filter_map(priority)
/// );
/// let expected = vec![16, 38, 42, 22, 20, 19];
/// assert_eq!(priorities, expected);
///
/// let rucksacks = input_generator_part2(lines);
/// let priorities = Vec::from_iter(
///     rucksacks
///         .iter()
///         .map(|(a, b, c)| common_chars(a, common_chars(b, c).as_str()))
///         .filter_map(|s| s.chars().next())
///         .filter_map(priority)
/// );
/// assert_eq!(priorities, vec![18, 52]);
pub fn priority(ch: char) -> Option<u32> {
    // Scores: A-Z => 27-52, a-z => 1-26
    match ch {
        'A'..='Z' => Some(ch as u32 - 'A' as u32 + 27),
        'a'..='z' => Some(ch as u32 - 'a' as u32 + 1),
        _ => None,
    }
}

/// Given an input in the form of lines of chars, split the line equally in half, and place the
/// first (including the middle char) into the first String, and place the second in the second
/// String.
///
/// # Example
///
/// ```rust
/// # use aoc::day3::*;
/// let lines = "vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw";
/// let compartments = input_generator_part1(lines);
/// let expected = vec![
///     ("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string()),
///     ("jqHRNqRjqzjGDLGL".to_string(), "rsFMfFZSrLrFZsSL".to_string()),
///     ("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()),
///     ("wMqvLMZHhHMvwLH".to_string(), "jbvcjnnSBnvTQFn".to_string()),
///     ("ttgJtRGJ".to_string(), "QctTZtZT".to_string()),
///     ("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string()),
/// ];
/// assert_eq!(compartments, expected);
/// ```
#[aoc_generator(day3, part1)]
pub fn input_generator_part1(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            (
                line[0..(line.len() / 2)].to_string(),
                line[(line.len() / 2)..line.len()].to_string(),
            )
        })
        .collect()
}

/// Given an input in the form of chunks of 3 lines of chars, group those chunks together.
///
/// # Example
///
/// ```rust
/// # use aoc::day3::*;
/// let lines = "vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw";
/// let rucksacks = input_generator_part2(lines);
/// let expected = vec![
///     ("vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
///      "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
///      "PmmdzqPrVvPwwTWBwg".to_string()),
///     ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
///      "ttgJtRGJQctTZtZT".to_string(),
///      "CrZsJsPPZsGzwwsLwLmpwMDw".to_string())
/// ];
/// assert_eq!(rucksacks, expected);
#[aoc_generator(day3, part2)]
pub fn input_generator_part2(input: &str) -> Vec<(String, String, String)> {
    // Note: This function is messy. It allocates two vectors because it needs to iterate over
    // slices of the first. This isn't *ideal* but it's not *that* bad.
    let lines = input.lines().map(String::from).collect::<Vec<_>>();
    lines
        .chunks(3)
        .map(|n| match n {
            [l1, l2, l3] => (l1.clone(), l2.clone(), l3.clone()),
            v => panic!("bad input: {v:?}"),
        })
        .collect()
}

#[doc(hidden)]
#[aoc(day3, part1)]
pub fn solve_part1(compartments: &[(String, String)]) -> String {
    compartments
        .iter()
        .filter_map(|(c1, c2)| common_chars(c1.as_str(), c2.as_str()).chars().next())
        .filter_map(priority)
        .sum::<u32>()
        .to_string()
}

#[doc(hidden)]
#[aoc(day3, part2)]
pub fn solve_part2(rucksacks: &[(String, String, String)]) -> String {
    rucksacks
        .iter()
        .map(|(a, b, c)| common_chars(a, common_chars(b, c).as_str()))
        .filter_map(|s| s.chars().next())
        .filter_map(priority)
        .sum::<u32>()
        .to_string()
}
