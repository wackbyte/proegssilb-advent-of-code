use aoc_zen_runner_macros::{aoc, generator, solver};
use std::collections::BinaryHeap;

#[aoc(2022, day01)]
pub mod solutions {
    use super::*;

    #[generator(trimparse)]
    pub fn input_generator(input: &str) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();
        let mut current_sum: i32 = 0;
        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                results.push(current_sum);
                current_sum = 0;
            } else {
                current_sum += trimmed.parse::<i32>().unwrap();
            }
        }
        results
    }

    #[solver(part1, iter)]
    pub fn solve_part1(input: Vec<i32>) -> i32 {
        *input.iter().max().unwrap()
    }

    #[solver(part2, heap)]
    pub fn solve_part2(input: Vec<i32>) -> i32 {
        let mut results = BinaryHeap::new();
        for backpack in input {
            results.push(backpack);
            if results.len() > 3 {
                _ = results.pop();
            }
        }

        results.into_iter().sum()
    }
}
