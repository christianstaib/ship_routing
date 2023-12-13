use std::usize;

use indicatif::ProgressIterator;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::routing::{
    fast_graph::{FastEdgeAccess, FastGraph},
    graph::{Edge, Graph},
};

use super::ch_helper::ChHelper;

#[derive()]
pub struct Contractor {
    pub graph: Graph,
    pub levels: Vec<u32>,
    pub shortcuts: Vec<(Edge, [Edge; 2])>,
}

impl Contractor {
    pub fn new(graph: Graph) -> Contractor {
        let levels = vec![u32::MAX; graph.nodes.len()];
        Contractor {
            graph,
            levels,
            shortcuts: Vec::new(),
        }
    }

    pub fn contract(&mut self) {
        let graph = self.graph.clone();
        for (level, node) in (0..self.graph.nodes.len()).enumerate().progress() {
            self.contract_node(node as u32);

            self.levels[node] = level as u32;
        }

        self.graph = graph;
        self.shortcuts
            .iter()
            .map(|(_, edges)| edges)
            .flatten()
            .for_each(|edge| self.graph.add_edge(edge));
    }

    pub fn get_fast_graph(&self) -> FastGraph {
        let forward_edges = self
            .graph
            .forward_edges
            .iter()
            .flatten()
            .filter(|&edge| self.levels[edge.source as usize] >= self.levels[edge.target as usize])
            .cloned()
            .collect();
        let backward_edges = self
            .graph
            .backward_edges
            .iter()
            .flatten()
            .filter(|&edge| self.levels[edge.source as usize] <= self.levels[edge.target as usize])
            .map(|edge| edge.get_inverted())
            .collect();
        FastGraph {
            nodes: self.graph.nodes.clone(),
            forward_edges: FastEdgeAccess::new(&forward_edges),
            backward_edges: FastEdgeAccess::new(&backward_edges),
        }
    }

    /// Generates shortcuts for a node and removes it from the graph.
    ///
    /// Removing means, that afterwards, there will be no edges going into node or going out of
    /// node. The generated shortcuts are added to the graph.
    fn contract_node(&mut self, node: u32) {
        let shortcuts_for_node = self.get_shortcuts(node);
        shortcuts_for_node
            .iter()
            .map(|(_, edges)| edges)
            .flatten()
            .for_each(|edge| self.graph.add_edge(edge));
        self.shortcuts.extend(shortcuts_for_node);
        self.graph.remove(node);
    }

    /// Generates shortcuts for a node.
    ///
    /// A shortcut (s -> t) for (s -> node -> t) will be generated, if (s -> node -> t) is the
    /// shortest path from s to t.
    ///
    /// The first element of the tupples in the returned vec is the shortcut, the second the edges
    /// it cuts short.
    fn get_shortcuts(&self, node: u32) -> Vec<(Edge, [Edge; 2])> {
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
                            let k = Edge {
                                source: incoming_edge.source,
                                target: outgoing_edge.target,
                                cost: incoming_edge.cost + outgoing_edge.cost,
                            };
                            let v = [incoming_edge.clone(), outgoing_edge.clone()];
                            shortcuts.push((k, v));
                        }
                    });
                }
                shortcuts
            })
            .flatten()
            .collect()
    }
}
