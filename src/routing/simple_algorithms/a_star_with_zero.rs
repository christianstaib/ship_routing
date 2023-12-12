use crate::routing::{
    dijkstra_data::DijkstraData,
    fast_graph::FastGraph,
    route::{Route, RouteRequest, RouteResponse, Routing},
};

use super::{a_star::AStar, heuristics::zero::Zero};

pub struct AStarWithZero<'a> {
    a_star: AStar<'a>,
}

impl<'a> Routing for AStarWithZero<'a> {
    fn get_route(&self, request: &RouteRequest) -> RouteResponse {
        let heuristic = Box::new(Zero {});
        self.a_star.get_data(request, heuristic)
    }
}

impl<'a> AStarWithZero<'a> {
    pub fn new(graph: &'a FastGraph) -> AStarWithZero {
        let a_star = AStar::new(graph);
        AStarWithZero { a_star }
    }
}
