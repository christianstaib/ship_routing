use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    geometry::{radians_to_meter, Arc},
    routing::{
        dijkstra_data::DijkstraData,
        route::{Route, RouteRequest, Routing},
        Graph,
    },
};

use super::dijkstra::{self};

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
                .par_bridge()
                .for_each(|edge| {
                    let h = radians_to_meter(
                        Arc::new(
                            &self.graph.nodes[edge.target as usize],
                            &self.graph.nodes[request.target as usize],
                        )
                        .central_angle(),
                    ) as u32;
                    let dijkstra = dijkstra::Dijkstra::new(self.graph);
                    if let Some(route) = dijkstra.get_route(&RouteRequest {
                        source: edge.target,
                        target: request.target,
                    }) {
                        assert!(h <= route.cost);
                    }
                });

            self.graph
                .outgoing_edges(state.value)
                .iter()
                .for_each(|edge| {
                    data.update_with_h(state.value, edge, 0);
                })
        }

        data.get_route(request.target)
    }
}
