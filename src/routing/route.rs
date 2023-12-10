use super::Graph;

#[derive(Clone)]
pub struct RouteRequest {
    pub source: u32,
    pub target: u32,
}

pub struct Route {
    pub nodes: Vec<u32>,
    pub cost: u32,
}

pub trait Routing {
    fn get_route(&self, route_request: &RouteRequest) -> Option<Route>;
    // fn get_data(&self, route_request: &RouteRequest) -> (Option<Route>, Vec<Vec<u32>>);
}

impl Route {
    pub fn is_valid(&self, graph: &Graph, request: &RouteRequest) -> bool {
        let mut true_cost = 0;
        for (source, target) in self.nodes.windows(2).map(|vec| (vec[0], vec[1])) {
            true_cost += graph
                .outgoing_edges(source)
                .iter()
                .find(|edge| edge.target == target)
                .unwrap()
                .cost;
        }
        (true_cost == self.cost)
            && (self.nodes.first().unwrap() == &request.source)
            && (self.nodes.last().unwrap() == &request.target)
    }
}

impl RouteRequest {
    pub fn reversed(&self) -> RouteRequest {
        RouteRequest {
            source: self.target,
            target: self.source,
        }
    }
}
