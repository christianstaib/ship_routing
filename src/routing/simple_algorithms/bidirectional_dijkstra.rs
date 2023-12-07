use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
};

#[derive(Clone)]
pub struct Dijkstra<'a> {
    graph: &'a Graph,
    max_edge_cost: u32,
}

impl<'a> Routing for Dijkstra<'a> {
    fn get_route(&self, route_request: &RouteRequest) -> Option<Route> {
        self.dijkstra(route_request)
    }
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Dijkstra {
        let max_edge_cost = graph
            .forward_edges
            .edges
            .iter()
            .map(|edge| edge.cost)
            .max()
            .unwrap_or(0);
        Dijkstra {
            graph,
            max_edge_cost,
        }
    }

    fn dijkstra(&self, request: &RouteRequest) -> Option<Route> {
        let mut forward_data =
            DijkstraData::new(self.max_edge_cost, self.graph.nodes.len(), request.source);
        let mut backward_data =
            DijkstraData::new(self.max_edge_cost, self.graph.nodes.len(), request.target);

        let contact_node;
        loop {
            let forward_source = forward_data.pop()?;
            if backward_data.nodes[forward_source as usize].is_expanded {
                contact_node = forward_source;
                break;
            }
            self.graph
                .outgoing_edges(forward_source)
                .iter()
                .for_each(|edge| forward_data.update(forward_source, edge));

            let backward_source = backward_data.pop()?;
            if forward_data.nodes[backward_source as usize].is_expanded {
                contact_node = backward_source;
                break;
            }
            self.graph
                .incoming_edges(backward_source)
                .iter()
                .for_each(|edge| backward_data.update(backward_source, edge));
        }
        println!("contact node is {}", contact_node);

        let mut forward_route = forward_data.get_route(contact_node)?;
        let mut backward_route = backward_data.get_route(contact_node)?;
        backward_route.nodes.pop();
        backward_route.nodes.reverse();
        // assert_eq!(forward_route.nodes.last(), backward_route.nodes.first());
        forward_route.nodes.extend(backward_route.nodes);
        forward_route.cost += backward_route.cost;

        Some(forward_route)
    }
}
