use aoc_zen_runner_macros::{aoc, solution};
use itertools::Itertools;

fn find_error(rucksacks: &str) -> impl Iterator<Item = &u8> {
    let half = rucksacks.len() / 2;
    let [left, right] = [rucksacks[..half].as_bytes(), rucksacks[half..].as_bytes()];
    left.iter().unique().chain(right.iter().unique()).duplicates()
}

fn error_priority(error: &u8) -> i32 {
    match error {
        b'a'..=b'z' => (error - b'a' + 1) as i32,
        b'A'..=b'Z' => (error - b'A' + 27) as i32,
        _ => panic!("Incorrect error found: {:?}", error),
    }
}

#[aoc(2022, day03)]
pub mod solutions {
    use super::*;

    #[solution(part1, flatmapmap)]
    pub fn solve_part1(input: &str) -> i32 {
        input.lines().flat_map(find_error).map(error_priority).sum()
    }

    #[solution(part2, iterharder)]
    pub fn solve_part2(input: &str) -> i32 {
        let groups = input.lines().map(|l| l.as_bytes()).chunks(3);

        let mut priority_sum = 0;

        'groups: for elf_group in &groups {
            let [group1, group2, group3] = elf_group.collect_vec()[..] else {
                panic!("Asked for chunks of 3, did not get chunks of three.");
            };

            for item_type in group1.iter().unique() {
                if group2.contains(item_type) && group3.contains(item_type) {
                    priority_sum += error_priority(item_type);
                    continue 'groups;
                }
            }
        }

        priority_sum
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;

    #[test]
    fn test_find_error() {
        assert_eq!(find_error("vJrwpWtwJgWrhcsFMMfFFhFp").collect_vec(), vec![&b'p']);
        assert_eq!(
            find_error("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").collect_vec(),
            vec![&b'L']
        );
        assert_eq!(find_error("PmmdzqPrVvPwwTWBwg").collect_vec(), vec![&b'P']);
        assert_eq!(find_error("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").collect_vec(), vec![&b'v']);
        assert_eq!(find_error("ttgJtRGJQctTZtZT").collect_vec(), vec![&b't']);
        assert_eq!(find_error("CrZsJsPPZsGzwwsLwLmpwMDw").collect_vec(), vec![&b's']);
    }
}
