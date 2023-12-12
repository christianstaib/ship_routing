use crate::routing::{
    route::{RouteRequest, RouteResponse, Routing},
    Graph,
};

use super::{a_star::AStar, heuristics::distance::Distance};

pub struct ASTarWithDistance<'a> {
    a_star: AStar<'a>,
}

impl<'a> Routing for ASTarWithDistance<'a> {
    fn get_route(&self, request: &RouteRequest) -> RouteResponse {
        let heuristic = Box::new(Distance::new(self.a_star.graph, request.target));
        self.a_star.get_data(request, heuristic)
    }
}

impl<'a> ASTarWithDistance<'a> {
    pub fn new(graph: &'a Graph) -> ASTarWithDistance {
        let a_star = AStar::new(graph);
        ASTarWithDistance { a_star }
    }
}
