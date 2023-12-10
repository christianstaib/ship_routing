use crate::{
    geometry::{radians_to_meter, Arc},
    routing::{graph, Graph},
};

use super::Heuristic;

#[derive(Clone)]
pub struct Distance {
    pub graph: Graph,
}

impl Distance {
    pub fn new(graph: &Graph) -> Distance {
        Distance {
            graph: graph.clone(),
        }
    }
}

impl Heuristic for Distance {
    fn lower_bound(&self, source: u32, target: u32) -> u32 {
        radians_to_meter(
            Arc::new(
                &self.graph.nodes[source as usize],
                &self.graph.nodes[target as usize],
            )
            .central_angle(),
        )
        .round() as u32
    }
}
