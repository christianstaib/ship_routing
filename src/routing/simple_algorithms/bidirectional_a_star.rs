use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
};

use super::heuristics::Heuristic;

#[derive(Clone)]
pub struct BiAStar<'a> {
    pub graph: &'a Graph,
}

impl<'a> BiAStar<'a> {
    pub fn new(graph: &'a Graph) -> BiAStar {
        BiAStar { graph }
    }

    pub fn get_data(
        &self,
        request: &RouteRequest,
        forward_heuristic: Box<dyn Heuristic>,
        backward_heuristic: Box<dyn Heuristic>,
    ) -> (Option<Route>, Vec<DijkstraData>) {
        let mut forward_data = DijkstraData::new(self.graph.nodes.len(), request.source);
        let mut backward_data = DijkstraData::new(self.graph.nodes.len(), request.target);

        let route = self.get_route(
            request,
            forward_heuristic,
            backward_heuristic,
            &mut forward_data,
            &mut backward_data,
        );

        (route, vec![forward_data, backward_data])
    }

    pub fn get_route(
        &self,
        request: &RouteRequest,
        forward_heuristic: Box<dyn Heuristic>,
        backward_heuristic: Box<dyn Heuristic>,
        forward_data: &mut DijkstraData,
        backward_data: &mut DijkstraData,
    ) -> Option<Route> {
        let mut minimal_cost = u32::MAX;
        let mut minimal_cost_node = u32::MAX;

        let pr_t = backward_heuristic.lower_bound(request.target);
        loop {
            let forward_state = forward_data.pop()?;
            if backward_data.nodes[forward_state.value as usize].is_expanded {
                let contact_cost = forward_data.nodes[forward_state.value as usize].cost
                    + backward_data.nodes[forward_state.value as usize].cost;
                if contact_cost < minimal_cost {
                    minimal_cost = contact_cost;
                    minimal_cost_node = forward_state.value;
                }
            }
            self.graph
                .outgoing_edges(forward_state.value)
                .iter()
                .for_each(|edge| {
                    let h = forward_heuristic.lower_bound(edge.target);
                    forward_data.update(forward_state.value, edge, h)
                });

            let backward_state = backward_data.pop()?;
            if forward_data.nodes[backward_state.value as usize].is_expanded {
                let contact_cost = forward_data.nodes[backward_state.value as usize].cost
                    + backward_data.nodes[backward_state.value as usize].cost;
                if contact_cost < minimal_cost {
                    minimal_cost = contact_cost;
                    minimal_cost_node = backward_state.value;
                }
            }
            self.graph
                .incoming_edges(backward_state.value)
                .iter()
                .for_each(|edge| {
                    let h = backward_heuristic.lower_bound(edge.target);
                    backward_data.update(backward_state.value, edge, h);
                });

            if forward_state.key + backward_state.key
                > minimal_cost.checked_add(pr_t).unwrap_or(u32::MAX)
            {
                break;
            }
        }

        construct_route(minimal_cost_node, forward_data, backward_data)
    }
}

fn construct_route(
    _contact_node: u32,
    forward_data: &DijkstraData,
    backward_data: &DijkstraData,
) -> Option<Route> {
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
