use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type CampRange = RangeInclusive<i32>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(CampRange, CampRange)> {
    input.lines()
        .map(|l| l.split(',').map(|r| {
            let (a, b) = r.split('-').collect_tuple::<(&str, &str)>().unwrap_or_else(|| panic!("Invalid range found: {}", r));
            RangeInclusive::new(a.parse().unwrap(), b.parse().unwrap())
        }).collect_tuple().unwrap())
        .collect_vec()
}

fn range_either_contains(a: &CampRange, b: &CampRange) -> i32 {
    let res = (a.contains(b.start()) && a.contains(b.end())) || (b.contains(a.start()) && b.contains(a.end()));
    res as i32
}

fn range_overlaps(a: &CampRange, b: &CampRange) -> i32 {
    let res = !(a.end() < b.start() || b.end() < a.start());
    res as i32
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[(CampRange, CampRange)]) -> i32 {
    input
        .iter()
        .map(|(a, b)|range_either_contains(a, b))
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[(CampRange, CampRange)]) -> i32 {
    input
        .iter()
        .map(|(a, b)|range_overlaps(a, b))
        .sum()
}

