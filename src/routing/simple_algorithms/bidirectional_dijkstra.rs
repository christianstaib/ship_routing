use std::time::Instant;

use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
};

#[derive(Clone)]
pub struct Dijkstra<'a> {
    graph: &'a Graph,
    max_edge_diff: u32,
}

impl<'a> Routing for Dijkstra<'a> {
    fn get_route(&self, route_request: &RouteRequest) -> Option<Route> {
        self.dijkstra(route_request)
    }
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Dijkstra {
        Dijkstra {
            graph,
            max_edge_diff: graph
                .forward_edges
                .edges
                .iter()
                .max_by_key(|edge| edge.cost)
                .unwrap()
                .cost,
        }
    }

    fn dijkstra(&self, request: &RouteRequest) -> Option<Route> {
        let mut forward_data = DijkstraData::new(self.graph.nodes.len(), request.source);
        let mut backward_data = DijkstraData::new(self.graph.nodes.len(), request.target);

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
                .for_each(|edge| forward_data.update(forward_state.value, edge));

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
                .for_each(|edge| backward_data.update(backward_state.value, edge));

            if forward_state.key + backward_state.key >= minmal_cost {
                break;
            }
        }

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
}
