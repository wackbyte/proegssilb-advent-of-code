use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::mem::take;
use std::ops::{Add, RangeInclusive};

pub type GenData = Vec<(i64, i64, i64, i64)>;
pub type InData<'a> = &'a [(i64, i64, i64, i64)];
pub type OutData = usize;

fn range_overlaps<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: Ord,
{
    !(a.end() < b.start() || b.end() < a.start())
}

fn range_adjacent<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: Ord + Add<T> + From<i32> + Copy,
    <T as std::ops::Add>::Output: PartialEq<T>,
{
    *a.end() + 1.into() == *b.start() || *b.end() + 1.into() == *a.start()
}

#[derive(Debug)]
pub struct RangeLine {
    ranges_set: Vec<RangeInclusive<i64>>,
}

impl RangeLine {
    fn new() -> RangeLine {
        RangeLine {
            ranges_set: Vec::new(),
        }
    }

    fn set_range(&mut self, x: &RangeInclusive<i64>) {
        let mut x = x.clone();
        for idx in 0..self.ranges_set.len() {
            while idx < self.ranges_set.len()
                && (range_overlaps(&x, &self.ranges_set[idx])
                    || range_adjacent(&x, &self.ranges_set[idx]))
            {
                let old = self.ranges_set.remove(idx);
                let existing_min = *old.start();
                let existing_max = *old.end();
                let new_min = min(*x.start(), existing_min);
                let new_max = max(*x.end(), existing_max);
                x = new_min..=new_max;
            }
        }
        self.ranges_set.push(x);
    }

    fn unset_point(&mut self, pt: i64) {
        for idx in 0..self.ranges_set.len() {
            if self.ranges_set[idx].contains(&pt) {
                let range_a = *self.ranges_set[idx].start()..=(pt - 1);
                let range_b = (pt + 1)..=*(self.ranges_set[idx].end());
                self.ranges_set[idx] = range_a;
                self.set_range(&range_b);
                return;
            }
        }
    }

    fn len(&self) -> i64 {
        self.ranges_set
            .iter()
            .map(|r| {
                let e = *r.end();
                let s = *r.start();
                (e.abs_diff(s) + 1) as i64
            })
            .sum()
    }

    fn normalize(&mut self) {
        let ranges = take(&mut self.ranges_set);
        for r in ranges {
            self.set_range(&r);
        }
    }

    fn ranges_count(&self) -> usize {
        self.ranges_set.len()
    }

    fn contains(&self, point: i64) -> bool {
        self.ranges_set.iter().any(|r| r.contains(&point))
    }

    fn raw<'a>(&'a self) -> &'a Vec<RangeInclusive<i64>> {
        &self.ranges_set
    }
}

// Solution ---------------------------------------------------------

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> GenData {
    let input = input.trim_start();

    let mut data: GenData = Vec::new();

    for ln in input.lines() {
        let (_, rest) = ln.split_once('=').unwrap();
        let (sensor_x, rest) = rest.split_once(',').unwrap();
        let (_, rest) = rest.split_once('=').unwrap();
        let (sensor_y, rest) = rest.split_once(':').unwrap();
        let (_, rest) = rest.split_once('=').unwrap();
        let (beacon_x, rest) = rest.split_once(',').unwrap();
        let (_, beacon_y) = rest.split_once('=').unwrap();

        let sensor_x: i64 = sensor_x.parse().unwrap();
        let sensor_y: i64 = sensor_y.parse().unwrap();
        let beacon_x: i64 = beacon_x.parse().unwrap();
        let beacon_y: i64 = beacon_y.parse().unwrap();

        data.push((sensor_x, sensor_y, beacon_x, beacon_y));
    }

    data
}

#[aoc(day15, part1)]
pub fn solve_part1(input: InData) -> OutData {
    let mut mapped_spots: RangeLine = RangeLine::new();
    let mut beacon_spots: HashSet<i64> = HashSet::new();

    let target_y = if cfg!(test) { 10i64 } else { 2_000_000i64 };

    for (sensor_x, sensor_y, beacon_x, beacon_y) in input.iter() {
        let distance = (sensor_x.abs_diff(*beacon_x) + sensor_y.abs_diff(*beacon_y)) as i64;
        let dist_remain = distance - sensor_y.abs_diff(target_y) as i64;
        if dist_remain > 0 {
            let min_x = sensor_x - dist_remain;
            let max_x = sensor_x + dist_remain;
            let range = min_x..=max_x;
            mapped_spots.set_range(&range);
        }
        if *beacon_y == target_y {
            beacon_spots.insert(*beacon_x);
        }
    }

    for pt in beacon_spots.iter() {
        mapped_spots.unset_point(*pt);
    }

    mapped_spots.normalize();

    dbg!(&beacon_spots.len());
    dbg!(&mapped_spots.len());

    mapped_spots.len() as usize
}

#[cfg(test)]
const MAX_COORD: u32 = 20;

#[cfg(not(test))]
const MAX_COORD: u32 = 4_000_000;

const TUNING_ADJUST: i64 = 4_000_000;

#[aoc(day15, part2)]
pub fn solve_part2(input: InData) -> OutData {
    let mut grid: Vec<RangeLine> = Vec::new();
    for _ in 0..=MAX_COORD {
        grid.push(RangeLine::new());
    }

    for (sensor_x, sensor_y, beacon_x, beacon_y) in input.iter() {
        let total_distance = (sensor_x.abs_diff(*beacon_x) + sensor_y.abs_diff(*beacon_y)) as i64;
        let min_y = max(sensor_y - total_distance, 0);
        let max_y = min(sensor_y + total_distance, MAX_COORD as i64);
        // For each Y (within bounds) the sensor can cover {
        for y_val in min_y..=max_y {
            // Do projection to figure out coverage range
            let distance_remaining = total_distance - y_val.abs_diff(*sensor_y) as i64;
            let min_x = max(0, *sensor_x - distance_remaining);
            let max_x = min(MAX_COORD as i64, *sensor_x + distance_remaining);
            // Set range for that Y's RangeLine
            let range = min_x..=max_x;
            grid[y_val as usize].set_range(&range);
        }
    }

    dbg!(grid.iter().map(|r| r.raw().capacity()).max());

    for (idx, row) in grid
        .iter()
        .enumerate()
        .filter(|&(_, c)| c.ranges_count() > 1)
    {
        for x_val in 0..MAX_COORD {
            if !row.contains(x_val as i64) {
                return ((x_val as i64) * TUNING_ADJUST) as usize + idx;
            }
        }
    }

    0
}

#[allow(unused)]
const TEST_IN: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

#[test]
pub fn test_part1() {
    assert_eq!(solve_part1(&input_generator(TEST_IN)), 26);
}

#[test]
pub fn test_part2() {
    assert_eq!(solve_part2(&input_generator(TEST_IN)), 56000011);
}
