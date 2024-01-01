use crate::routing::graph::Graph;

use super::queue::PriorityTerm;

pub struct VoronoiRegion {}

impl PriorityTerm for VoronoiRegion {
    fn priority(&self, v: u32, graph: &Graph) -> i32 {
        let neighbors = graph.get_neighborhood(v, 1);

        let mut region_size = 0;
        neighbors.iter().for_each(|&neighbor| {
            if let Some(nearest) = graph
                .forward_edges
                .get(neighbor as usize)
                .unwrap()
                .iter()
                .map(|edge| (edge.target, edge.cost))
                .min_by_key(|(_, cost)| *cost)
            {
                if nearest.0 == v {
                    region_size += 1;
                }
            }
        });
        (region_size as f32).sqrt() as i32
    }

    #[allow(unused_variables)]
    fn update_before_contraction(&mut self, v: u32, graph: &Graph) {}
}

impl VoronoiRegion {
    pub fn new() -> Self {
        Self {}
    }
}
