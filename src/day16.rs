
use std::collections::{HashSet, BinaryHeap};
use std::{cmp::max, collections::HashMap};
use aoc_runner_derive::{aoc_generator, aoc};
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::{Graph, Undirected, graph};
use petgraph::graph::NodeIndex;

#[derive(Debug)]
pub struct NodeData {
    valve_id: String,
    flow_rate: i64,
    cost: i64,
}

#[derive(Debug)]
pub struct EdgeData {
    cost: i64,
}

pub type GenData = Graph<NodeData, EdgeData, Undirected>;
pub type InData<'a> = &'a GenData;
pub type OutData = i64;


// Solution ---------------------------------------------------------

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> GenData {
    let mut results: GenData = Graph::default();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

    for ln in input.lines() {
        if ln == "" { continue; }

        let (_, rest) = ln.split_once(' ').unwrap();
        let (valve_id, rest) = rest.split_once(' ').unwrap();
        let (_, rest) = rest.split_once('=').unwrap();
        let (rate, rest) = rest.split_once(';').unwrap();
        let (_, rest) = rest.split_once(" to ").unwrap();
        let (_, connecting_list) = rest.split_once(" ").unwrap();
        let connecting_list = connecting_list.split(", ").collect_vec();

        let node = NodeData {
            valve_id: valve_id.to_owned(),
            flow_rate: rate.parse().unwrap(),
            cost: 1,
        };

        let idx = results.add_node(node);
        nodes.insert(valve_id, idx);
        for node_id in connecting_list {
            match nodes.get(node_id) {
                // Undirected graph. If the node isn't in the dictionary, we haven't visited said node,
                // and we'll make the connection when we visit the other node.
                None => { continue; }
                Some(index) => {
                    results.add_edge(idx, *index, EdgeData {
                        cost: 1,
                    });
                }
            }
        }
    }

    results
}

#[derive(Debug, PartialEq, Eq)]
struct VisitData {
    node_index: NodeIndex,
    nodes_on: HashSet<NodeIndex>,
    current_score: i64,
    time_remaining: i64,
    path_idx: Vec<NodeIndex>,
    path_valves: Vec<String>,
}

impl Ord for VisitData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.current_score * (self.time_remaining + 1);
        let b = other.current_score * (other.time_remaining + 1);
        a.cmp(&b)
    }
}

impl PartialOrd for VisitData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(graph: InData) -> OutData {
    let start_node_idx = graph.node_indices()
        .map(|idx| (idx, graph.node_weight(idx).unwrap()))
        .filter(|(idx, weight)| weight.valve_id == "AA")
        .map(|(idx, _)| idx)
        .at_most_one()
        .unwrap()
        .unwrap();

    let mut heap: BinaryHeap<VisitData> = BinaryHeap::new();
    let max_valves = graph.node_count();

    let mut node_options_map: HashMap<NodeIndex, HashMap<NodeIndex, i64>> = HashMap::new();

    for idx in graph.node_indices() {
        let results = dijkstra(&graph, idx, None, |e| e.weight().cost);
        node_options_map.entry(idx).or_insert(results);
    }

    dbg!(&node_options_map);

    heap.push(VisitData {
        node_index: start_node_idx,
        nodes_on: HashSet::new(),
        current_score: 0,
        time_remaining: 30,
        path_idx: Vec::new(),
        path_valves: vec!["AA".to_owned()]
    });

    let mut max_score: i64 = 0;

    while let Some(current_node) = heap.pop() {
        if current_node.time_remaining < 2 || current_node.nodes_on.len() == max_valves {
            println!("Found 'winning' node:");
            dbg!(&current_node);
            //return current_node.current_score;
            max_score = max(max_score, current_node.current_score);
        }
        let current_data = graph.node_weight(current_node.node_index).unwrap();
        println!("Visiting node: {:?} - {:?} ({: >3}m )  \t-\t[{:?}]", current_node.node_index, current_data.valve_id, current_node.time_remaining, &current_node.path_valves);

        let neighbors = node_options_map[&current_node.node_index].clone();

        for n_idx in neighbors.keys() {
            // Check that this is a good idea
            let cost = neighbors.get(n_idx).unwrap_or(&9_999_999i64).saturating_add(1i64);
            let n_data = graph.node_weight(*n_idx).unwrap();
            println!("    Examining {} -> {} (cost {})", current_data.valve_id, n_data.valve_id, cost);
            if current_node.nodes_on.contains(&n_idx) {
                continue;
            }
            if current_node.time_remaining < cost {
                continue;
            }
            if n_data.flow_rate == 0 {
                continue;
            }

            // OK, let's queue up the work...
            let new_time = current_node.time_remaining - cost;
            let new_score = current_node.current_score + n_data.flow_rate * new_time;
            let mut new_nodes_on = current_node.nodes_on.clone();
            new_nodes_on.insert(*n_idx);
            let mut new_path_idx = current_node.path_idx.clone();
            new_path_idx.push(*n_idx);
            let mut new_path_valves = current_node.path_valves.clone();
            new_path_valves.push(n_data.valve_id.clone());
            heap.push(VisitData {
                node_index: *n_idx,
                current_score: new_score,
                time_remaining: new_time,
                nodes_on: new_nodes_on,
                path_idx: new_path_idx,
                path_valves: new_path_valves,
            });
        }
    }

    max_score
}

// #[aoc(day16, part2)]
// pub fn solve_part2(input: InData) -> OutData {
//     todo!()
// }

#[allow(unused)]
const TEST_IN: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

#[test]
pub fn test_part1() {
    assert_eq!(solve_part1(&input_generator(TEST_IN)), 1651);
}

// #[test]
// pub fn test_part2() {
//     assert_eq!(solve_part2(&input_generator(TEST_IN)), _Z);
// }
