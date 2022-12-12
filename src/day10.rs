#[allow(unused_imports)]
use std::cmp::max;
use aoc_runner_derive::{aoc_generator, aoc};
use itertools::Itertools;

pub enum Instruction {
    Noop,
    Addx(i32)
}

pub type GenData = Vec<Instruction>;
pub type InData<'a> = &'a [Instruction];
pub type OutData = i32;


// Solution ---------------------------------------------------------
// Choose One

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> GenData {
    let mut results = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line == "noop" {
            results.push(Instruction::Noop)
        }
        if line.starts_with("addx") {
            let (_, amt) = line.split_once(" ").unwrap();
            let amt = amt.parse::<i32>().unwrap();
            results.push(Instruction::Addx(amt));
        }
    }

    results
}

fn convert_instrs_to_register_stream(instrs: InData) -> Vec<(i32, i32)> {
    instrs.iter().map(|instr| {
        match instr {
            Instruction::Noop => (1, 0),
            Instruction::Addx(amt) => (2, *amt),
        }
    }).scan((1, 1), |(cycle_count, reg_val), (cyc_delta, reg_delta)| {
        *cycle_count += cyc_delta;
        *reg_val += reg_delta;
        Some((*cycle_count, *reg_val))
    }).collect_vec()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: InData) -> OutData {
    let reg_stream = convert_instrs_to_register_stream(input);
    let breakpoints = [20, 60, 100, 140, 180, 220];
    reg_stream.windows(2).scan(0, |break_idx, w| {
        let [(last_cyc, last_reg), (curr_cyc, curr_reg)] = w else { panic!("Logical contradiction in window extraction.") };
        let break_val: &i32 = breakpoints.get(*break_idx as usize)?;
        if last_cyc < break_val && curr_cyc >= break_val {
            // Return the current
            *break_idx += 1;
            if curr_cyc == break_val {
                return Some(*curr_reg * break_val);
            } else {
                return Some(*last_reg * break_val);
            }
        } else {
            return Some(-1);
        }
    }).filter(|x| *x > 0).sum()
}

fn format_screen(pixels: &str) -> String {
    pixels.chars().chunks(40).into_iter().map(|mut c| c.join("")).take(6).join("\n")
}

#[aoc(day10, part2)]
pub fn solve_part2(input: InData) -> String {
    let instr_stream = convert_instrs_to_register_stream(input);
    let reg_stream = [(1, 1)].iter().chain(instr_stream.iter());
    let mut instr_iter = reg_stream.tuple_windows::<(_, _)>();
    let mut nxt = instr_iter.next();
    let mut cycle_cntr = 1;
    let mut disp = "".to_string();

    while let Some(curr_window) = nxt {
        let ((last_cyc, last_reg), (curr_cyc, _)) = curr_window;

        if cycle_cntr < *last_cyc {
            panic!("Cycle counter prior to expected range. Counter: {}, last_cyc: {}, curr_cyc: {}", cycle_cntr, last_cyc, curr_cyc);
        }
        if cycle_cntr >= *curr_cyc {
            nxt = instr_iter.next();
            continue;
        }

        let px_ptr = (cycle_cntr - 1) % 40;

        let diff = px_ptr.abs_diff(*last_reg);
        let px_on = diff <= 1;

        if px_on {
            disp.push('#');
        } else {
            disp.push('.');
        }

        if cfg!(debug_assertions) {
            println!("---- Cycle {:>5} {:->30}", cycle_cntr, "");
            println!("Current Register Window: {:?}", curr_window);
            dbg!(px_ptr);

            dbg!(diff);
            dbg!(px_on);

            let mut sprite_vis = "".to_string();

            for px in 0i32..40i32 {
                let diff = px.abs_diff(*last_reg);
                let px_on = diff <= 1;

                if px_on {
                    sprite_vis.push('#');
                } else {
                    sprite_vis.push('.');
                }
            }

            println!("Sprite:\n0123456789012345678901234567890123456789\n{}", sprite_vis);
            println!("disp:\n{}", disp.chars().chunks(40).into_iter().map(|mut ch| ch.join("")).join("\n"));
            println!("{:->45}", "");
        }

        cycle_cntr = cycle_cntr + 1;
    }

    disp = format_screen(&disp);
    println!("{}\n{:->50}", disp, "");
    disp
}

// Testing ----------------------------------------------------------
#[allow(unused)]
const TEST_IN: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

#[allow(unused)]
const TEST_OUT_2: &str = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

#[test]
pub fn test_part1() {
    assert_eq!(solve_part1(&input_generator(TEST_IN)), 13140);
}

#[test]
pub fn test_part2() {
    assert_eq!(solve_part2(&input_generator(TEST_IN)), TEST_OUT_2.trim());
}