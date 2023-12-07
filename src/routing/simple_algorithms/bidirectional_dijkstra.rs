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
        let max_edge_cost = graph.edges.iter().map(|edge| edge.cost).max().unwrap_or(0);
        Dijkstra {
            graph,
            max_edge_cost,
        }
    }

    fn dijkstra(&self, request: &RouteRequest) -> Option<Route> {
        let mut data =
            DijkstraData::new(self.max_edge_cost, self.graph.nodes.len(), request.source);

        while let Some(source) = data.pop() {
            if source == request.target {
                break;
            }

            self.graph
                .outgoing_edges(source)
                .iter()
                .for_each(|edge| data.update(source, edge));
        }

        data.get_route(request.target)
    }
}
