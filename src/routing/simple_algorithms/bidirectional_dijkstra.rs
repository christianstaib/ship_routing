use crate::routing::{
    route::{Route, RouteRequest, Routing},
    Graph,
};

use super::{bidirectional_a_star::BiAStar, heuristics::zero::Zero};

#[derive(Clone)]
pub struct BiDijkstra<'a> {
    bi_a_star: BiAStar<'a>,
}

impl<'a> Routing for BiDijkstra<'a> {
    fn get_route(&self, request: &RouteRequest) -> Option<Route> {
        self.bi_a_star.get_route(request, Box::new(Zero {}))
    }
}

impl<'a> BiDijkstra<'a> {
    pub fn new(graph: &'a Graph) -> BiDijkstra<'a> {
        let bi_a_star = BiAStar::new(graph);
        BiDijkstra { bi_a_star }
    }
}
