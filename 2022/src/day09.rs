use aoc_zen_runner_macros::{aoc, generator, solver};
use itertools::{Itertools, MinMaxResult};
use std::collections::HashSet;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Movement {
    dir: Direction,
    count: u8,
}

// Solution ---------------------------------------------------------
// Choose One

#[aoc(2022, day9)]
pub mod solutions {
    use super::*;

    pub type GenData = Vec<Movement>;
    pub type OutData = usize;

    #[generator(draft)]
    pub fn input_generator(input: &str) -> GenData {
        let mut results: GenData = Vec::new();
        for line in input.lines() {
            if line.trim() == "" {
                continue;
            }
            let (dir_b, count_str) = line.split_once(" ").unwrap();
            let count: u8 = str::parse::<u8>(count_str).unwrap();
            let instruction = match dir_b {
                "R" => Movement {
                    dir: Direction::Right,
                    count,
                },
                "L" => Movement {
                    dir: Direction::Left,
                    count,
                },
                "D" => Movement {
                    dir: Direction::Down,
                    count,
                },
                "U" => Movement {
                    dir: Direction::Up,
                    count,
                },
                _ => panic!("Invalid movement direction: {}", line),
            };
            results.push(instruction);
        }

        results
    }

    #[solver(part1, draft)]
    pub fn solve_part1(input: GenData) -> OutData {
        let mut h_loc: (i32, i32) = (0, 0);
        let mut t_loc: (i32, i32) = (0, 0);
        let mut tail_coords: HashSet<(i32, i32)> = HashSet::new();
        tail_coords.insert((0, 0));
        for Movement { dir, count } in input {
            let offset = match dir {
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
                Direction::Down => (0, -1),
                Direction::Up => (0, 1),
            };
            for _ in 0..count {
                h_loc = (h_loc.0 + offset.0, h_loc.1 + offset.1);
                let delta: (i32, i32) = (h_loc.0 - t_loc.0, h_loc.1 - t_loc.1);
                match (delta.0.abs(), delta.1.abs()) {
                    (1, 0) | (0, 1) | (1, 1) | (0, 0) => {
                        continue;
                    }
                    (_, 0) => {
                        t_loc.0 += delta.0.signum();
                    }
                    (0, _) => {
                        t_loc.1 += delta.1.signum();
                    }
                    (_, _) => {
                        t_loc.0 += delta.0.signum();
                        t_loc.1 += delta.1.signum();
                    }
                }
                tail_coords.insert(t_loc);
            }
        }

        tail_coords.len()
    }

    #[allow(unused)]
    fn draw_rope(rope: &[(i32, i32)]) {
        let MinMaxResult::MinMax(x_low, x_high) = rope.iter().map(|k| k.0).minmax() else {
            panic!("Grid is only one column.")
        };
        let MinMaxResult::MinMax(y_low, y_high) = rope.iter().map(|k| k.1).minmax() else {
            panic!("Grid is only one row.")
        };

        let offset_x = -x_low; // x == 0 => do nothing, x < 0 => offset right, x > 0 should never happen (start is 0)
        let offset_y = -y_low;

        let grid_width = (x_high + offset_x + 1) as usize;
        let grid_height = (y_high + offset_y + 1) as usize;

        let mut grid = vec![vec!['.'; grid_width]; grid_height];

        for (idx, knot) in rope.iter().enumerate() {
            let (x, y) = knot;
            grid[(y + offset_y) as usize][(x + offset_x) as usize] =
                idx.to_string().chars().next().unwrap()
        }

        println!("Current rope grid:");
        for row in grid.iter().rev() {
            println!("{:?}", row);
        }
        println!("");
    }

    #[solver(part2, draft)]
    pub fn solve_part2(input: GenData) -> OutData {
        const KNOT_COUNT: usize = 10;
        let mut knots = [(0i32, 0i32); KNOT_COUNT];
        let mut tail_coords: HashSet<(i32, i32)> = HashSet::new();
        tail_coords.insert((0, 0));
        for Movement { dir, count } in input {
            let offset = match dir {
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
                Direction::Down => (0, -1),
                Direction::Up => (0, 1),
            };
            for _ in 0..count {
                knots[0] = (knots[0].0 + offset.0, knots[0].1 + offset.1);
                for (last_idx, current_idx) in (0..knots.len()).tuple_windows() {
                    let old_knot = knots[last_idx];
                    let knot = knots.get_mut(current_idx).unwrap();
                    let delta: (i32, i32) = (old_knot.0 - knot.0, old_knot.1 - knot.1);
                    match (delta.0.abs(), delta.1.abs()) {
                        (1, 0) | (0, 1) | (1, 1) | (0, 0) => {
                            continue;
                        }
                        (_, 0) => {
                            knot.0 += delta.0.signum();
                        }
                        (0, _) => {
                            knot.1 += delta.1.signum();
                        }
                        (_, _) => {
                            knot.0 += delta.0.signum();
                            knot.1 += delta.1.signum();
                        }
                    }
                }
                tail_coords.insert(knots[KNOT_COUNT - 1]);
            }
        }

        tail_coords.len()
    }
}

#[cfg(test)]
pub mod tests {
    use super::solutions::*;

    #[allow(unused)]
    const TEST_IN: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    #[allow(unused)]
    const TEST_IN_2: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    pub fn test_part1() {
        assert_eq!(solve_part1(input_generator(TEST_IN)), 13);
    }

    #[test]
    pub fn test_part2() {
        //assert_eq!(solve_part2(&input_generator(TEST_IN)), 1);
        assert_eq!(solve_part2(input_generator(TEST_IN_2)), 36);
    }
}
