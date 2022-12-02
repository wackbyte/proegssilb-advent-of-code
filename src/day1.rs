use aoc_runner_derive::{aoc_generator, aoc};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            match line.trim() {
                "" => None,
                _ => Some(line.trim().parse::<u64>().unwrap()),
            }
        })
        .group_by(|x| x.is_some())
        .into_iter()
        .filter(|(key, _)| *key )
        .map(|(_, group)| {
            let items : Vec<u64> = group.map(|i| i.unwrap()).collect();
            items.iter().sum()
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u64>) -> u64 {
    (*input)
        .clone()
        .into_iter()
        .max()
        .unwrap_or_default()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<u64>) -> u64 {
    let mut max_backpacks = (*input)
        .clone();
    max_backpacks.sort_by_key(|i| -(*i as i64));
    max_backpacks[0..3].iter().sum()
}