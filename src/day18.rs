#[allow(unused_imports)]
use std::cmp::max;
use std::{ops::{RangeInclusive, IndexMut, Index}, collections::HashSet};
use aoc_runner_derive::{aoc_generator, aoc};
use itertools::{Itertools, MinMaxResult};
use rayon::prelude::*;

pub type GenData = Vec<(i64, i64, i64)>;
pub type InData<'a> = &'a [(i64, i64, i64)];
pub type OutData = usize;

pub struct Small3Grid<T> {
    data: Vec<T>,
    x_min: i64,
    x_width: i64,
    x_max: i64,
    y_min: i64,
    y_width: i64,
    y_max: i64,
    z_min: i64,
    z_width: i64,
    z_max: i64,
}

impl<T: Clone + Default> Small3Grid<T>
{
    fn new_from_dims(x: RangeInclusive<i64>, y: RangeInclusive<i64>, z: RangeInclusive<i64>) -> Small3Grid<T> {
        Small3Grid { 
            x_min: *x.start(), x_max: *x.end(), x_width: x.clone().count() as i64,
            y_min: *y.start(), y_max: *y.end(), y_width: y.clone().count() as i64,
            z_min: *z.start(), z_max: *z.end(), z_width: z.clone().count() as i64,
            data: vec![T::default(); x.count() * y.count() * z.count()],
        }
    }

    fn iter(&self) -> impl Iterator + '_ {
        self.data.iter()
    }
}

impl<T> IndexMut<(i64, i64, i64)> for Small3Grid<T> {
    fn index_mut(&mut self, index: (i64, i64, i64)) -> &mut Self::Output {
        let (x, y, z) = index;

        if x < self.x_min || x > self.x_max { panic!("X out of range: {}", x)}
        if y < self.y_min || y > self.y_max { panic!("Y out of range: {}", y)}
        if z < self.z_min || z > self.z_max { panic!("Z out of range: {}", z)}

        // Simulate grid[x][y][z]
        let idx = (x * self.y_width * self.z_width + y * self.z_width + z) as usize;
        &mut self.data[idx]
    }
}

impl<T> Index<(i64, i64, i64)> for Small3Grid<T> {
    type Output = T;
    fn index(&self, index: (i64, i64, i64)) -> &Self::Output {
        let (x, y, z) = index;

        if x < self.x_min || x > self.x_max { panic!("X out of range: {}", x)}
        if y < self.y_min || y > self.y_max { panic!("Y out of range: {}", y)}
        if z < self.z_min || z > self.z_max { panic!("Z out of range: {}", z)}

        // Simulate grid[x][y][z]
        let idx = (x * self.y_width * self.z_width + y * self.z_width + z) as usize;
        
        &self.data[idx]
    }
}
/*
impl<'a, T> IntoIterator for &'a Small3Grid<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Small3Grid<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for Small3Grid<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
*/

// -- -- --

/*
impl<'data, T: Sync + 'data> IntoParallelIterator for &'data Small3Grid<T> {
    type Item = &'data T;

    type Iter = rayon::slice::Iter<'data, T>;

    fn into_par_iter(self) -> Self::Iter {
        self.data.into_par_iter()
    }
}

impl<'data, T: Send + 'data> IntoParallelIterator for &'data mut Small3Grid<T> {
    type Item = &'data mut T;

    type Iter = rayon::slice::IterMut<'data, T>;

    fn into_par_iter(self) -> Self::Iter {
        <&mut [T]>::into_par_iter(&mut self.data)
    }
}

impl<'a, T> IntoParallelIterator for Small3Grid<T>
    where T: Send
{
    type Iter = rayon::vec::IntoIter<T>;

    type Item = T;

    fn into_par_iter(self) -> Self::Iter {
        self.data.into_par_iter()
    }
}
*/


// Solution ---------------------------------------------------------
// Choose One

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> GenData {
    let input = input.trim_start();
    let points: Vec<(i64, i64, i64)> = input.par_lines().map(|ln| ln.split(',').map(|s| s.parse::<i64>().unwrap()).collect_tuple().unwrap()).collect();
    
    points
}

