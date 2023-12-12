use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{RouteRequest, RouteResponse},
    Graph,
};

use super::heuristics::Heuristic;

pub struct AStar<'a> {
    pub graph: &'a Graph,
}

impl<'a> AStar<'a> {
    pub fn new(graph: &'a Graph) -> AStar {
        AStar { graph }
    }

    pub fn get_data(&self, request: &RouteRequest, heuristic: Box<dyn Heuristic>) -> RouteResponse {
        let mut data = DijkstraData::new(self.graph.nodes.len(), request.source);

        while let Some(state) = data.pop() {
            if state.value == request.target {
                break;
            }

            self.graph
                .outgoing_edges(state.value)
                .iter()
                .for_each(|edge| {
                    let h = heuristic.lower_bound(edge.target);
                    data.update(state.value, edge, h);
                })
        }

        RouteResponse {
            route: data.get_route(request.target),
            data: vec![data],
        }
    }
}
