use std::collections::HashSet;

use crate::routing::graph::Graph;

use super::queue::PriorityTerm;

pub struct DeletedNeighbors {
    deleted: Vec<bool>,
}

impl PriorityTerm for DeletedNeighbors {
    fn priority(&self, v: u32, graph: &Graph) -> i32 {
        let neighbors: HashSet<_> = graph.forward_edges[v as usize]
            .iter()
            .map(|edge| edge.target)
            .collect();
        neighbors
            .iter()
            .filter(|&&neighbor| self.deleted[neighbor as usize] == true)
            .count() as i32
    }

    #[allow(unused_variables)]
    fn update(&mut self, v: u32) {
        self.deleted[v as usize] = true;
    }
}

impl DeletedNeighbors {
    pub fn new(num_nodes: u32) -> Self {
        Self {
            deleted: vec![false; num_nodes as usize],
        }
    }
}
