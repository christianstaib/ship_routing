use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
};

use super::{a_star::AStar, heuristics::zero::Zero};

pub struct AStarWithZero<'a> {
    a_star: AStar<'a>,
}

impl<'a> Routing for AStarWithZero<'a> {
    fn get_route(&self, request: &RouteRequest) -> (Option<Route>, Vec<DijkstraData>) {
        let heuristic = Box::new(Zero {});
        let data = self.a_star.get_data(request, heuristic);
        let route = data.get_route(request.target);
        (route, vec![data])
    }
}

impl<'a> AStarWithZero<'a> {
    pub fn new(graph: &'a Graph) -> AStarWithZero {
        let a_star = AStar::new(graph);
        AStarWithZero { a_star }
    }
}
