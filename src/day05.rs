use std::io;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct MoveInfo {
    source_stack: u32,
    dest_stack: u32,
    crate_count: i32,
}

pub type CrateStack = Vec<char>;

pub type Board = Vec<CrateStack>;

pub type Output = String;

fn print_board(brd: &Board) {
    println!("Board [");
    for (i, stack) in brd.iter().enumerate() {
        let s = stack.iter().join(" ");
        println!("  {}  {}", i, s);
    }
    println!("]")
}

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> (Board, Vec<MoveInfo>) {
    let first_line = input.lines().next().unwrap();
    let expected_stacks = (first_line.len() + 1) / 4;
    let mut stacks: Board = (0..expected_stacks).map(|_| Vec::new()).collect_vec();
    
    let mut parsed_board = false;

    let mut moves: Vec<MoveInfo> = Vec::new();
    let re = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();

    for line in input.lines() {
        if line.contains(" 1  ") {
            parsed_board = true;
            for stack in stacks.iter_mut() {
                stack.reverse();
                stack.retain(|c| c != &' ');
            }
            continue;
        }
        if !parsed_board {
            for (i, col) in line.chars().chunks(4).into_iter().enumerate() {
                let crate_label = col.collect_vec()[1];
                stacks[i].push(crate_label);
            }
        } else {
            let match_info = re.captures(line);
            match match_info {
                None => { continue; },
                Some(m) => {
                    let count: i32 = m.get(1).unwrap().as_str().parse().unwrap();
                    let mut src: u32 = m.get(2).unwrap().as_str().parse().unwrap();
                    let mut dest: u32 = m.get(3).unwrap().as_str().parse().unwrap();
                    src -= 1;
                    dest -= 1;
                    moves.push(MoveInfo { source_stack: src, dest_stack: dest, crate_count: count });
                }
            }
        }
        
    }
    // println!("Initial Board:");
    // print_board(&stacks);
    (stacks, moves)
}

const DEBUG_MOVE: bool = false;

fn run_arrangement(board: &mut Board, moves: &[MoveInfo], do_reverse: bool) {
    let stdin = io::stdin();
    let num_moves = moves.len();
    for (i, mov) in moves.iter().enumerate() {
        if DEBUG_MOVE {
            println!("Executing move {} of {}: Move {} crates from stack {} to stack {}", i+1, num_moves, mov.crate_count, mov.source_stack, mov.dest_stack);
            println!("Board before move:");
            print_board(board);
        }
        let mut i = {
            let stack =  board.get_mut(mov.source_stack as usize).unwrap();
            let mut xs = stack
                .drain((stack.len()-mov.crate_count as usize)..)
                .collect_vec();
            if do_reverse {
                xs.reverse();
            }
            xs
        };
        {
            board.get_mut(mov.dest_stack as usize).unwrap().append(&mut i);
        }
        if DEBUG_MOVE {
            println!("Board after move:");
            print_board(board);
            println!("\n\n");
            println!("Continue? ");
            let mut line_read: String = String::new();
            _ = stdin.read_line(&mut line_read);
            if line_read.trim() == "n" {
                panic!("Aborted by user request.");
            }
        }
    }
}

#[aoc(day05, part1)]
pub fn solve_part1(input: &(Board, Vec<MoveInfo>)) -> Output {
    let mut board = input.0.iter().cloned().collect_vec();
    run_arrangement(&mut board, &input.1, true);
    let mut result = String::new();
    for stack in board {
        result = result + &stack.last().unwrap().to_string();
    }

    result
}

#[aoc(day05, part2)]
pub fn solve_part2(input: &(Board, Vec<MoveInfo>)) -> Output {
    let mut board = input.0.iter().cloned().collect_vec();
    run_arrangement(&mut board, &input.1, false);
    let mut result = String::new();
    for stack in board {
        result = result + &stack.last().unwrap().to_string();
    }

    result
}
