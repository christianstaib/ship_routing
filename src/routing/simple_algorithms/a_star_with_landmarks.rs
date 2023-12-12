use crate::routing::{
    fast_graph::Graph,
    route::{RouteRequest, RouteResponse, Routing},
};

use super::{a_star::AStar, heuristics::landmark::LandmarkCollection};

pub struct AStarWithLandmarks<'a> {
    a_star: AStar<'a>,
    heuristic: LandmarkCollection,
}

impl<'a> Routing for AStarWithLandmarks<'a> {
    fn get_route(&self, request: &RouteRequest) -> RouteResponse {
        let heuristic = Box::new(self.heuristic.tune(request, 3));
        self.a_star.get_data(request, heuristic)
    }
}

impl<'a> AStarWithLandmarks<'a> {
    pub fn new(graph: &'a Graph) -> AStarWithLandmarks {
        let a_star = AStar::new(graph);
        let heuristic = LandmarkCollection::new(graph, 500);
        AStarWithLandmarks { a_star, heuristic }
    }
}
