#[allow(unused_imports)]
use std::cmp::max;
use aoc_runner_derive::{aoc_generator, aoc};
use itertools::Itertools;

pub type GenData = Vec<i32>;
pub type InData<'a> = &'a [i32];
pub type OutData = u64;


// Solution ---------------------------------------------------------
// Choose One

// #[aoc_generator(day17)]
// pub fn input_generator(input: &str) -> GenData {
//     let input = input.trim_start();
//     todo!()
// }

// #[aoc(day17, part1)]
// pub fn solve_part1(input: InData) -> OutData {
//     todo!()
// }

// #[aoc(day17, part2)]
// pub fn solve_part2(input: InData) -> OutData {
//     todo!()
// }

// #[allow(unused)]
// const TEST_IN: &str = r#"
// "#;

// #[test]
// pub fn test_part1() {
//     assert_eq!(solve_part1(&input_generator(TEST_IN)), _Y);
// }

// #[test]
// pub fn test_part2() {
//     assert_eq!(solve_part2(&input_generator(TEST_IN)), _Z);
// }

// ------------- Or -------------

// #[aoc(day17, part1)]
// pub fn solve_part1(input: &str) -> OutData {
//     let input = input.trim_start();
// }

// #[aoc(day17, part2)]
// pub fn solve_part2(input: &str) -> OutData {
//     let input = input.trim_start();
// }

// #[allow(unused)]
// const TEST_IN: &str = r#"
// "#;

// #[test]
// pub fn test_part1() {
//     assert_eq!(solve_part1(TEST_IN), _Y);
// }

// #[test]
// pub fn test_part2() {
//     assert_eq!(solve_part2(TEST_IN), _Z);
// }