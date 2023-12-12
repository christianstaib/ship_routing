use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, RouteResponse, Routing},
    Graph,
};

use super::{bi_a_star::BiAStar, heuristics::zero::Zero};

#[derive(Clone)]
pub struct BiAStarWithZero<'a> {
    bi_a_star: BiAStar<'a>,
}

impl<'a> Routing for BiAStarWithZero<'a> {
    fn get_route(&self, request: &RouteRequest) -> RouteResponse {
        self.bi_a_star
            .get_data(request, Box::new(Zero {}), Box::new(Zero {}))
    }
}

impl<'a> BiAStarWithZero<'a> {
    pub fn new(graph: &'a Graph) -> BiAStarWithZero<'a> {
        let bi_a_star = BiAStar::new(graph);
        BiAStarWithZero { bi_a_star }
    }
}
