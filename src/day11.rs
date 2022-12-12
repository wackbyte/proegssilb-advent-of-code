#[allow(unused_imports)]
use std::cmp::max;
use std::ops::{Mul, Add};
use aoc_runner_derive::aoc;
use itertools::Itertools;

type WorryType = u128;

pub struct Monkey {
    id: i32,
    items: Vec<WorryType>,
    operation: Box<dyn Fn(WorryType) -> WorryType>,
    throw_test: Box<dyn Fn(WorryType) -> bool>,
    target_true: usize,
    target_false: usize,
    items_inspected: u128,
}

pub type GenData = Vec<Monkey>;
pub type InData<'a> = &'a [Monkey];
pub type OutData = u128;

pub fn input_generator(input: &str) -> GenData {
    let regex = regex::Regex::new(r#"Monkey (\d+):
\s+Starting items: ([1234567890, ]+)
\s+Operation: new = (\w+|\d+) (.) (\w+|\d+)
\s+Test: divisible by (\d+)
\s+If true: throw to monkey (\d+)
\s+If false: throw to monkey (\d+)"#).unwrap();

    let mut results: Vec<Monkey> = Vec::new();

    for raw_monkey in regex.captures_iter(input) {
        let monkey_number: i32 = raw_monkey.get(1).unwrap().as_str().parse().unwrap();
        let starting_items = raw_monkey.get(2).unwrap().as_str().split(", ").map(|s| s.parse::<WorryType>().unwrap()).collect_vec();
        let op = make_op(raw_monkey.get(3).unwrap().as_str(), raw_monkey.get(4).unwrap().as_str(), raw_monkey.get(5).unwrap().as_str());
        let target_true = raw_monkey.get(7).unwrap().as_str().parse::<usize>().unwrap();
        let target_false = raw_monkey.get(8).unwrap().as_str().parse::<usize>().unwrap();
        let test_val: WorryType = raw_monkey.get(6).unwrap().as_str().parse().unwrap();
        let throw_test = Box::new(move |old: WorryType| old % test_val == 0);

        let monke = Monkey {id: monkey_number, items: starting_items, operation: op, items_inspected: 0, throw_test, target_false, target_true};
        results.push(monke);
    }

    results
}

fn make_op(left: &str, op: &str, right: &str) -> Box<dyn Fn(WorryType) -> WorryType> {
    let f = match op {
        "*" => <WorryType as Mul>::mul,
        "+" => <WorryType as Add>::add,
        _ => panic!("Invalid op: {}", op),
    };

    match (left, right) {
        ("old", "old") => Box::new(move |x| f(x, x)),
        ("old", _) => {
            let y: WorryType = right.parse().unwrap();
            Box::new(move |x| f(x, y))
        },
        (_, "old") => {
            let y: WorryType = left.parse().unwrap();
            Box::new(move |x| f(y, x))
        },
        _ => panic!("Invalid call detected (left, op, right): {} {} {}", left, op, right),
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> OutData {
    let mut monkeys = input_generator(input);
    monkeys.sort_by_key(|m| m.id);

    for round_num in 1..=20 {
        println!("{:#^50}", format!("Round {}", round_num));
        for idx in 0..monkeys.len() {
            process_turn(&mut monkeys, idx, true);
        }
    }

    monkeys.sort_by_key(|m| -(m.items_inspected as i128));

    monkeys[0].items_inspected * monkeys[1].items_inspected
}

fn process_turn(monkeys: &mut [Monkey], idx: usize, decrease_worry: bool) {
    {
        println!("Monkey {}", monkeys[idx].id);
    }

    let items = {
        let mut current_monkey = monkeys.get_mut(idx).unwrap();
        current_monkey.items_inspected += current_monkey.items.len() as u128;
        let items = std::mem::take(&mut current_monkey.items);
        items
    };
    for curr_item in items {
        let (target_idx, item) = {
            let current_monkey = &monkeys[idx];
            let mut item = curr_item;
            println!("\tMonkey inspects an item with worry level {}", item);
            item = current_monkey.operation.as_ref()(item);
            println!("\t\tWorry level shifts to {}", item);
            if decrease_worry {
                item /= 3;
                println!("\t\tMonkey gets bored with item. Worry level shifts to {}", item);
            }
            let check_result = current_monkey.throw_test.as_ref()(item);
            println!("\t\tCurrent worry level {} check", if check_result {"passes"} else {"fails"});
            let target_idx = if check_result {current_monkey.target_true} else {current_monkey.target_false};
            (target_idx, item)
        };
        println!("\t\tItem with worry level {} is thrown to monkey {}", item, target_idx);
        monkeys[target_idx].items.push(item);
    }
}

// #[aoc(day11, part2)]
// pub fn solve_part2(input: &str) -> OutData {
//     let mut monkeys = input_generator(input);
//     monkeys.sort_by_key(|m| m.id);

//     for round_num in 1..=10_000 {
//         println!("{:#^50}", format!("Round {}", round_num));
//         for idx in 0..monkeys.len() {
//             process_turn(&mut monkeys, idx, false);
//         }
//     }

//     monkeys.sort_by_key(|m| -(m.items_inspected as i128));

//     monkeys[0].items_inspected * monkeys[1].items_inspected
// }

// Testing ----------------------------------------------------------
#[allow(unused)]
const TEST_IN: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

#[test]
pub fn test_part1() {
    assert_eq!(solve_part1(&TEST_IN), 10605);
}

// #[test]
// pub fn test_part2() {
//     assert_eq!(solve_part2(&TEST_IN), 2713310158u128);
// }