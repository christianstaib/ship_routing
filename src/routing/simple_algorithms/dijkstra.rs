use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
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
        let mut data = DijkstraData::new(self.graph.nodes.len(), request.source);

        while let Some(state) = data.pop() {
            if state.value == request.target {
                break;
            }

            self.graph
                .outgoing_edges(state.value)
                .iter()
                .for_each(|edge| data.update(state.value, edge));
        }

        data.get_route(request.target)
    }

    pub fn single_source(&self, request: &RouteRequest) -> DijkstraData {
        let mut data = DijkstraData::new(self.graph.nodes.len(), request.source);

        while let Some(state) = data.pop() {
            self.graph
                .outgoing_edges(state.value)
                .iter()
                .for_each(|edge| data.update(state.value, edge));
        }

        data
    }
}
