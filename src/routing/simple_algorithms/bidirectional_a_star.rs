use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest},
    Graph,
};

use super::heuristics::{self, Heuristic};

#[derive(Clone)]
pub struct BiAStar<'a> {
    graph: &'a Graph,
}

impl<'a> BiAStar<'a> {
    pub fn new(graph: &'a Graph) -> BiAStar {
        BiAStar { graph }
    }

    pub fn get_route(
        &self,
        request: &RouteRequest,
        heuristic: Box<dyn Heuristic>,
    ) -> Option<Route> {
        let mut forward_data = DijkstraData::new(self.graph.nodes.len(), request.source);
        let mut backward_data = DijkstraData::new(self.graph.nodes.len(), request.target);

        let mut minimal_cost = u32::MAX;
        let mut minimal_cost_node = u32::MAX;

        loop {
            let forward_state = forward_data.pop()?;
            if backward_data.nodes[forward_state.value as usize].is_expanded {
                let contact_cost = forward_data.nodes[forward_state.value as usize]
                    .cost
                    .checked_add(backward_data.nodes[forward_state.value as usize].cost)
                    .unwrap();
                if contact_cost < minimal_cost {
                    minimal_cost = contact_cost;
                    minimal_cost_node = forward_state.value;
                }
            }
            self.graph
                .outgoing_edges(forward_state.value)
                .iter()
                .for_each(|edge| {
                    let _h = (heuristic.lower_bound(edge.target, request.target)
                        - heuristic.lower_bound(request.source, edge.target))
                        / 2
                        + heuristic.lower_bound(request.source, request.target) / 2;
                    forward_data.update_with_h(forward_state.value, edge, _h)
                });

            let backward_state = backward_data.pop()?;
            if forward_data.nodes[backward_state.value as usize].is_expanded {
                let contact_cost = forward_data.nodes[backward_state.value as usize]
                    .cost
                    .checked_add(backward_data.nodes[backward_state.value as usize].cost)
                    .unwrap();
                if contact_cost < minimal_cost {
                    minimal_cost = contact_cost;
                    minimal_cost_node = backward_state.value;
                }
            }
            self.graph
                .incoming_edges(backward_state.value)
                .iter()
                .for_each(|edge| {
                    let _h = (heuristic.lower_bound(request.source, edge.target)
                        - heuristic.lower_bound(edge.target, request.target))
                        / 2
                        + heuristic.lower_bound(request.target, request.source) / 2;
                    backward_data.update_with_h(backward_state.value, edge, _h);
                });

            let x = 0;
            let pr_x = (heuristic.lower_bound(request.source, x)
                - heuristic.lower_bound(x, request.target))
                / 2
                + heuristic.lower_bound(request.target, request.source) / 2;
            let pf_x = (heuristic.lower_bound(x, request.target)
                - heuristic.lower_bound(request.source, x))
                / 2
                + heuristic.lower_bound(request.source, request.target) / 2;

            let x = request.target;
            let pr_t = (heuristic.lower_bound(request.source, x)
                - heuristic.lower_bound(x, request.target))
                / 2
                + heuristic.lower_bound(request.target, request.source) / 2;
            if forward_state.key.checked_add(backward_state.key).unwrap()
                > minimal_cost.checked_add(pr_t).unwrap_or(u32::MAX)
            {
                break;
            }
        }

        construct_route(minimal_cost_node, forward_data, backward_data)
    }
}

fn construct_route(
    contact_node: u32,
    forward_data: DijkstraData,
    backward_data: DijkstraData,
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
