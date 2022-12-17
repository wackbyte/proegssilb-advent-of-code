#[allow(unused_imports)]
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::VecDeque;

#[aoc(day06, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let mut i = input
        .iter()
        .enumerate()
        .scan(0u32, |state, (idx, b)| {
            *state = *state << 8 | (*b as u32);
            let [a, b, c, d] = state.to_be_bytes();
            if idx >= 4 && a != b && a != c && a != d && b != c && b != d && c != d {
                return Some(idx + 1);
            } else {
                return Some(0);
            }
        })
        .filter(|x| *x != 0);
    i.next().unwrap()
}

#[test]
pub fn test_part1() {
    assert_eq!(solve_part1(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(solve_part1(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(solve_part1(b"nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(solve_part1(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(solve_part1(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[aoc(day06, part2, devec)]
pub fn solve_part2_devec(input: &[u8]) -> usize {
    let mut i = input
        .iter()
        .enumerate()
        .scan(VecDeque::new(), |state, (idx, b)| {
            state.push_front(*b);
            state.truncate(14);
            if state.len() < 14 {
                return Some(0);
            }
            if state.iter().all_unique() {
                return Some(idx + 1);
            } else {
                return Some(0);
            }
        })
        .filter(|x| *x != 0);
    i.next().unwrap()
}

#[aoc(day06, part2, default)]
pub fn solve_part2_window(input: &[u8]) -> usize {
    let mut i = input.windows(14).position(|w| w.iter().all_unique());
    i.take().unwrap() + 14
}

#[aoc(day06, part2, bitbang)]
pub fn solve_part2_bitbang(input: &[u8]) -> usize {
    let mut i = input
        .iter()
        .enumerate()
        .scan(0u128, |state, (idx, b)| {
            *state = (*state << 8 | *b as u128) & 0x0000_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFu128;
            if idx < 14 {
                return Some(0);
            }
            let [_, _, a, b, c, d, e, f, g, h, i, j, k, l, m, n] = state.to_be_bytes();
            if a != b
                && a != c
                && a != d
                && a != e
                && a != f
                && a != g
                && a != h
                && a != i
                && a != j
                && a != k
                && a != l
                && a != m
                && a != n
                && b != c
                && b != d
                && b != e
                && b != f
                && b != g
                && b != h
                && b != i
                && b != j
                && b != k
                && b != l
                && b != m
                && b != n
                && c != d
                && c != e
                && c != f
                && c != g
                && c != h
                && c != i
                && c != j
                && c != k
                && c != l
                && c != m
                && c != n
                && d != e
                && d != f
                && d != g
                && d != h
                && d != i
                && d != j
                && d != k
                && d != l
                && d != m
                && d != n
                && e != f
                && e != g
                && e != h
                && e != i
                && e != j
                && e != k
                && e != l
                && e != m
                && e != n
                && f != g
                && f != h
                && f != i
                && f != j
                && f != k
                && f != l
                && f != m
                && f != n
                && g != h
                && g != i
                && g != j
                && g != k
                && g != l
                && g != m
                && g != n
                && h != i
                && h != j
                && h != k
                && h != l
                && h != m
                && h != n
                && i != j
                && i != k
                && i != l
                && i != m
                && i != n
                && j != k
                && j != l
                && j != m
                && j != n
                && k != l
                && k != m
                && k != n
                && l != m
                && l != n
                && m != n
            {
                return Some(idx + 1);
            } else {
                return Some(0);
            }
        })
        .filter(|x| *x != 0);
    i.next().unwrap()
}

#[test]
pub fn test_part2_devec() {
    assert_eq!(solve_part2_devec(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(solve_part2_devec(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(solve_part2_devec(b"nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(solve_part2_devec(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(solve_part2_devec(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

#[test]
pub fn test_part2_window() {
    assert_eq!(solve_part2_window(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(solve_part2_window(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(solve_part2_window(b"nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(solve_part2_window(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(solve_part2_window(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

#[test]
pub fn test_part2_bitbang() {
    assert_eq!(solve_part2_bitbang(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(solve_part2_bitbang(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(solve_part2_bitbang(b"nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(
        solve_part2_bitbang(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
        29
    );
    assert_eq!(solve_part2_bitbang(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
