use crate::routing::{
    dijkstra_data::DijkstraData,
    fast_graph::FastGraph,
    route::{Route, RouteRequest, RouteResponse, Routing},
};

#[derive(Clone)]
pub struct ChDijkstra<'a> {
    pub graph: &'a FastGraph,
}

impl<'a> Routing for ChDijkstra<'a> {
    fn get_route(&self, request: &RouteRequest) -> RouteResponse {
        self.get_data(request)
    }
}

impl<'a> ChDijkstra<'a> {
    pub fn new(graph: &'a FastGraph) -> ChDijkstra {
        ChDijkstra { graph }
    }

    pub fn get_data(&self, request: &RouteRequest) -> RouteResponse {
        let mut forward_data = DijkstraData::new(self.graph.num_nodes as usize, request.source);
        let mut backward_data = DijkstraData::new(self.graph.num_nodes as usize, request.target);

        let route = self.get_route(request, &mut forward_data, &mut backward_data);

        RouteResponse {
            route,
            data: vec![forward_data, backward_data],
        }
    }

    pub fn get_route(
        &self,
        _request: &RouteRequest,
        forward_data: &mut DijkstraData,
        backward_data: &mut DijkstraData,
    ) -> Option<Route> {
        let mut minimal_cost = u32::MAX;
        let mut meeting_node = u32::MAX;

        loop {
            let forward_state = forward_data.pop();
            if let Some(forward_state) = forward_state {
                let current_node = forward_state.value;

                if backward_data.nodes[forward_state.value as usize].is_expanded {
                    let contact_cost = forward_data.nodes[forward_state.value as usize].cost
                        + backward_data.nodes[forward_state.value as usize].cost;
                    if contact_cost < minimal_cost {
                        minimal_cost = contact_cost;
                        meeting_node = forward_state.value;
                    }
                }

                self.graph
                    .outgoing_edges(forward_state.value)
                    .iter()
                    .for_each(|edge| {
                        let alternative_cost =
                            forward_data.nodes[current_node as usize].cost + edge.cost;
                        let current_cost = forward_data.nodes[edge.target as usize].cost;
                        if alternative_cost < current_cost {
                            forward_data.nodes[edge.target as usize].cost = alternative_cost;
                            forward_data.nodes[edge.target as usize].predecessor = current_node;
                            forward_data.queue.insert(alternative_cost + 0, edge.target);
                        }
                    });
            }

            let backward_state = backward_data.pop();
            if let Some(backward_state) = backward_state {
                if forward_data.nodes[backward_state.value as usize].is_expanded {
                    let contact_cost = forward_data.nodes[backward_state.value as usize].cost
                        + backward_data.nodes[backward_state.value as usize].cost;
                    if contact_cost < minimal_cost {
                        minimal_cost = contact_cost;
                        meeting_node = backward_state.value;
                    }
                }

                self.graph
                    .incoming_edges(backward_state.value)
                    .iter()
                    .for_each(|edge| {
                        {
                            let source = backward_state.value;
                            let alternative_cost =
                                backward_data.nodes[source as usize].cost + edge.cost;
                            let current_cost = backward_data.nodes[edge.target as usize].cost;
                            if alternative_cost < current_cost {
                                backward_data.nodes[edge.target as usize].predecessor = source;
                                backward_data.nodes[edge.target as usize].cost = alternative_cost;
                                backward_data
                                    .queue
                                    .insert(alternative_cost + 0, edge.target);
                            }
                        };
                    });
            }

            if forward_state.is_none() && backward_state.is_none() {
                break;
            }
        }

        construct_route(meeting_node, forward_data, backward_data)
    }
}

fn construct_route(
    contact_node: u32,
    forward_data: &DijkstraData,
    backward_data: &DijkstraData,
) -> Option<Route> {
    let mut forward_route = forward_data.get_route(contact_node)?;
    let mut backward_route = backward_data.get_route(contact_node)?;
    backward_route.nodes.pop();
    backward_route.nodes.reverse();
    forward_route.nodes.extend(backward_route.nodes);
    forward_route.cost += backward_route.cost;

    Some(forward_route)
}
