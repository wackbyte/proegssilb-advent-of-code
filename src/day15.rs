use std::cmp::Ordering;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use aoc_runner_derive::{aoc_generator, aoc};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Field(u32),
    Sensor(u8),
    Beacon(u8),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Field(0u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point{x: i64, y: i64}

pub type SparseGrid = HashMap<Point, Cell>;

pub type GenData = Vec<(i64, i64, i64, i64)>;
pub type InData<'a> = &'a [(i64, i64, i64, i64)];
pub type OutData = usize;

fn range_either_contains<T>(a: RangeInclusive<T>, b: RangeInclusive<T>) -> bool
    where T: Ord
{
    let cmp1 = a.start().cmp(b.start());
    let cmp2 = a.end().cmp(b.end());
    let res = cmp1 != cmp2 || cmp1 == Ordering::Equal;
    res
}

fn range_overlaps<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool 
where T: Ord
{
    !(a.end() < b.start() || b.end() < a.start())
}

pub struct RangeLine {
    ranges_set: Vec<RangeInclusive<i64>>,
}

impl RangeLine {
    fn new() -> RangeLine {
        RangeLine { ranges_set: Vec::new()}
    }

    fn set_range(&mut self, x: &RangeInclusive<i64>) {
        for idx in 0..self.ranges_set.len() {
            let intersect = range_overlaps(x, &self.ranges_set[idx]);
            if intersect {
                let existing_min = *self.ranges_set[idx].start();
                let existing_max = *self.ranges_set[idx].end();
                let new_min = min(*x.start(), existing_min);
                let new_max = max(*x.end(), existing_max);
                self.ranges_set[idx] = RangeInclusive::new(new_min, new_max);
                break;
            }
        }
    }

    fn unset_point(&mut self, pt: i64) {
        for idx in 0..self.ranges_set.len() {
            if self.ranges_set[idx].contains(&pt) {
                let range_a = *self.ranges_set[idx].start()..=(pt-1);
                let range_b = (pt+1)..=*(self.ranges_set[idx].end());
                self.ranges_set[idx] = range_a;
                self.ranges_set.push(range_b);
                break;
            }
        }
    }

    fn len(&self) -> i64 {
        self.ranges_set.iter().map(|r| r.end() - r.start()).sum()
    }
}

// Solution ---------------------------------------------------------
// Choose One

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

    let target_y = if cfg!(test) {
        10i64
    } else {
        2_000_000i64
    };

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

    dbg!(&beacon_spots.len());
    dbg!(&mapped_spots.len());

    for pt in beacon_spots {
        mapped_spots.unset_point(pt);
    }

    mapped_spots.len() as usize
}

#[aoc(day15, part2)]
pub fn solve_part2(input: InData) -> OutData {
    let max_coord = if cfg!(test) {
        20i64
    } else {
        4_000_000i64
    };

    let tuning_adjust = 4_000_000i64;

    todo!()
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
