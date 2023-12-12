use std::{collections::HashMap, usize};

use indicatif::ProgressIterator;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::routing::{
    graph::{Edge, Graph},
    queue::heap_queue::HeapQueue,
    route::RouteRequest,
};

use super::ch_helper::ChHelper;

#[derive()]
pub struct Contractor {
    pub graph: Graph,
    pub levels: Vec<u32>,
    pub shortcuts: HashMap<(u32, u32), Vec<(u32, u32, u32)>>,
}

impl Contractor {
    pub fn new(graph: Graph) -> Contractor {
        let levels = vec![u32::MAX; graph.nodes.len()];
        Contractor {
            graph,
            levels,
            shortcuts: HashMap::new(),
        }
    }

    pub fn contract(&mut self) {
        let graph = self.graph.clone();
        let mut shortcuts = Vec::new();
        for (level, node) in (0..self.graph.nodes.len()).enumerate().progress() {
            let this_shortcuts = self.contract_node(node as u32);
            for (_, shortcuts) in this_shortcuts.iter() {
                for (s, t, c) in shortcuts {
                    let edge = Edge {
                        source: *s,
                        target: *t,
                        cost: *c,
                    };
                    self.graph.forward_edges[edge.source as usize].push(edge.clone());
                    self.graph.backward_edges[edge.target as usize].push(edge.clone());
                }
            }
            shortcuts.extend(this_shortcuts);
            self.levels[node] = level as u32;

            if level >= 2_000_000 {
                break;
            }
        }

        self.graph = graph;
        for (_, shortcuts) in shortcuts.iter() {
            for (s, t, c) in shortcuts {
                let edge = Edge {
                    source: *s,
                    target: *t,
                    cost: *c,
                };
                self.graph.forward_edges[edge.source as usize].push(edge.clone());
                self.graph.backward_edges[edge.target as usize].push(edge.clone());
            }
        }
    }

    fn contract_node(&mut self, node: u32) -> Vec<((u32, u32), Vec<(u32, u32, u32)>)> {
        let shortcuts = self.get_shortcuts(node);
        self.remove(node);

        shortcuts
    }

    pub fn get_cost(&self, request: &RouteRequest) -> Option<u32> {
        let mut f_queue = HeapQueue::new();
        let mut f_cost = vec![u32::MAX; self.graph.nodes.len()];
        // let mut f_predecessor = HashMap::new();
        f_queue.insert(0, request.source);
        f_cost[request.source as usize] = 0;

        let mut b_queue = HeapQueue::new();
        let mut b_cost = vec![u32::MAX; self.graph.nodes.len()];
        // let mut b_predecessor = HashMap::new();
        b_queue.insert(0, request.target);
        b_cost[request.target as usize] = 0;

        let mut i = 0;
        while !f_queue.is_empty() || !f_queue.is_empty() {
            println!("{}", i);
            i += 1;
            if let Some(f_state) = f_queue.pop() {
                self.graph.forward_edges[f_state.value as usize]
                    .iter()
                    .filter(|edge| {
                        self.levels[edge.source as usize] <= self.levels[edge.target as usize]
                    })
                    .for_each(|edge| {
                        let alteranativ_cost = f_cost[f_state.value as usize] + edge.cost;
                        if alteranativ_cost < f_cost[edge.target as usize] {
                            assert!(
                                self.levels[f_state.value as usize]
                                    <= self.levels[edge.target as usize]
                            );
                            f_cost[edge.target as usize] = alteranativ_cost;
                            f_queue.insert(alteranativ_cost, edge.target)
                        }
                    });
            }

            if let Some(b_state) = b_queue.pop() {
                self.graph.backward_edges[b_state.value as usize]
                    .iter()
                    .filter(|edge| {
                        self.levels[edge.source as usize] >= self.levels[edge.target as usize]
                    })
                    .map(|edge| edge.get_inverted())
                    .for_each(|edge| {
                        let alteranativ_cost = b_cost[b_state.value as usize] + edge.cost;
                        if alteranativ_cost < b_cost[edge.target as usize] {
                            assert!(
                                self.levels[b_state.value as usize]
                                    >= self.levels[edge.target as usize]
                            );
                            b_cost[edge.target as usize] = alteranativ_cost;
                            b_queue.insert(alteranativ_cost, edge.target)
                        }
                    });
            }
        }

        (0..self.graph.nodes.len())
            .filter(|&i| f_cost[i as usize] != u32::MAX && b_cost[i as usize] != u32::MAX)
            .map(|i| f_cost[i] + b_cost[i])
            .min()
    }

    fn edge_difference(&self, node: u32) -> i32 {
        let outgoing_edges = &self.graph.forward_edges[node as usize];
        let incoming_edges = &self.graph.backward_edges[node as usize];

        let current_num = outgoing_edges.len() + incoming_edges.len();
        let shortcut_num = self.get_shortcuts(node).len();

        shortcut_num as i32 - current_num as i32
    }

    fn get_shortcuts(&self, node: u32) -> Vec<((u32, u32), Vec<(u32, u32, u32)>)> {
        let outgoing_edges = &self.graph.forward_edges[node as usize];
        let incoming_edges = &self.graph.backward_edges[node as usize];

        let ch_dijkstra = ChHelper::new(&self.graph);

        incoming_edges
            .iter()
            .par_bridge()
            .map(|incoming_edge| {
                let mut shortcuts = Vec::new();
                if let Some(max_outgoing_cost) = outgoing_edges.iter().map(|edge| edge.cost).max() {
                    let max_cost = incoming_edge.cost + max_outgoing_cost;
                    let cost = ch_dijkstra.costs_without(incoming_edge.source, max_cost, node);

                    outgoing_edges.iter().for_each(|outgoing_edge| {
                        let pair_cost = incoming_edge.cost + outgoing_edge.cost;

                        // shortcut needed
                        if &pair_cost < cost.get(&outgoing_edge.target).unwrap_or(&u32::MAX) {
                            let k = (incoming_edge.source, outgoing_edge.target);
                            let v = vec![
                                (incoming_edge.source, node, pair_cost),
                                (node, outgoing_edge.target, pair_cost),
                            ];
                            shortcuts.push((k, v));
                        }
                    });
                }
                shortcuts
            })
            .flatten()
            .collect()
    }

    fn remove(&mut self, node: u32) {
        let outgoing_edges = std::mem::take(&mut self.graph.forward_edges[node as usize]);
        outgoing_edges.iter().for_each(|outgoing_edge| {
            self.graph.backward_edges[outgoing_edge.target as usize]
                .retain(|backward_edge| backward_edge != outgoing_edge)
        });

        let incoming_edges = std::mem::take(&mut self.graph.backward_edges[node as usize]);
        incoming_edges.iter().for_each(|incoming_edge| {
            self.graph.forward_edges[incoming_edge.source as usize]
                .retain(|forward_edge| forward_edge != incoming_edge)
        });
    }
}
