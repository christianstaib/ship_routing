use std::{usize};

use indicatif::ProgressIterator;
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    routing::{
        dijkstra_data::DijkstraData,
        route::{Route, RouteRequest, Routing},
        Graph,
    },
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
        let data = dijkstra.single_source(&request);
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
    fn get_route(&self, request: &RouteRequest) -> Option<Route> {
        let mut forward_data = DijkstraData::new(self.graph.nodes.len(), request.source);
        let mut backward_data = DijkstraData::new(self.graph.nodes.len(), request.target);

        let landmarks = self.tune_landmarks(request, 10);

        let mut minmal_cost = u32::MAX;

        loop {
            let forward_state = forward_data.pop()?;
            if backward_data.nodes[forward_state.value as usize].is_expanded {
                minmal_cost = minmal_cost.min(
                    forward_data.nodes[forward_state.value as usize].cost
                        + backward_data.nodes[forward_state.value as usize].cost,
                );
            }
            self.graph
                .outgoing_edges(forward_state.value)
                .iter()
                .for_each(|edge| {
                    let _h = landmarks
                        .iter()
                        .map(|landmark| landmark.lower_bound(edge.target, request.target))
                        .max()
                        .unwrap();
                    forward_data.update_with_h(forward_state.value, edge, 0)
                });

            let backward_state = backward_data.pop()?;
            if forward_data.nodes[backward_state.value as usize].is_expanded {
                minmal_cost = minmal_cost.min(
                    forward_data.nodes[backward_state.value as usize].cost
                        + backward_data.nodes[backward_state.value as usize].cost,
                );
            }
            self.graph
                .incoming_edges(backward_state.value)
                .iter()
                .for_each(|edge| {
                    let _h = landmarks
                        .iter()
                        .map(|landmark| landmark.lower_bound(edge.target, request.source))
                        .max()
                        .unwrap();
                    backward_data.update_with_h(backward_state.value, edge, 0)
                });

            if forward_data.nodes[forward_state.value as usize].cost
                + backward_data.nodes[backward_state.value as usize].cost
                >= minmal_cost
            {
                break;
            }
        }

        get_route(forward_data, backward_data)
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

    fn tune_landmarks(&self, request: &RouteRequest, n: u32) -> Vec<Landmark> {
        let mut diff: Vec<_> = self
            .landmarks
            .iter()
            .map(|landmark| -(landmark.lower_bound(request.source, request.target) as isize))
            .enumerate()
            .collect();

        diff.sort_by_key(|&(_, diff)| diff);

        let mut landmarks = Vec::new();
        for (i, _) in diff.iter().take(n as usize) {
            landmarks.push(self.landmarks[*i].clone());
        }
        landmarks
    }
}

fn get_route(forward_data: DijkstraData, backward_data: DijkstraData) -> Option<Route> {
    let contact_node = forward_data
        .nodes
        .iter()
        .zip(backward_data.nodes.iter())
        .enumerate()
        .min_by_key(|(_, (forward, backward))| {
            forward.cost.checked_add(backward.cost).unwrap_or(u32::MAX)
        })
        .unwrap()
        .0 as u32;

    let mut forward_route = forward_data.get_route(contact_node)?;
    let mut backward_route = backward_data.get_route(contact_node)?;
    backward_route.nodes.pop();
    backward_route.nodes.reverse();
    forward_route.nodes.extend(backward_route.nodes);
    forward_route.cost += backward_route.cost;

    Some(forward_route)
}
