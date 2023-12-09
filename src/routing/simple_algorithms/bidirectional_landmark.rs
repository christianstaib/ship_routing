use std::usize;

use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
};

use super::heuristics::{landmark::LandmarkCollection, Heuristic};

#[derive(Clone)]
pub struct Dijkstra<'a> {
    graph: &'a Graph,
    heuristic: LandmarkCollection,
}

impl<'a> Routing for Dijkstra<'a> {
    fn get_route(&self, request: &RouteRequest) -> Option<Route> {
        let mut forward_data = DijkstraData::new(self.graph.nodes.len(), request.source);
        let mut backward_data = DijkstraData::new(self.graph.nodes.len(), request.target);

        let landmarks = self.heuristic.tune(request, 10);

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
                    let _h = landmarks.lower_bound(edge.target, request.target);
                    forward_data.update_with_h(forward_state.value, edge, _h)
                });

            let backward_state = backward_data.pop()?;
            if forward_data.nodes[backward_state.value as usize].is_expanded {
                minmal_cost = minmal_cost.min(
                    forward_data.nodes[forward_state.value as usize]
                        .cost
                        .checked_add(backward_data.nodes[backward_state.value as usize].cost)
                        .unwrap(),
                );
            }
            self.graph
                .incoming_edges(backward_state.value)
                .iter()
                .for_each(|edge| {
                    let _h = landmarks.lower_bound(edge.target, request.source);
                    backward_data.update_with_h(backward_state.value, edge, 0)
                });

            if forward_data.nodes[forward_state.value as usize]
                .cost
                .checked_add(backward_data.nodes[backward_state.value as usize].cost)
                .unwrap()
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
        let heuristic = LandmarkCollection::new(graph, 50);
        Dijkstra { graph, heuristic }
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
