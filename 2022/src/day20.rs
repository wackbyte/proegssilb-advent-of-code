use aoc_zen_runner_macros::{aoc, solution};
use itertools::Itertools;

#[aoc(2022, day20)]
pub mod solutions {
    pub type GenData = Vec<i32>;
    pub type OutData = i64;

    use super::*;
    fn mix(input: &[i64], num_rounds: u32) -> Vec<i64> {
        let mut input = input.iter().enumerate().collect_vec();
        for _ in 0..num_rounds {
            for old_idx in 0..input.len() {
                let (curr_idx, _) = input
                    .iter()
                    .find_position(|(oi, _)| *oi == old_idx)
                    .unwrap();
                let elem = input.remove(curr_idx);
                let new_idx = (curr_idx as i64 + elem.1).rem_euclid(input.len() as i64);
                input.insert(new_idx as usize, elem)
            }
        }

        input.into_iter().map(|(_, val)| *val).collect_vec()
    }

    #[solution(part1, draft)]
    pub fn solve_part1(input: &str) -> OutData {
        let input = input
            .lines()
            .filter(|ln| ln.trim() != "")
            .map(|ln| ln.trim().parse::<i64>().unwrap())
            .collect_vec();

        let mixed = mix(&input, 1);

        let (zero_idx, _) = mixed.iter().find_position(|i| **i == 0).unwrap();

        let first_var = mixed[(zero_idx + 1000).rem_euclid(input.len()) as usize];
        let second_var = mixed[(zero_idx + 2000).rem_euclid(input.len()) as usize];
        let third_var = mixed[(zero_idx + 3000).rem_euclid(input.len()) as usize];

        first_var + second_var + third_var
    }

    #[solution(part2, draft)]
    pub fn solve_part2(input: &str) -> OutData {
        let input = input
            .lines()
            .filter(|ln| ln.trim() != "")
            .map(|ln| ln.trim().parse::<i64>().unwrap())
            .map(|x| x * 811589153)
            .collect_vec();

        let mixed = mix(&input, 10);

        let (zero_idx, _) = mixed.iter().find_position(|i| **i == 0).unwrap();

        let first_var = mixed[(zero_idx + 1000).rem_euclid(input.len()) as usize];
        let second_var = mixed[(zero_idx + 2000).rem_euclid(input.len()) as usize];
        let third_var = mixed[(zero_idx + 3000).rem_euclid(input.len()) as usize];

        first_var + second_var + third_var
    }
}

#[cfg(test)]
mod test {
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(3, 1623178306)]
    const test_in: &str = r#"
1
2
-3
3
-2
0
4
"#;
}
