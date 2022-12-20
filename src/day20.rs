#[allow(unused_imports)]
use std::cmp::max;
use aoc_runner_derive::aoc;
use itertools::Itertools;

pub type GenData = Vec<i32>;
pub type InData<'a> = &'a [i32];
pub type OutData = i32;


// Solution ---------------------------------------------------------

#[aoc(day20, part1)]
pub fn solve_part1(input: &str) -> OutData {
    let mut input = input.lines().filter(|ln| ln.trim() != "").map(|ln| ln.trim().parse::<i32>().unwrap()).enumerate().collect_vec();
    for old_idx in 0..input.len() {
        //println!("State: [{}]", &input.iter().map(|(_, val)| val.to_string()).join(", "));
        let (curr_idx, _) = input.iter().find_position(|(oi, _)| *oi == old_idx ).unwrap();
        let elem = input.remove(curr_idx);
        let new_idx = (curr_idx as i32 + elem.1).rem_euclid(input.len() as i32);
        //println!("Moving {} from {} to {}, between {} and {}.", elem.1, elem.0, new_idx, input[]);
        input.insert(new_idx as usize, elem)
    }

    let (zero_idx, _) = input.iter().find_position(|(_, i)| *i == 0).unwrap();

    let first_var = input[(zero_idx + 1000).rem_euclid(input.len()) as usize].1;
    let second_var = input[(zero_idx + 2000).rem_euclid(input.len()) as usize].1;
    let third_var = input[(zero_idx + 3000).rem_euclid(input.len()) as usize].1;

    first_var + second_var + third_var
}

// #[aoc(day20, part2)]
// pub fn solve_part2(input: &str) -> OutData {
//     let input = input.trim_start();
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_IN: &str = r#"
1
2
-3
3
-2
0
4
"#;

    #[test]
    pub fn test_part1() {
        assert_eq!(solve_part1(TEST_IN), 3);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(solve_part2(TEST_IN), _Z);
    // }
}
