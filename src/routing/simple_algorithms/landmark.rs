use std::usize;

use indicatif::ProgressIterator;
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
};

use super::dijkstra;

#[derive(Clone)]
pub struct Landmark {
    pub node: u32,
    pub costs: Vec<u32>,
}

impl Landmark {
    pub fn new(node: u32, graph: &Graph) -> Landmark {
        let dijkstra = dijkstra::Dijkstra::new(graph);
        let request = RouteRequest {
            source: node,
            target: u32::MAX,
        };
        let data = dijkstra.get_data(&request);
        let costs = (0..graph.nodes.len())
            .map(|node| data.nodes[node].cost)
            .collect();
        Landmark { node, costs }
    }

    pub fn lower_bound(&self, source: u32, target: u32) -> u32 {
        self.costs[source as usize].abs_diff(self.costs[target as usize])
    }

    pub fn upper_bound(&self, source: u32, target: u32) -> u32 {
        self.costs[source as usize]
            .checked_add(self.costs[target as usize])
            .unwrap_or(u32::MAX)
    }
}

#[derive(Clone)]
pub struct Dijkstra<'a> {
    graph: &'a Graph,
    landmarks: Vec<Landmark>,
}

impl<'a> Routing for Dijkstra<'a> {
    fn get_route(&self, route_request: &RouteRequest) -> Option<Route> {
        self.dijkstra(route_request)
    }
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Dijkstra {
        println!("creating landmarks");
        let landmarks = (0..100)
            .progress()
            .par_bridge()
            .map(|_| {
                let mut rng = rand::thread_rng();
                Landmark::new(rng.gen_range(0..graph.nodes.len() as u32), graph)
            })
            .collect();
        Dijkstra { graph, landmarks }
    }

    fn dijkstra(&self, request: &RouteRequest) -> Option<Route> {
        let mut data = DijkstraData::new(self.graph.nodes.len(), request.source);

        let n = 10;
        let mut diff: Vec<_> = self
            .landmarks
            .iter()
            .map(|landmark| -(landmark.lower_bound(request.source, request.target) as isize))
            .enumerate()
            .collect();

        diff.sort_by_key(|&(_, diff)| diff);

        let mut landmarks = Vec::new();
        for (i, _) in diff.iter().take(n) {
            landmarks.push(self.landmarks[*i].clone());
        }

        while let Some(state) = data.pop() {
            if state.value == request.target {
                break;
            }

            self.graph
                .outgoing_edges(state.value)
                .iter()
                .for_each(|edge| {
                    let h = landmarks
                        .iter()
                        .map(|landmark| landmark.lower_bound(edge.target, request.target))
                        .max()
                        .unwrap();
                    data.update_with_h(state.value, edge, h);
                })
        }

        data.get_route(request.target)
    }
}
