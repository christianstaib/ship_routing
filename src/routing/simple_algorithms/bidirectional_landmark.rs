use crate::routing::{
    route::{Route, RouteRequest, Routing},
    Graph,
};

use super::{bidirectional_a_star::BiAStar, heuristics::landmark::LandmarkCollection};

#[derive(Clone)]
pub struct BiLandmark<'a> {
    bi_a_star: BiAStar<'a>,
    heuristic: LandmarkCollection,
}

impl<'a> Routing for BiLandmark<'a> {
    fn get_route(&self, request: &RouteRequest) -> Option<Route> {
        self.bi_a_star
            .get_route(request, Box::new(self.heuristic.tune(request, 10)))
    }
}

impl<'a> BiLandmark<'a> {
    pub fn new(graph: &'a Graph) -> BiLandmark<'a> {
        let heuristic = LandmarkCollection::new(graph, 50);
        let bi_a_star = BiAStar::new(graph);
        BiLandmark {
            bi_a_star,
            heuristic,
        }
    }
}
