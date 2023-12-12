use std::{collections::HashMap, usize};

use indicatif::ProgressIterator;

use crate::routing::graph::{Edge, Graph};

pub struct Contractor {
    pub graph: Graph,
    pub levels: Vec<u32>,
    pub shortcuts: HashMap<(u32, u32), Vec<(u32, u32)>>,
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
        for (level, node) in (0..self.graph.nodes.len()).enumerate().progress() {
            self.contract_node(node as u32);
            self.levels[node] = level as u32;
        }
    }

    fn contract_node(&mut self, node: u32) {
        let outgoing_edges = self.remove_outgoung_edges(node);
        let incoming_edges = self.remove_incoming_edges(node);

        let ch_dijkstra = ChHelper::new(&self.graph);
        incoming_edges.iter().for_each(|incoming_edge| {
            if let Some(max_outgoing_cost) = outgoing_edges.iter().map(|edge| edge.cost).max() {
                let max_cost = incoming_edge.cost + max_outgoing_cost;
                let cost = ch_dijkstra.costs(incoming_edge.source, max_cost);

                outgoing_edges.iter().for_each(|outgoing_edge| {
                    let pair_cost = incoming_edge.cost + outgoing_edge.cost;

                    // shortcut needed
                    if &pair_cost < cost.get(&outgoing_edge.target).unwrap_or(&u32::MAX) {
                        let k = (incoming_edge.source, outgoing_edge.target);
                        let v = vec![(incoming_edge.source, node), (node, outgoing_edge.target)];
                        self.shortcuts.insert(k, v);
                    }
                });
            }
        });
    }

    fn remove_outgoung_edges(&mut self, node: u32) -> Vec<Edge> {
        let outgoing_edges = std::mem::take(&mut self.graph.forward_edges[node as usize]);
        outgoing_edges.iter().for_each(|outgoing_edge| {
            self.graph.backward_edges[outgoing_edge.target as usize]
                .retain(|backward_edge| backward_edge != outgoing_edge)
        });
        outgoing_edges
    }

    fn remove_incoming_edges(&mut self, node: u32) -> Vec<Edge> {
        let incoming_edges = std::mem::take(&mut self.graph.backward_edges[node as usize]);
        incoming_edges.iter().for_each(|incoming_edge| {
            self.graph.forward_edges[incoming_edge.source as usize]
                .retain(|forward_edge| forward_edge != incoming_edge)
        });
        incoming_edges
    }
}
