use serde_derive::{Deserialize, Serialize};

use super::{dijkstra_data::DijkstraData, fast_graph::FastGraph};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteRequest {
    pub source: u32,
    pub target: u32,
}

#[derive(Clone)]
pub struct Route {
    pub nodes: Vec<u32>,
    pub cost: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteValidationRequest {
    pub request: RouteRequest,
    pub cost: Option<u32>,
}

impl RouteValidationRequest {
    pub fn from_str(str: &str) -> Option<RouteValidationRequest> {
        let line: Vec<_> = str.split(',').collect();
        let mut cost = None;
        if let Ok(str_cost) = line[2].parse::<u32>() {
            cost = Some(str_cost);
        }
        Some(RouteValidationRequest {
            request: RouteRequest {
                source: line[0].parse().ok()?,
                target: line[1].parse().ok()?,
            },
            cost,
        })
    }
}

#[derive(Clone)]
pub struct RouteResponse {
    pub route: Option<Route>,
    pub data: Vec<DijkstraData>,
}

pub trait Routing {
    fn get_route(&self, route_request: &RouteRequest) -> RouteResponse;
}

impl Route {
    pub fn is_valid(&self, graph: &FastGraph, request: &RouteRequest) -> bool {
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
