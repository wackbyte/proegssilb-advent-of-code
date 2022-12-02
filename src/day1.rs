use aoc_runner_derive::{aoc_generator, aoc};
use itertools::Itertools;


#[derive(Default, Clone)]
pub struct BackpackCalories {
    total_calories: u64,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<BackpackCalories> {
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
        .filter(|(key, _)| *key == true )
        .map(|(_, group)| {
            let items : Vec<u64> = group.map(|i| i.unwrap()).collect();
            BackpackCalories {
                total_calories: items.iter().sum()
            }
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<BackpackCalories>) -> u64 {
    (*input)
        .clone()
        .into_iter()
        .max_by_key(|bc| bc.total_calories)
        .unwrap_or_default()
        .total_calories
}