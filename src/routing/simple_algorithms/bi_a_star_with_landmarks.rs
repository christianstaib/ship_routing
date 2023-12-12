use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, RouteResponse, Routing},
    Graph,
};

use super::{bi_a_star::BiAStar, heuristics::landmark::LandmarkCollection};

#[derive(Clone)]
pub struct BiAStarWithLandmarks<'a> {
    bi_a_star: BiAStar<'a>,
    forward_heuristic: LandmarkCollection,
    backward_heuristic: LandmarkCollection,
}

impl<'a> Routing for BiAStarWithLandmarks<'a> {
    fn get_route(&self, request: &RouteRequest) -> RouteResponse {
        self.bi_a_star.get_data(
            request,
            Box::new(self.forward_heuristic.tune(request, 3)),
            Box::new(self.backward_heuristic.tune(&request.reversed(), 3)),
        )
    }
}

impl<'a> BiAStarWithLandmarks<'a> {
    pub fn new(graph: &'a Graph) -> BiAStarWithLandmarks<'a> {
        let bi_a_star = BiAStar::new(graph);
        BiAStarWithLandmarks {
            bi_a_star,
            forward_heuristic: LandmarkCollection::new(graph, 100),
            backward_heuristic: LandmarkCollection::new(graph, 100),
        }
    }
}