#[aoc(day18, part1)]
pub fn solve_part1(input: InData) -> OutData {
    let points: HashSet<(i64, i64, i64)> = input.into_iter().cloned().collect();

    points.iter().map(|&(x, y, z)| {
        let neighbors = [(x-1, y, z), (x+1, y, z), 
                (x, y-1, z), (x, y+1, z),
                (x, y, z-1), (x, y, z+1),
                ];

        neighbors.iter().map(|c| !points.contains(c)).filter(|b| *b).count()
    }).sum()
}

fn neighbors(x: i64, y: i64, z: i64) -> [(i64, i64, i64); 6] {
    [(x-1, y, z), (x+1, y, z), 
     (x, y-1, z), (x, y+1, z),
     (x, y, z-1), (x, y, z+1),
     ]
}

#[aoc(day18, part1, rayon)]
pub fn solve_part1_rayon(input: InData) -> OutData {
    let points: HashSet<(i64, i64, i64)> = input.into_iter().cloned().collect();

    points.par_iter().map(|&(x, y, z)| {
        neighbors(x, y, z).iter().map(|c| !points.contains(c)).filter(|b| *b).count()
    }).sum()
}

/*
// This code fails at run-time because the math involved with indexing the 3D grid is wrong.

#[aoc(day18, part1, grid)]
pub fn solve_part1_grid(points: InData) -> OutData {
    let MinMaxResult::MinMax(x_min, x_max) = points.iter().map(|p| p.0).minmax() else { panic!("Did not find two points in X"); };
    let MinMaxResult::MinMax(y_min, y_max) = points.iter().map(|p| p.1).minmax() else { panic!("Did not find two points in Y"); };
    let MinMaxResult::MinMax(z_min, z_max) = points.iter().map(|p| p.2).minmax() else { panic!("Did not find two points in Z"); };

    let mut grid = Small3Grid::new_from_dims(x_min..=x_max, y_min..=y_max, z_min..=z_max);

    for point in points {
        grid[*point] = true;
    }

    // Just getting the coords lined up is some work
    let coords = (grid.x_min..=grid.x_max)
        .cartesian_product(grid.y_min..=grid.y_max)
        .cartesian_product(grid.z_min..=grid.z_max)
        .par_bridge()
        .map(|((x, y), z)| (x, y, z));

    // OK, let's do this.
    coords.map(|(x, y, z)| {
        if grid[(x, y, z)] {
            let neighbors = [grid[(x-1, y, z)], grid[(x+1, y, z)], 
                grid[(x, y-1, z)], grid[(x, y+1, z)],
                grid[(x, y, z-1)], grid[(x, y, z+1)],
                ];
            neighbors.iter().filter(|b| !**b).count()
        } else {
            0
        }
    }).sum()
}
*/

// #[aoc(day18, part2)]
// pub fn solve_part2(input: InData) -> OutData {
//     todo!()
// }

#[allow(unused)]
const TEST_IN: &str = r#"
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
pub fn test_part1() {
    assert_eq!(solve_part1(&input_generator(TEST_IN)), 64);
}

// #[test]
// pub fn test_part2() {
//     assert_eq!(solve_part2(&input_generator(TEST_IN)), _Z);
// }

// ------------- Or -------------

// #[aoc(day18, part1)]
// pub fn solve_part1(input: &str) -> OutData {
//     let input = input.trim_start();
// }

// #[aoc(day18, part2)]
// pub fn solve_part2(input: &str) -> OutData {
//     let input = input.trim_start();
// }

// #[allow(unused)]
// const TEST_IN: &str = r#"
// "#;

// #[test]
// pub fn test_part1() {
//     assert_eq!(solve_part1(TEST_IN), _Y);
// }

// #[test]
// pub fn test_part2() {
//     assert_eq!(solve_part2(TEST_IN), _Z);
// }