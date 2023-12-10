use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
};

use super::{a_star::AStar, heuristics::landmark::LandmarkCollection};

pub struct AStarWithLandmarks<'a> {
    a_star: AStar<'a>,
    heuristic: LandmarkCollection,
}

impl<'a> Routing for AStarWithLandmarks<'a> {
    fn get_route(&self, request: &RouteRequest) -> (Option<Route>, Vec<DijkstraData>) {
        let heuristic = Box::new(self.heuristic.tune(request, 3));
        let data = self.a_star.get_data(request, heuristic);
        let route = data.get_route(request.target);
        (route, vec![data])
    }
}

impl<'a> AStarWithLandmarks<'a> {
    pub fn new(graph: &'a Graph) -> AStarWithLandmarks {
        let a_star = AStar::new(graph);
        let heuristic = LandmarkCollection::new(graph, 100);
        AStarWithLandmarks { a_star, heuristic }
    }
}
