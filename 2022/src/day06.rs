use itertools::Itertools;
use std::collections::VecDeque;
use aoc_zen_runner_macros::{aoc, solution};

#[aoc(2022, day06)]
pub mod solutions {
    use super::*;

    #[solution(part1, dotchain)]
    pub fn solve_part1(inp: &str) -> usize {
        let input = inp.as_bytes();
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

    #[solution(part2, devec)]
    pub fn solve_part2_devec(inp: &str) -> usize {
        let input = inp.as_bytes();
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

    #[solution(part2, itertools)]
    pub fn solve_part2_window(inp: &str) -> usize {
        let input = inp.as_bytes();
        let mut i = input.windows(14).position(|w| w.iter().all_unique());
        i.take().unwrap() + 14
    }

    #[solution(part2, bitbang)]
    pub fn solve_part2_bitbang(inp: &str) -> usize {
        let input = inp.as_bytes();
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
}

#[cfg(test)]
mod test {
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(7, 19)]
    const input_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[aoc_case(5, 23)]
    const input_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    #[aoc_case(6, 23)]
    const input_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";

    #[aoc_case(10, 29)]
    const input_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[aoc_case(11, 26)]
    const input_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
}
