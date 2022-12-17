use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*};
#[allow(unused_imports)]
use std::cmp::max;

pub struct ParseResults {
    graph: GraphType,
    starting: NodeIndex<usize>,
    ending: NodeIndex<usize>,
}

pub type CellType = char;
pub type GraphType = DiGraph<CellType, (), usize>;
pub type GenData = ParseResults;
pub type InData<'a> = &'a ParseResults;
pub type OutData = u64;

fn check_add_edges(
    graph: &mut GraphType,
    cell: &CellType,
    cell_idx: &NodeIndex<usize>,
    last_cell: &CellType,
    last_idx: &NodeIndex<usize>,
) {
    let diff = (*last_cell as i16) - (*cell as i16);
    let sig = diff.signum();
    match sig {
        -1 => {
            graph.add_edge(*cell_idx, *last_idx, ());
            if diff == -1 {
                graph.add_edge(*last_idx, *cell_idx, ());
            }
        }
        1 => {
            graph.add_edge(*last_idx, *cell_idx, ());
            if diff == 1 {
                graph.add_edge(*cell_idx, *last_idx, ());
            }
        }
        0 => {
            graph.add_edge(*cell_idx, *last_idx, ());
            graph.add_edge(*last_idx, *cell_idx, ());
        }
        _ => {
            panic!("signum() returned an unexpected value: {}", sig)
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> GenData {
    let mut results: GraphType = DiGraph::default();

    let input = input.trim_start();

    let mut input = input
        .lines()
        .map(|ln| ln.chars().collect_vec())
        .collect_vec();
    let mut indices = input
        .iter()
        .map(|ln| ln.iter().map(|_| 0 as usize).collect_vec())
        .collect_vec();

    let mut starting_coords = (0, 0);
    let mut ending_coords = (0, 0);

    for (ln_idx, ln) in input.iter_mut().enumerate() {
        for (col_idx, cell) in ln.iter_mut().enumerate() {
            if *cell == 'S' {
                starting_coords = (ln_idx, col_idx);
                *cell = 'a';
            } else if *cell == 'E' {
                ending_coords = (ln_idx, col_idx);
                *cell = 'z';
            }
        }
    }

    for (ln_idx, ln) in input.iter().enumerate() {
        for (col_idx, cell) in ln.iter().enumerate() {
            let idx = results.add_node(*cell);
            indices[ln_idx][col_idx] = idx.index();

            if col_idx > 0 {
                let last_cell = input[ln_idx][col_idx - 1];
                let last_idx = NodeIndex::from(indices[ln_idx][col_idx - 1]);

                check_add_edges(&mut results, cell, &idx, &last_cell, &last_idx);
            }

            if ln_idx > 0 {
                let last_cell = input[ln_idx - 1][col_idx];
                let last_idx = NodeIndex::from(indices[ln_idx - 1][col_idx]);

                check_add_edges(&mut results, cell, &idx, &last_cell, &last_idx);
            }
        }
    }

    let starting = NodeIndex::from(indices[starting_coords.0][starting_coords.1]);
    let ending = NodeIndex::from(indices[ending_coords.0][ending_coords.1]);

    // dbg!(starting_coords);
    // dbg!(starting);
    // dbg!(ending_coords);
    // dbg!(ending);
    // dbg!(&results);

    ParseResults {
        graph: results,
        starting,
        ending,
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(input: InData) -> OutData {
    let graph = &input.graph;
    let starting_idx = input.starting;
    let ending_idx = input.ending;

    let results = dijkstra(graph, starting_idx, Some(ending_idx), |_| 1u64);

    // dbg!(&results);

    *results
        .get(&ending_idx)
        .unwrap_or_else(|| panic!("Could not locate ending node in explored part of graph."))
}

#[aoc(day12, part2)]
pub fn solve_part2(input: InData) -> OutData {
    let mut graph = input.graph.clone();
    let ending_idx = input.ending;

    // Get the starting points...
    let possible_starts = {
        let node_list = graph.raw_nodes();
        graph
            .node_indices()
            .map(|idx| {
                (
                    idx,
                    node_list
                        .get(idx.index())
                        .unwrap_or_else(|| panic!("Could not find node in graph {}", idx.index())),
                )
            })
            .filter(|(_, a)| a.weight == 'a')
            .map(|(idx, _)| idx)
            .collect_vec()
    };

    // Reverse the edges so that the ending point is where we can start the search
    graph.reverse();

    let search_results = dijkstra(&graph, ending_idx, None, |_| 1u64);

    possible_starts
        .iter()
        .filter(|idx| search_results.contains_key(idx))
        .map(|idx| {
            *search_results.get(idx).unwrap_or_else(|| {
                panic!("Could not locate node {} in search results.", idx.index())
            })
        })
        .min()
        .unwrap_or_else(|| panic!("No possible starts found in search results."))
}

#[allow(unused)]
const TEST_IN: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

#[test]
pub fn test_d12_part1() {
    assert_eq!(solve_part1(&input_generator(TEST_IN)), 31);
}

#[test]
pub fn test_d12_part2() {
    assert_eq!(solve_part2(&input_generator(TEST_IN)), 29);
}
