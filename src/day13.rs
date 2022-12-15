use std::{cmp::{Ordering, min}, fmt::Display};
#[allow(unused_imports)]
use std::cmp::max;
use aoc_runner_derive::{aoc_generator, aoc};
use itertools::{Itertools, EitherOrBoth};
use serde::{Serialize, Deserialize};
use serde_json::from_str;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum IntOrList {
    Int(i8),
    List(Vec<IntOrList>),
}

use IntOrList::*;

impl Ord for IntOrList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Rules are quoted from AoC website for reference.
        // Code assumes pairs will be checked against `<=`
        match (self, other) {
            /*
                If both values are integers, the lower integer should come first. 
                If the left integer is lower than the right integer, the inputs 
                are in the right order. If the left integer is higher than the 
                right integer, the inputs are not in the right order. Otherwise,
                the inputs are the same integer; continue checking the next part
                of the input.
            */
            (Int(s), Int(o)) => s.cmp(o),
            /*
                If both values are lists, compare the first value of each list,
                then the second value, and so on. If the left list runs out of
                items first, the inputs are in the right order. If the right
                list runs out of items first, the inputs are not in the right
                order. If the lists are the same length and no comparison makes
                a decision about the order, continue checking the next part of
                the input.
             */
            (List(s), List(o)) => {
                for i in s.iter().zip_longest(o) {
                    match i {
                        EitherOrBoth::Both(a, b) if a.eq(b) => { continue; },
                        EitherOrBoth::Both(a, b) => {
                            let res = a.cmp(b);
                            if res == Ordering::Equal {
                                continue;
                            } else {
                                return res;
                            }
                        },
                        EitherOrBoth::Left(_) => { return Ordering::Greater; },
                        EitherOrBoth::Right(_) => { return Ordering::Less; }
                    };
                }
                Ordering::Equal
            }
            /*
                If exactly one value is an integer, convert the integer to a list
                which contains that integer as its only value, then retry the 
                comparison. For example, if comparing [0,0,0] and 2, convert the
                right value to [2] (a list containing 2); the result is then found
                by instead comparing [0,0,0] and [2].
             */
            (Int(s), o@List(_)) => List(vec![Int(*s)]).cmp(o),
            (s@List(_), Int(o)) => s.cmp(&List(vec![Int(*o)])),
        }
    }
}

impl PartialOrd for IntOrList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for IntOrList {
    fn default() -> Self {
        Int(0)
    }
}

impl Display for IntOrList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Int(a) => a.fmt(f),
            List(ls) => {
                write!(f, "{}", "[")?;
                for (idx, i) in ls.iter().enumerate() {
                    i.fmt(f)?;
                    if idx < ls.len() - 1 {
                        write!(f, "{}", ", ")?;
                    }
                }
                write!(f, "{}", "]")?;
                Ok(())
            }
        }
    }
}

pub type GenData = Vec<(IntOrList, IntOrList)>;
pub type InData<'a> = &'a [(IntOrList, IntOrList)];
pub type OutData = usize;


// Solution ---------------------------------------------------------
// Choose One

#[aoc_generator(day13, part1)]
pub fn input_generator(input: &str) -> GenData {
    let input = input.trim_start();
    let pair_list = input.split("\n\n");
    pair_list
        .map(|s| s.split_once("\n").unwrap())
        .map(|(a, b)| (from_str(a).unwrap(),
                                   from_str(b).unwrap()))
        .collect_vec()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: InData) -> OutData {
    input.iter()
        // .inspect(|(a, b)| {
        //     println!("a: {}\nb: {}\nOrd: {:?}\n\n", a, b, a.partial_cmp(b))
        // })
        .enumerate()
        .filter(|(_, (a,b))| a <= b)
        .map(|(idx, _)| idx + 1)
        .sum()
}

#[aoc_generator(day13, part2)]
pub fn input_generator_p2(input: &str) -> Vec<IntOrList> {
    let input = input.to_owned() + "\n[[2]]\n[[6]]\n";
    input.lines()
        .filter(|s| s.trim() != "")
        .map(|s| from_str(s).unwrap())
        .collect_vec()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[IntOrList]) -> OutData {
    let mut input = input.to_vec();
    input.sort();
    let mut two_idx: usize = input.len() + 3;
    let mut six_idx: usize = input.len() + 3;

    for (idx, i) in input.iter().enumerate() {
        if *i == List(vec![List(vec![Int(2)])]) {
            two_idx = min(two_idx, idx);
        }
        if *i == List(vec![List(vec![Int(6)])]) {
            six_idx = min(six_idx, idx);
        }
    }

    two_idx = two_idx + 1;
    six_idx = six_idx + 1;

    two_idx * six_idx
}

#[allow(unused)]
const TEST_IN: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

#[test]
pub fn test_d13_part1() {
    assert_eq!(solve_part1(&input_generator(TEST_IN)), 13);
}

#[test]
pub fn test_d13_part2() {
    assert_eq!(solve_part2(&input_generator_p2(TEST_IN)), 140);
}
