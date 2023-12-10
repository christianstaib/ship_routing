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
    fn get_route(&self, route_request: &RouteRequest) -> (Option<Route>, Vec<DijkstraData>) {
        let data = self.get_data(route_request);
        let route = data.get_route(route_request.target);
        (route, vec![data])
    }
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Dijkstra {
        Dijkstra { graph }
    }

    pub fn get_data(&self, request: &RouteRequest) -> DijkstraData {
        let mut data = DijkstraData::new(self.graph.nodes.len(), request.source);

        while let Some(state) = data.pop() {
            if state.value == request.target {
                break;
            }

            self.graph
                .outgoing_edges(state.value)
                .iter()
                .for_each(|edge| data.update(state.value, edge, 0));
        }

        data
    }
}
