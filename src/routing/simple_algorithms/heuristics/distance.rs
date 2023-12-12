use crate::{
    routing::graph::Graph,
    sphere::geometry::{arc::Arc, point::radians_to_meter},
};

use super::Heuristic;

#[derive(Clone)]
pub struct Distance {
    pub graph: Graph,
    pub source: u32,
}

impl Distance {
    pub fn new(graph: &Graph, source: u32) -> Distance {
        Distance {
            graph: graph.clone(),
            source,
        }
    }
}

impl Heuristic for Distance {
    fn lower_bound(&self, target: u32) -> u32 {
        radians_to_meter(
            Arc::new(
                &self.graph.nodes[self.source as usize],
                &self.graph.nodes[target as usize],
            )
            .central_angle(),
        )
        .round() as u32
    }
}
