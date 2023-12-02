use aoc_zen_runner_macros::{solution, aoc};
use std::collections::HashMap;
use bstr::ByteSlice;

#[aoc(2023, day1)]
pub mod solutions {
    use super::*;

    // ----------------------- Part 1 -----------------------
    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> u32 {
        let mut items = vec![];
        let input = input.trim();
        for l in input.lines() {
            let chars: Vec<char> = l.chars().filter(|c| c.is_numeric()).collect();
            if chars.len() == 0 {
                unreachable!()
            }
            let s: String = [chars[0], chars[chars.len() - 1]].iter().collect();
            let v = s.parse::<u32>().unwrap();
            items.push(v);
        }

        let rv = items.iter().sum();
        rv
    }

    #[solution(part1, no_copy)]
    pub fn part1_nocopy(input: &str) -> u32 {
        let mut running_sum = 0u32;
        let target_range = b'0'..=b'9';
        for li in input.lines() {
            let line = li.as_bytes();

            let c1: u8 = *line.iter().find(|c| target_range.contains(c)).expect("Forward find failed to find a match.");
            let c2: u8 = *line.iter().rfind(|c| target_range.contains(c)).expect("Reverse find failed to find a match.");

            let s = 10*(c1-b'0') + (c2 - b'0');
            running_sum += s as u32;
        }

        running_sum
    }

    // ---- These two fail testing for reasons I don't care to work through ----
    // #[solution(part1, no_range)]
    // pub fn part1_norange(input: &str) -> u32 {
    //     let mut running_sum = 0u32;
    //     for li in input.lines() {
    //         let line = li.as_bytes();

    //         let c1: u8 = *line.iter().find(|&c| c <= &b'9').expect("Forward find failed to find a match.");
    //         let c2: u8 = *line.iter().rfind(|&c| c <= &b'9').expect("Reverse find failed to find a match.");

    //         let s = 10*(c1-b'0') + (c2 - b'0');
    //         running_sum += s as u32;
    //     }

    //     running_sum
    // }

    // #[solution(part1, multicounter)]
    // pub fn part1_multicounter(input: &str) -> u32 {
    //     let mut ones = 0u32;
    //     let mut tens = 0u32;
    //     let mut lines = 0u32;
    //     for li in input.lines() {
    //         let line = li.as_bytes();

    //         let c1: u8 = *line.iter().find(|&c| c <= &b'9').expect("Forward find failed to find a match.");
    //         let c2: u8 = *line.iter().rfind(|&c| c <= &b'9').expect("Reverse find failed to find a match.");

    //         tens += c1 as u32;
    //         ones += c2 as u32;
    //         lines += 1;
    //     }

    //     let comp = lines * (b'0' as u32);
    //     10*(tens-comp) + ones - comp
    // }

    // ----------------------- Part 2 -----------------------
    #[allow(dead_code)]
    fn substitute_word(item: &str) -> &str {
        match item {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => item,
        }
    }

    pub fn replace_first(text: &str, replacements: &HashMap<&str, &str>) -> String {
        if text.len() <= 1 {
            return text.to_string();
        }

        for (k, v) in replacements.iter() {
            if text.starts_with(k) {
                let (_, rest) = text.split_at(k.len());
                return v.to_string() + &rest;
            }
            // Else, continue.
        }

        // Remove one character, then recurse.
        let (c, rest) = text.split_at(1);
        return c.to_string() + &replace_first(rest, replacements);
    }

    pub fn replace_last(text: &str, replacements: &HashMap<&str, &str>) -> String {
        if text.len() <= 1 {
            return text.to_string();
        }

        for (k, v) in replacements.iter() {
            if text.ends_with(k) {
                let (start, _) = text.split_at(text.len() - k.len());
                return start.to_string() + v;
            }
            // Else, continue.
        }

        // Remove one character, then recurse.
        let (start, w) = text.split_at(text.len() - 1);
        return replace_last(start, replacements) + w;
    }

    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> u32 {
        let inp = input.trim();
        let replacement_table = HashMap::from([
            ("one", "one1one"),
            ("two", "two2two"),
            ("three", "three3three"),
            ("four", "four4four"),
            ("five", "five5five"),
            ("six", "six6six"),
            ("seven", "seven7seven"),
            ("eight", "eight8eight"),
            ("nine", "nine9nine"),
        ]);

        let mut items = vec![];

        for orig_line in inp.lines() {
            let line1 = replace_first(orig_line, &replacement_table);
            let line2 = replace_last(&line1, &replacement_table);

            let chars: Vec<char> = line2.chars().filter(|c| c.is_numeric()).collect();
            if chars.len() == 0 {
                unreachable!()
            }
            let s: String = [chars[0], chars[chars.len() - 1]].iter().collect();
            let v = s.parse::<u32>().unwrap();
            items.push(v);
        }

        items.iter().sum()
    }

