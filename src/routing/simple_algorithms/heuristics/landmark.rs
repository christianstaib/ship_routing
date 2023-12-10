use indicatif::ProgressIterator;
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::routing::{route::RouteRequest, simple_algorithms::dijkstra, Graph};

use super::Heuristic;

#[derive(Clone)]
pub struct LandmarkCollection {
    landmarks: Vec<Landmark>,
}

impl LandmarkCollection {
    pub fn new(graph: &Graph, num_landmarks: u32) -> LandmarkCollection {
        println!("creating landmarks");
        let landmarks = (0..num_landmarks)
            .progress()
            .par_bridge()
            .map(|_| {
                let mut rng = rand::thread_rng();
                Landmark::new(rng.gen_range(0..graph.nodes.len() as u32), graph)
            })
            .collect();
        LandmarkCollection { landmarks }
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

        LandmarkCollection { landmarks }
    }
}

impl Heuristic for LandmarkCollection {
    fn lower_bound(&self, source: u32, target: u32) -> u32 {
        self.landmarks
            .iter()
            .map(|landmark| landmark.lower_bound(source, target))
            .max()
            .unwrap_or(0)
    }
}

#[derive(Clone)]
pub struct Landmark {
    pub source: u32,
    pub costs: Vec<u32>,
}

impl Landmark {
    pub fn new(node: u32, graph: &Graph) -> Landmark {
        let dijkstra = dijkstra::Dijkstra::new(graph);
        let request = RouteRequest {
            source: node,
            target: u32::MAX,
        };
        let data = dijkstra.single_source(&request);
        let costs = (0..graph.nodes.len())
            .map(|node| data.nodes[node].cost)
            .collect();
        Landmark {
            source: node,
            costs,
        }
    }

    pub fn upper_bound(&self, source: u32, target: u32) -> u32 {
        self.costs[source as usize]
            .checked_add(self.costs[target as usize])
            .unwrap_or(u32::MAX)
    }
}

impl Heuristic for Landmark {
    fn lower_bound(&self, source: u32, target: u32) -> u32 {
        if self.costs[source as usize] != u32::MAX && self.costs[target as usize] != u32::MAX {
            return self.costs[source as usize].abs_diff(self.costs[target as usize]);
        }
        println!("err");
        0
    }
}
