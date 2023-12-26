use crate::{
    routing::fast_graph::FastGraph,
};

use super::Heuristic;

#[derive(Clone)]
pub struct Distance {
    pub graph: FastGraph,
    pub source: u32,
}

impl Distance {
    pub fn new(graph: &FastGraph, source: u32) -> Distance {
        Distance {
            graph: graph.clone(),
            source,
        }
    }
}

impl Heuristic for Distance {
    fn lower_bound(&self, _target: u32) -> u32 {
        // radians_to_meter(
        //     Arc::new(
        //         &self.graph.nodes[self.source as usize],
        //         &self.graph.nodes[target as usize],
        //     )
        //     .central_angle(),
        // )
        // .round() as u32
        0
    }
}