    pub fn find_number_at_start(text: &[u8]) -> Option<usize> {
        const FIND: [&[u8]; 18] = [b"one", b"1", b"two", b"2", b"three", b"3", b"four", b"4", b"five", b"5", b"six", b"6", b"seven", b"7", b"eight", b"8", b"nine", b"9"];

        for (idx, f) in FIND.iter().enumerate() {
            if text.starts_with(f) {
                return Some((idx >> 1) + 1);
            }
        }

        None
    }

    #[solution(part2, startswith)]
    pub fn part2_startswith(input: &str) -> u32 {
        let mut tens = 0u32;
        let mut ones = 0u32;

        for ln in input.as_bytes().lines() {

            for i in 0..ln.len() {
                let text = &ln[i..];
                if let Some(i) = find_number_at_start(text) {
                    tens += i as u32;
                    break;
                }
            }

            for i in (0..ln.len()).rev() {
                let text = &ln[i..];
                if let Some(i) = find_number_at_start(text) {
                    ones += i as u32;
                    break;
                }
            }
        }

        tens*10 + ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::solutions::*;
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(142, 142)]
    const INPUT1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const INPUT2: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn test_p2() {
        assert_eq!(281, part2_draft(INPUT2));
        assert_eq!(22, part2_draft("2tqbxgrrpmxqfglsqjkqthree6nhjvbxpflhr1eightwohr"));
        assert_eq!(81, part2_draft("tqbeightwoxgrrpmxqfglsqjkqthree6nhjvbxpflhr1twonehr"));
        assert_eq!(91, part2_draft("tqbei9ghtwoxgrrpmxqfglsqjkqthree6nhjvbxpflhr1eightw1ohr"));
        assert_eq!(96, part2_draft("nine7twoslseven4sfoursix"));
        assert_eq!(53, part2_draft("five6mnjxbrnsvltwo3"));
        assert_eq!(62, part2_draft("6fivemnjxbrnsvl3two"));
        assert_eq!(22, part2_draft("two2two"));
        assert_eq!(57, part2_draft("527"));
        assert_eq!(11, part2_draft("h1"));
        assert_eq!(83, part2_draft("eighthree"));
        assert_eq!(79, part2_draft("sevenine"));
    }

    #[test]
    fn test_replacement() {
        let replacement_table = HashMap::from([
            ("one", "one1one"),
            ("two", "two2two"),
            ("three", "three3three"),
            ("four", "four4four"),
            ("five", "five5five"),
            ("six", "six6six"),
            ("seven", "seven7seven"),
            ("eight", "eight8eight"),
            ("nine", "nine9nine"),
        ]);

        assert_eq!("eight8eightwothree", replace_first("eightwothree", &replacement_table));
        assert_eq!("eightwothree3three", replace_last("eightwothree", &replacement_table));
        assert_eq!("2tqbxgrrpmxqfglsqjkqthree6nhjvbxpflhr1eightwo2twohr", replace_last("2tqbxgrrpmxqfglsqjkqthree6nhjvbxpflhr1eightwohr", &replacement_table));
    }

    #[test]
    fn test_startswith() {
        assert_eq!(Some(1), find_number_at_start(b"1bc2"));
        assert_eq!(None, find_number_at_start(b"pqr3stu8vwx"));
        assert_eq!(None, find_number_at_start(b"qr3stu8vwx"));
        assert_eq!(None, find_number_at_start(b"r3stu8vwx"));
        assert_eq!(Some(3), find_number_at_start(b"3stu8vwx"));
        assert_eq!(None, find_number_at_start(b"a1b2c3d4e5f"));
        assert_eq!(Some(1), find_number_at_start(b"1b2c3d4e5f"));
    }
}
