use indicatif::ProgressIterator;
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::routing::{fast_graph::FastGraph, route::RouteRequest, simple_algorithms::dijkstra};

use super::Heuristic;

#[derive(Clone)]
pub struct LandmarkCollection {
    target: u32,
    landmarks: Vec<Landmark>,
}

impl LandmarkCollection {
    pub fn new(graph: &FastGraph, num_landmarks: u32) -> LandmarkCollection {
        println!("creating landmarks");
        let landmarks = (0..num_landmarks)
            .progress()
            .par_bridge()
            .map(|_| {
                let mut rng = rand::thread_rng();
                Landmark::new(rng.gen_range(0..graph.nodes.len() as u32), graph)
            })
            .collect();
        LandmarkCollection {
            landmarks,
            target: 0,
        }
    }

    pub fn tune(&self, request: &RouteRequest, num_landmarks: u32) -> LandmarkCollection {
        let mut diff: Vec<_> = self
            .landmarks
            .iter()
            .map(|landmark| -(landmark.lower_bound(request.source, request.target) as isize))
            .enumerate()
            .collect();

        diff.sort_by_key(|&(_, diff)| diff);

        let mut landmarks = Vec::new();
        for (i, _) in diff.iter().take(num_landmarks as usize) {
            landmarks.push(self.landmarks[*i].clone());
        }

        LandmarkCollection {
            landmarks,
            target: request.target,
        }
    }
}

impl Heuristic for LandmarkCollection {
    fn lower_bound(&self, source: u32) -> u32 {
        self.landmarks
            .iter()
            .map(|landmark| landmark.lower_bound(source, self.target))
            .max()
            .unwrap_or(0)
    }
}

#[derive(Clone)]
pub struct Landmark {
    pub reference: u32,
    pub costs_to: Vec<u32>,
    pub costs_from: Vec<u32>,
}

impl Landmark {
    pub fn new(reference: u32, graph: &FastGraph) -> Landmark {
        let dijkstra = dijkstra::Dijkstra::new(graph);

        let forward_data = dijkstra.get_forward_data(reference);
        let costs_to = (0..graph.nodes.len())
            .map(|node| forward_data.nodes[node].cost)
            .collect();

        let backward_data = dijkstra.get_backward_data(reference);
        let costs_from = (0..graph.nodes.len())
            .map(|node| backward_data.nodes[node].cost)
            .collect();

        Landmark {
            reference,
            costs_to,
            costs_from,
        }
    }

    pub fn upper_bound(&self, source: u32, target: u32) -> u32 {
        self.costs_from[source as usize] + self.costs_to[target as usize]
    }

    fn lower_bound(&self, source: u32, target: u32) -> u32 {
        std::cmp::max(
            self.costs_to[target as usize].saturating_sub(self.costs_from[source as usize]),
            self.costs_from[source as usize].saturating_sub(self.costs_to[target as usize]),
        )
    }
}
