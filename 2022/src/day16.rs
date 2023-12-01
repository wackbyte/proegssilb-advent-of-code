use aoc_zen_runner_macros::{aoc, generator, solver};
use bit_set::*;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::VecDeque;

use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};

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

// Solution ---------------------------------------------------------
#[aoc(2022, day16)]
pub mod solutions {
    use super::*;

    pub type GenData = Graph<NodeData, EdgeData, Undirected>;
    pub type OutData = i64;

    #[generator(tuple_unpack)]
    pub fn input_generator(input: &str) -> GenData {
        let mut results: GenData = Graph::default();
        let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

        for ln in input.lines() {
            if ln == "" {
                continue;
            }

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
                    None => {
                        continue;
                    }
                    Some(index) => {
                        results.add_edge(idx, *index, EdgeData { cost: 1 });
                    }
                }
            }
        }

        results
    }

    #[derive(Debug, PartialEq, Eq)]
    struct VisitData {
        node_index: NodeIndex,
        nodes_on: BitSet,
        current_score: i64,
        time_remaining: u8,
        path_valves: Vec<String>,
    }

    impl Ord for VisitData {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let a = self.current_score * (self.time_remaining as i64 + 1);
            let b = other.current_score * (other.time_remaining as i64 + 1);
            a.cmp(&b)
        }
    }

    impl PartialOrd for VisitData {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    type NodeDistanceMap = HashMap<NodeIndex, HashMap<NodeIndex, i64>>;

    #[solver(part1, bfs)]
    pub fn solve_part1_bfs(graph: GenData) -> OutData {
        let start_node_idx = graph
            .node_indices()
            .map(|idx| (idx, graph.node_weight(idx).unwrap()))
            .filter(|(_, weight)| weight.valve_id == "AA")
            .map(|(idx, _)| idx)
            .at_most_one()
            .unwrap()
            .unwrap();

        let mut work_queue: VecDeque<VisitData> = VecDeque::new();
        let max_valves = graph.node_count();

        let mut node_options_map: NodeDistanceMap = HashMap::new();

        for idx in graph.node_indices() {
            let results = dijkstra(&graph, idx, None, |e| e.weight().cost);
            node_options_map.entry(idx).or_insert(results);
        }

        work_queue.push_back(VisitData {
            node_index: start_node_idx,
            nodes_on: BitSet::with_capacity(graph.node_count()),
            current_score: 0,
            time_remaining: 30,
            path_valves: vec!["AA".to_owned()],
        });

        let mut max_score: i64 = 0;

        // Swap the line comments for DFS vs. BFS
        //while let Some(c_visit) = work_queue.pop_front() { // BFS / queue
        while let Some(c_visit) = work_queue.pop_back() {
            // DFS / stack
            max_score = max(max_score, c_visit.current_score);
            if c_visit.time_remaining < 2 || c_visit.nodes_on.len() == max_valves {
                continue;
            }

            // let c_node = graph.node_weight(c_visit.node_index).unwrap();
            // println!("    Visiting node: {:?} - {:?} ({: >3}m, {: >7}pts )  \t-\t[{:?}]", c_visit.node_index, c_node.valve_id, c_visit.time_remaining, c_visit.current_score, &c_visit.path_valves);

            let neighbors = node_options_map[&c_visit.node_index]
                .keys()
                .filter(|idx| !c_visit.nodes_on.contains(idx.index()))
                .filter(|idx| graph.node_weight(**idx).unwrap().flow_rate != 0)
                .map(|i| *i);

            for n_idx in neighbors {
                let n_data = graph.node_weight(n_idx).unwrap();
                let route_cost = node_options_map[&c_visit.node_index][&n_idx];

                if c_visit.time_remaining as i64 >= 1 + route_cost {
                    let new_time = c_visit.time_remaining as i64 - route_cost - 1;
                    let new_score = c_visit.current_score + n_data.flow_rate * (new_time as i64);
                    let mut new_nodes_on = c_visit.nodes_on.clone();
                    new_nodes_on.insert(n_idx.index());
                    let mut new_path_valves = c_visit.path_valves.clone();
                    new_path_valves.push(n_data.valve_id.clone());

                    let move_and_on_data = VisitData {
                        node_index: n_idx,
                        nodes_on: new_nodes_on,
                        current_score: new_score,
                        time_remaining: new_time as u8,
                        path_valves: new_path_valves.clone(),
                    };

                    work_queue.push_back(move_and_on_data);
                }
            }
        }

        max_score
    }

    #[derive(Debug, PartialEq, Eq)]
    struct VisitData2 {
        idx: [NodeIndex; 2],
        eta: [u8; 2],
        nodes_on: BitSet,
        current_score: i64,
        time_remaining: u8,
    }

    impl VisitData2 {
        fn ready_count(&self) -> usize {
            self.eta
                .iter()
                .filter(|eta| *eta <= &self.time_remaining)
                .count()
        }

        fn which_ready(&self) -> Option<usize> {
            self.eta
                .iter()
                .enumerate()
                .filter(|(_, eta)| *eta <= &self.time_remaining)
                .map(|(i, _)| i)
                .exactly_one()
                .ok()
        }
    }

    fn get_node_targets<'a>(
        graph: &'a GenData,
        nodes_on: &'a BitSet,
    ) -> impl Iterator<Item = NodeIndex> + 'a {
        graph
            .node_indices()
            .filter(move |idx| !nodes_on.contains(idx.index()))
            .filter(|idx| graph.node_weight(*idx).unwrap().flow_rate != 0)
    }

    fn actor_can_reach(
        node_map: &NodeDistanceMap,
        actor_eta: u8,
        c_idx: NodeIndex,
        t_idx: NodeIndex,
    ) -> bool {
        node_map[&c_idx][&t_idx] + 1 < actor_eta as i64
    }

    fn get_next_visit(
        graph: &GenData,
        node_map: &NodeDistanceMap,
        c_visit: &VisitData2,
        which_actor: usize,
        n_idx: NodeIndex,
    ) -> VisitData2 {
        let c_idx = c_visit.idx[which_actor];
        let n_data = graph.node_weight(n_idx).unwrap();
        let route_cost = node_map[&c_idx][&n_idx] + 1;

        let new_time = c_visit.time_remaining as i64 - route_cost;
        let new_score = c_visit.current_score + n_data.flow_rate * (new_time as i64);
        let mut new_nodes_on = c_visit.nodes_on.clone();
        new_nodes_on.insert(n_idx.index());
        let new_idx = match which_actor {
            0 => [n_idx, c_visit.idx[1]],
            1 => [c_visit.idx[0], n_idx],
            _ => panic!("This shouldn't happen."),
        };
        let new_eta = match which_actor {
            0 => [c_visit.eta[0] - route_cost as u8, c_visit.eta[1]],
            1 => [c_visit.eta[0], c_visit.eta[1] - route_cost as u8],
            _ => panic!("This shouldn't happen."),
        };

        let move_and_on_data = VisitData2 {
            idx: new_idx,
            eta: new_eta,
            nodes_on: new_nodes_on,
            current_score: new_score,
            time_remaining: new_time as u8,
        };

        move_and_on_data
    }

    fn get_next_visit2(
        graph: &GenData,
        node_map: &NodeDistanceMap,
        c_visit: &VisitData2,
        z_idx: NodeIndex,
        o_idx: NodeIndex,
    ) -> VisitData2 {
        let [z_c_idx, o_c_idx] = c_visit.idx;
        let z_data = graph.node_weight(z_idx).unwrap();
        let o_data = graph.node_weight(o_idx).unwrap();
        let z_route_cost = node_map[&z_c_idx][&z_idx] + 1;
        let o_route_cost = node_map[&o_c_idx][&o_idx] + 1;

        let new_eta = [
            c_visit.eta[0] - z_route_cost as u8,
            c_visit.eta[1] - o_route_cost as u8,
        ];
        let new_time = c_visit.time_remaining as i64 - *new_eta.iter().min().unwrap() as i64;
        let new_score = c_visit.current_score
            + z_data.flow_rate * (new_eta[0] as i64)
            + o_data.flow_rate * (new_eta[1] as i64);
        let mut new_nodes_on = c_visit.nodes_on.clone();
        new_nodes_on.insert(z_idx.index());
        new_nodes_on.insert(o_idx.index());
        let new_idx = [z_idx, o_idx];

        let move_and_on_data = VisitData2 {
            idx: new_idx,
            eta: new_eta,
            nodes_on: new_nodes_on,
            current_score: new_score,
            time_remaining: new_time as u8,
        };

        move_and_on_data
    }


    #[solver(part2, dijkstra_queue)]
    pub fn solve_part2(graph: GenData) -> OutData {
        let start_node_idx = graph
            .node_indices()
            .map(|idx| (idx, graph.node_weight(idx).unwrap()))
            .filter(|(_, weight)| weight.valve_id == "AA")
            .map(|(idx, _)| idx)
            .at_most_one()
            .unwrap()
            .unwrap();

        let mut work_queue: VecDeque<VisitData2> = VecDeque::new();
        let max_valves = graph.node_count();

        let mut node_options_map: NodeDistanceMap = HashMap::new();

        for idx in graph.node_indices() {
            let results = dijkstra(&graph, idx, None, |e| e.weight().cost);
            node_options_map.entry(idx).or_insert(results);
        }

        // dbg!(&node_options_map);

        work_queue.push_back(VisitData2 {
            idx: [start_node_idx, start_node_idx],
            eta: [26, 26],
            nodes_on: BitSet::with_capacity(graph.node_count()),
            current_score: 0,
            time_remaining: 26,
        });

        let mut max_score: i64 = 0;

        // Swap the line comments for DFS vs. BFS
        //while let Some(c_visit) = work_queue.pop_front() { // BFS / queue
        while let Some(c_visit) = work_queue.pop_back() {
            // DFS / stack
            max_score = max(max_score, c_visit.current_score);
            if c_visit.time_remaining < 2 || c_visit.nodes_on.len() == max_valves {
                continue;
            }

            match c_visit.ready_count() {
                0 => {
                    // This probably shouldn't happen, but what the hey...
                    let next_clock = *c_visit.eta.iter().max().unwrap();
                    let next_visit = VisitData2 {
                        time_remaining: next_clock,
                        ..c_visit
                    };
                    work_queue.push_back(next_visit);
                }
                1 => {
                    let which = c_visit.which_ready().unwrap();
                    for t in get_node_targets(&graph, &c_visit.nodes_on) {
                        if actor_can_reach(
                            &node_options_map,
                            c_visit.eta[which],
                            c_visit.idx[which],
                            t,
                        ) {
                            let n_visit =
                                get_next_visit(&graph, &node_options_map, &c_visit, which, t);
                            work_queue.push_back(n_visit);
                        }
                    }
                }
                2 => {
                    let targets: BitSet = get_node_targets(&graph, &c_visit.nodes_on)
                        .map(|ni| ni.index())
                        .collect();
                    let mut zero_targets: BitSet = targets
                        .iter()
                        .filter(|t| {
                            actor_can_reach(
                                &node_options_map,
                                c_visit.eta[0],
                                c_visit.idx[0],
                                NodeIndex::new(*t),
                            )
                        })
                        .collect();
                    let mut one_targets: BitSet = targets
                        .iter()
                        .filter(|t| {
                            actor_can_reach(
                                &node_options_map,
                                c_visit.eta[1],
                                c_visit.idx[1],
                                NodeIndex::new(*t),
                            )
                        })
                        .collect();
                    let common_targets = zero_targets.intersection(&one_targets).collect();
                    zero_targets.difference_with(&common_targets);
                    one_targets.difference_with(&common_targets);

                    // Let's first queue up as if Zero takes an exclusive target
                    if zero_targets.len() > 0 {
                        let one_rest = one_targets.union(&common_targets);
                        for z_choice in &zero_targets {
                            for o_choice in one_rest.clone() {
                                let z_idx = NodeIndex::new(z_choice);
                                let o_idx = NodeIndex::new(o_choice);
                                let nv = get_next_visit2(
                                    &graph,
                                    &node_options_map,
                                    &c_visit,
                                    z_idx,
                                    o_idx,
                                );
                                work_queue.push_back(nv);
                            }
                        }
                    }

                    // Now for if One takes an exclusive target
                    if one_targets.len() > 0 {
                        let zero_rest = zero_targets.union(&common_targets);
                        for z_choice in zero_rest {
                            for o_choice in &one_targets {
                                let z_idx = NodeIndex::new(z_choice);
                                let o_idx = NodeIndex::new(o_choice);
                                let nv = get_next_visit2(
                                    &graph,
                                    &node_options_map,
                                    &c_visit,
                                    z_idx,
                                    o_idx,
                                );
                                work_queue.push_back(nv);
                            }
                        }
                    }

                    // Finally if both take a common target
                    for combo in common_targets.iter().combinations(2) {
                        let [z_choice, o_choice] = combo[..] else {
                            todo!()
                        };
                        let z_idx = NodeIndex::new(z_choice);
                        let o_idx = NodeIndex::new(o_choice);
                        let nv = get_next_visit2(&graph, &node_options_map, &c_visit, z_idx, o_idx);
                        work_queue.push_back(nv);
                    }
                }
                _ => panic!("This can't actually happen"),
            }
        }

        max_score
    }
}

#[cfg(test)]
pub mod test {
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(1651, 1707)]
    const test_in: &str = r#"
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
}
