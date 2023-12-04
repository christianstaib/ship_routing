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
}
