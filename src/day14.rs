use core::panic;
use std::cmp::max;
use std::{io::{stdout, Write}, fmt::Display, cmp::min};
use aoc_runner_derive::{aoc_generator, aoc};
use grid::Grid;
use itertools::{Itertools, MinMaxResult};
use termion::cursor;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Cell {
    #[default]
    Nothing,
    Sand,
    Stone,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nothing => ".".fmt(f),
            Sand => "o".fmt(f),
            Stone => "#".fmt(f),
        }
    }
}

use Cell::*;

pub struct GenData {
    x_offset: usize,
    y_abyss: usize,
    grid: Grid<Cell>,
}
pub type OutData = u64;

pub fn parser(input: &str, gen_floor: bool) -> GenData {
    let input = input.trim_start();
    let paths: Vec<Vec<(usize, usize)>> = input.lines().map(|path| {
        path.split(" -> ").map(|s| {
            let Some((x, y)) = s.split_once(",") else { panic!("Invalid coordinate: {}", s) };
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            (x, y)
        }).collect_vec()
    }).collect_vec();

    let MinMaxResult::MinMax(min_x, max_x) = paths.iter().flat_map(|ps| ps.iter().map(|pt| pt.0)).minmax() else {
        panic!("MinMaxResult X produced fewer than 2 distinct values.");
    };
    let MinMaxResult::MinMax(_, max_y) = paths.iter().flat_map(|ps| ps.iter().map(|pt| pt.1)).minmax() else {
        panic!("MinMaxResult Y produced fewer than 2 distinct values.");
    };

    // println!("X Range: {}-{}", min_x, max_x);
    // println!("Y Range: {}-{}", min_y, max_y);

    let x_offset = min_x - 200;
    let y_abyss = max_y + 4;

    let grid: Grid<Cell> = Grid::new(max_y + 6, max_x - x_offset + 200);

    let mut res = GenData { grid, x_offset, y_abyss};

    for path in paths {
        for seg in path.windows(2) {
            let [p1, p2] = seg else { panic!("Logical contradiction in window extraction: {:?}", seg) };
            if p1.0 == p2.0 {
                let x = p1.0 - x_offset;
                let start = min(p1.1, p2.1);
                let stop = max(p1.1, p2.1);
                for y in start..=stop {
                    let cell = res.grid.get_mut(y, x).unwrap();
                    *cell = Stone;
                }
            } else {
                assert_eq!(p1.1, p2.1, "Neither X nor Y were equal in segment: {:?}", seg);
                let y = p1.1;
                let start = min(p1.0, p2.0);
                let stop = max(p1.0, p2.0);
                for x in start..=stop {
                    let x = x - x_offset;
                    let cell = res.grid.get_mut(y, x).unwrap();
                    *cell = Stone;
                }
            }
        }
    }

    if gen_floor {
        for c in res.grid.iter_row_mut(max_y + 2) {
            *c = Stone;
        }
    }


    //draw_cave(&res, "Initial Grid:");

    res
}

#[aoc_generator(day14, part1)]
pub fn input_p1(input: &str) -> GenData {
    parser(input, false)
}

#[aoc_generator(day14, part2)]
pub fn input_p2(input: &str) -> GenData {
    parser(input, true)
}

fn get_next_locs(data: &GenData, (x, y): (usize, usize)) -> Option<[Cell; 3]> {
    let GenData {grid, x_offset, y_abyss } = data;

    if y+1 == *y_abyss {
        return None;
    }

    let (x, false) = x.overflowing_sub(*x_offset) else {panic!("Could not offset X-Coord of point {:?} by {}", (x, y), x_offset)};
    let c1 = *grid.get(y + 1, x).expect(&format!("Coords could not be offset: {:?} {}", (x + x_offset, y), "c1"));
    let c2 = *grid.get(y + 1, x-1).expect(&format!("Coords could not be offset: {:?} {}", (x + x_offset, y), "c2"));
    let c3 = *grid.get(y + 1, x + 1).expect(&format!("Coords could not be offset: {:?} {}", (x + x_offset, y), "c3"));
    Some([c1, c2, c3])
}

