use crate::routing::{
    dijkstra_data::DijkstraData,
    route::{Route, RouteRequest, Routing},
    Graph,
};

use super::{a_star::AStar, heuristics::distance::Distance};

pub struct ASTarWithDistance<'a> {
    a_star: AStar<'a>,
}

impl<'a> Routing for ASTarWithDistance<'a> {
    fn get_route(&self, request: &RouteRequest) -> (Option<Route>, Vec<DijkstraData>) {
        let heuristic = Box::new(Distance::new(self.a_star.graph, request.target));
        let data = self.a_star.get_data(request, heuristic);
        let route = data.get_route(request.target);
        (route, vec![data])
    }
}

impl<'a> ASTarWithDistance<'a> {
    pub fn new(graph: &'a Graph) -> ASTarWithDistance {
        let a_star = AStar::new(graph);
        ASTarWithDistance { a_star }
    }
}
