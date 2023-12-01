use aoc_zen_runner_macros::{aoc, generator, solver};
use itertools::{Itertools, MinMaxResult};
use rayon::prelude::*;
#[allow(unused_imports)]
use std::cmp::max;
use std::{
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicI64, Ordering},
};

#[aoc(2022, day18)]
pub mod solutions {
    use super::*;

    pub type GenData = Vec<(i64, i64, i64)>;
    pub type OutData = usize;


    #[generator(dotchain)]
    pub fn input_generator(input: &str) -> GenData {
        let input = input.trim_start();
        let points: Vec<(i64, i64, i64)> = input
            .par_lines()
            .map(|ln| {
                ln.split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        points
    }

    #[solver(part1, mappyhelpy)]
    pub fn solve_part1(input: GenData) -> OutData {
        let points: HashSet<(i64, i64, i64)> = input.iter().cloned().collect();

        points
            .iter()
            .map(|&(x, y, z)| {
                let neighbors = [
                    (x - 1, y, z),
                    (x + 1, y, z),
                    (x, y - 1, z),
                    (x, y + 1, z),
                    (x, y, z - 1),
                    (x, y, z + 1),
                ];

                neighbors
                    .iter()
                    .map(|c| !points.contains(c))
                    .filter(|b| *b)
                    .count()
            })
            .sum()
    }

    fn neighbors(x: i64, y: i64, z: i64) -> [(i64, i64, i64); 6] {
        [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
    }

    /// Check if a value is in an inclusive and possibly-backwards range.
    pub fn in_range(i_test: i64, i_min: i64, i_max: i64) -> bool {
        i_test == i_min
            || i_test == i_max
            || ((i_test - i_min).signum() == (i_max - i_min).signum()
                && (i_test - i_max).signum() == (i_min - i_max).signum())
    }

    #[solver(part1, rayon)]
    pub fn solve_part1_rayon(input: GenData) -> OutData {
        let points: HashSet<(i64, i64, i64)> = input.iter().cloned().collect();

        points
            .par_iter()
            .map(|&(x, y, z)| {
                neighbors(x, y, z)
                    .iter()
                    .map(|c| !points.contains(c))
                    .filter(|b| *b)
                    .count()
            })
            .sum()
    }

    #[solver(part2, traverse)]
    pub fn solve_part2_traversal(input: GenData) -> OutData {
        let points: HashSet<(i64, i64, i64)> = input.iter().cloned().collect();

        let MinMaxResult::MinMax(x_min, x_max) = points.iter().map(|p| p.0).minmax() else {
            panic!("Did not find two points in X");
        };
        let MinMaxResult::MinMax(y_min, y_max) = points.iter().map(|p| p.1).minmax() else {
            panic!("Did not find two points in Y");
        };
        let MinMaxResult::MinMax(z_min, z_max) = points.iter().map(|p| p.2).minmax() else {
            panic!("Did not find two points in Z");
        };

        let mut work_queue: VecDeque<(i64, i64, i64)> = ((x_min - 1)..=(x_max + 1))
            .cartesian_product((y_min - 1)..=(y_max + 1))
            .map(|(x, y)| (x, y, z_min - 1))
            .collect();

        let mut explored: HashSet<(i64, i64, i64)> = HashSet::new();

        let surface = AtomicI64::new(0);

        let max_count = (x_max + 2) * (y_max + 2) * (z_max + 2);
        println!(
            "Maximum space: [X: {}..={}]  [Y: {}..={}]  [Z: {}..={}]  [Total: {}]",
            x_min, x_max, y_min, y_max, z_min, z_max, max_count
        );
        let seen = AtomicI64::new(0);

        while let Some(pt @ (x, y, z)) = work_queue.pop_front() {
            if explored.contains(&pt) {
                continue;
            }
            explored.insert(pt);

            // if pt == (2, 2, 4) {
            //     println!("Explored point {:?}", pt);
            // }

            let current_seen = seen.fetch_add(1, Ordering::AcqRel);
            // if current_seen % 100 == 0 {
            //     println!("{} nodes explored", current_seen);
            // }
            if current_seen > max_count {
                panic!("Explored too many nodes, there must be an infinite loop.")
            }

            let surface_seen = neighbors(x, y, z)
                .into_iter()
                .filter(|pt| points.contains(pt))
                .count() as i64;
            surface.fetch_add(surface_seen, Ordering::Relaxed);
            work_queue.extend(
                neighbors(x, y, z)
                    .into_iter()
                    .filter(|pt| !points.contains(pt) && !explored.contains(pt))
                    .filter(|&(x, y, z)| {
                        in_range(x, x_min - 1, x_max + 1)
                            && in_range(y, y_min - 1, y_max + 1)
                            && in_range(z, z_min - 1, z_max + 1)
                    }),
            );
        }

        surface.into_inner() as usize
    }
}

#[cfg(test)]
pub mod test {
    use aoc_zen_runner_macros::aoc_case;

    use super::solutions::*;

    #[test]
    fn test_in_range() {
        assert!(in_range(2, -5, 5));
        assert!(in_range(1, -5, 5));
        assert!(in_range(-1, -5, 5));
        assert!(in_range(0, -5, 5));
        assert!(in_range(5, -5, 5));
        assert!(in_range(-5, -5, 5));
        assert!(!in_range(7, -5, 5));
        assert!(!in_range(-7, -5, 5));
        assert!(!in_range(0, 2, 5));
        assert!(!in_range(1, 2, 5));
        // Repeat all tests with the ranges backward.
        assert!(in_range(2, 5, -5));
        assert!(in_range(1, 5, -5));
        assert!(in_range(-1, 5, -5));
        assert!(in_range(0, 5, -5));
        assert!(in_range(5, 5, -5));
        assert!(in_range(-5, 5, -5));
        assert!(!in_range(7, 5, -5));
        assert!(!in_range(-7, 5, -5));
        assert!(!in_range(0, 5, 2));
        assert!(!in_range(1, 5, 2));
    }

    #[aoc_case(64, 58)]
    const test_in: &str = r#"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;

    #[test]
    pub fn test_part2_2() {
        let points2 = r#"
1,1,1
1,1,2
1,1,3
1,2,1
1,2,2
1,2,3
1,3,1
1,3,2
1,3,3
2,1,1
2,1,2
2,1,3
2,2,1
2,2,3
2,3,1
2,3,2
2,3,3
3,1,1
3,1,2
3,1,3
3,2,1
3,2,2
3,2,3
3,3,1
3,3,2
3,3,3
"#;
        assert_eq!(solve_part2_traversal(input_generator(points2)), 54);
    }
}
