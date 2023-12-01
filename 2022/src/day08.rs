use aoc_zen_runner_macros::{aoc, solution};
use itertools::Itertools;
use std::cmp::max;

#[aoc(2022, day08)]
pub mod solutions {
    use super::*;
    
    pub type Day8Output = u64;

    #[solution(part1, draft)]
    pub fn solve_part1(input: &str) -> Day8Output {
        let grid = input.lines().map(|s| s.bytes().collect_vec()).collect_vec();
        let mut count = 0u64;
        for (row_num, row) in grid.iter().enumerate() {
            if row_num == 0 || row_num == grid.len() - 1 {
                count += row.len() as u64;
                continue;
            }
            for (col_num, cell) in row.iter().enumerate() {
                if col_num == 0 || col_num == row.len() - 1 {
                    count += 1;
                    continue;
                }
                // Up
                let mut vis = true;
                for r in 0..row_num {
                    vis = vis && grid[r][col_num] < *cell;
                }
                if vis {
                    count += 1;
                    continue;
                }

                // Down
                let mut vis = true;
                for r in row_num + 1..grid.len() {
                    vis = vis && grid[r][col_num] < *cell;
                }
                if vis {
                    count += 1;
                    continue;
                }

                // Left
                let mut vis = true;
                for c in 0..col_num {
                    vis = vis && grid[row_num][c] < *cell;
                }
                if vis {
                    count += 1;
                    continue;
                }

                // Right
                let mut vis = true;
                for c in col_num + 1..row.len() {
                    vis = vis && grid[row_num][c] < *cell;
                }
                if vis {
                    count += 1;
                    continue;
                }
            }
        }

        count
    }

    pub fn scenic_score(grid: &Vec<Vec<u8>>, row_num: usize, col_num: usize) -> u64 {
        let mut score = 1u64;
        let row = &grid[row_num];
        let cell = row[col_num];

        if row_num == 0 || col_num == 0 || row_num == grid.len() || col_num == row.len() {
            return 0;
        }

        // Up
        let mut subscore = 0u64;
        for r in (0..row_num).rev() {
            let target = grid[r][col_num];
            subscore += 1;
            if target >= cell {
                break;
            }
        }
        score *= subscore;

        // Down
        let mut subscore = 0u64;
        for r in row_num + 1..grid.len() {
            let target = grid[r][col_num];
            subscore += 1;
            if target >= cell {
                break;
            }
        }
        score *= subscore;

        // Left
        let mut subscore = 0u64;
        for c in (0..col_num).rev() {
            let target = grid[row_num][c];
            subscore += 1;
            if target >= cell {
                break;
            }
        }
        score *= subscore;

        // Right
        let mut subscore = 0u64;
        for c in col_num + 1..row.len() {
            let target = grid[row_num][c];
            subscore += 1;
            if target >= cell {
                break;
            }
        }
        score *= subscore;

        score
    }

    #[solution(part2, nestedfor)]
    pub fn solve_part2(input: &str) -> Day8Output {
        let grid = input.lines().map(|s| s.bytes().collect_vec()).collect_vec();
        let mut max_score = 0u64;
        for (row_num, row) in grid.iter().enumerate() {
            for (col_num, _cell) in row.iter().enumerate() {
                let score = scenic_score(&grid, row_num, col_num);
                max_score = max(max_score, score);
            }
        }

        max_score
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use super::solutions::*;
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(21, 8)]
    const TEST_IN: &str = r#"30373
25512
65332
33549
35390
"#;

    #[test]
    pub fn test_score() {
        let test_in: &str = r#"30373
        25512
        65332
        33549
        35390
        "#;
        let grid = test_in
            .lines()
            .map(|s| s.bytes().collect_vec())
            .collect_vec();
        assert_eq!(scenic_score(&grid, 1, 2), 4);
        assert_eq!(scenic_score(&grid, 3, 2), 8);
        assert_eq!(solve_part2(test_in), 8);
    }
}