#[allow(unused)]
fn draw_cave(cave: &GenData, msg: &str) {
    let GenData {grid, x_offset, y_abyss: _} = cave;

    let mut s = stdout();
    println!("{}{}", cursor::Save, msg);
    for r_idx in 0..grid.rows() {
        for (c_idx, c) in grid.iter_row(r_idx).enumerate() {
            if r_idx == 0 && c_idx + x_offset == 500 {
                print!("+");
            } else {
                print!("{}", c);
            }            
        }
        println!("")
    }
    print!("{}", cursor::Restore);
    s.flush().unwrap();
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &GenData) -> OutData {
    let mut input = GenData { x_offset: input.x_offset.clone(), grid: input.grid.clone(), y_abyss: input.y_abyss.clone(),};
    let mut sand_counter = 0;
    while input.grid.get(0, 500 - input.x_offset) != Some(&Sand) {
        sand_counter += 1;
        let mut current_loc = (500, 0);
        let Some(mut next_locs) = get_next_locs(&input, current_loc) else {
            return sand_counter - 1;
        };
        while next_locs.iter().any(|c| *c == Nothing) {
            match next_locs {
                [Nothing, _, _] => { current_loc.1 += 1; },
                [_, Nothing, _] => { current_loc.1 += 1; current_loc.0 -= 1; },
                [_, _, Nothing] => { current_loc.1 += 1; current_loc.0 += 1; },
                _ => { panic!("Logical contradiction while selecting next location") },
            }
            
            //draw_cave(&input, &format!("Sand Unit {}", sand_counter));
            match get_next_locs(&input, current_loc) {
                Some(nl) => {next_locs = nl;},
                None => {return sand_counter - 1;}
            }
        }

        let c = input.grid.get_mut(current_loc.1, current_loc.0 - input.x_offset).expect("Final location for grain outside of grid.");
        assert_eq!(*c, Nothing, "Expected location {:?} to be empty, but found {:?}", current_loc, c);
        *c = Sand;
    }

    todo!()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &GenData) -> OutData {
    let mut input = GenData { x_offset: input.x_offset.clone(), grid: input.grid.clone(), y_abyss: input.y_abyss.clone(),};
    let mut sand_counter = 0;
    while input.grid.get(0, 500 - input.x_offset) != Some(&Sand) {
        sand_counter += 1;
        let mut current_loc = (500, 0);
        let Some(mut next_locs) = get_next_locs(&input, current_loc) else {
            panic!("Should not have found the abyss, but did.");
        };
        while next_locs.iter().any(|c| *c == Nothing) {
            match next_locs {
                [Nothing, _, _] => { current_loc.1 += 1; },
                [_, Nothing, _] => { current_loc.1 += 1; current_loc.0 -= 1; },
                [_, _, Nothing] => { current_loc.1 += 1; current_loc.0 += 1; },
                _ => { panic!("Logical contradiction while selecting next location") },
            }
            
            //draw_cave(&input, &format!("Sand Unit {}", sand_counter));
            match get_next_locs(&input, current_loc) {
                Some(nl) => {next_locs = nl;},
                None => {return sand_counter - 1;}
            }
        }

        let c = input.grid.get_mut(current_loc.1, current_loc.0 - input.x_offset).expect("Final location for grain outside of grid.");
        assert_eq!(*c, Nothing, "Expected location {:?} to be empty, but found {:?}", current_loc, c);
        *c = Sand;
    }

    sand_counter
}

#[allow(unused)]
const TEST_IN: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

#[test]
pub fn test_d14_part1() {
    assert_eq!(solve_part1(&input_p1(TEST_IN)), 24);
}

#[test]
pub fn test_d14_part2() {
    assert_eq!(solve_part2(&input_p2(TEST_IN)), 93);
}