use crate::{
    geometry::{radians_to_meter, Arc},
    routing::{
        dijkstra_data::DijkstraData,
        route::{Route, RouteRequest, Routing},
        Graph,
    },
};

#[derive(Clone)]
pub struct Dijkstra<'a> {
    graph: &'a Graph,
}

impl<'a> Routing for Dijkstra<'a> {
    fn get_route(&self, route_request: &RouteRequest) -> Option<Route> {
        self.dijkstra(route_request)
    }
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Dijkstra {
        Dijkstra { graph }
    }

    fn dijkstra(&self, request: &RouteRequest) -> Option<Route> {
        let mut forward = DijkstraData::new(self.graph.nodes.len(), request.source);
        let mut backward = DijkstraData::new(self.graph.nodes.len(), request.target);

        let mut minmal_cost = u32::MAX;

        loop {
            let forward_state = forward.pop()?;
            if backward.nodes[forward_state.value as usize].is_expanded {
                minmal_cost = minmal_cost.min(
                    forward.nodes[forward_state.value as usize].cost
                        + backward.nodes[forward_state.value as usize].cost,
                );
            }
            self.graph
                .outgoing_edges(forward_state.value)
                .iter()
                .for_each(|edge| {
                    let _h = (radians_to_meter(
                        Arc::new(
                            &self.graph.nodes[edge.target as usize],
                            &self.graph.nodes[request.target as usize],
                        )
                        .central_angle(),
                    ))
                    .round() as u32;
                    forward.update_with_h(forward_state.value, edge, 0)
                });

            let backward_state = backward.pop()?;
            if forward.nodes[backward_state.value as usize].is_expanded {
                minmal_cost = minmal_cost.min(
                    forward.nodes[backward_state.value as usize].cost
                        + backward.nodes[backward_state.value as usize].cost,
                );
            }
            self.graph
                .incoming_edges(backward_state.value)
                .iter()
                .for_each(|edge| {
                    let _h = (radians_to_meter(
                        Arc::new(
                            &self.graph.nodes[request.source as usize],
                            &self.graph.nodes[edge.target as usize],
                        )
                        .central_angle(),
                    )
                    .round()) as u32;
                    backward.update_with_h(backward_state.value, edge, 0)
                });

            if forward.nodes[forward_state.value as usize]
                .cost
                .checked_add(backward.nodes[backward_state.value as usize].cost)
                .unwrap_or(0)
                >= minmal_cost
            {
                break;
            }
        }

        let contact_node = forward
            .nodes
            .iter()
            .zip(backward.nodes.iter())
            .enumerate()
            .min_by_key(|(_, (forward, backward))| {
                forward.cost.checked_add(backward.cost).unwrap_or(u32::MAX)
            })
            .unwrap()
            .0 as u32;

        let mut forward_route = forward.get_route(contact_node)?;
        let mut backward_route = backward.get_route(contact_node)?;
        backward_route.nodes.pop();
        backward_route.nodes.reverse();
        forward_route.nodes.extend(backward_route.nodes);
        forward_route.cost += backward_route.cost;

        Some(forward_route)
    }
}
