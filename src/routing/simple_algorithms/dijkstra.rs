use crate::routing::{
    queue::BucketQueue,
    route::{Route, RouteRequest, Routing},
    Graph,
};

#[derive(Clone)]
pub struct Dijkstra<'a> {
    graph: &'a Graph,
    max_edge_cost: u32,
}

impl<'a> Routing for Dijkstra<'a> {
    fn get_route(&self, route_request: &RouteRequest) -> Option<Route> {
        self.dijkstra(route_request)
    }
}

#[derive(Clone)]
struct DijsktraEntry {
    pub predecessor: u32,
    pub cost: u32,
}

impl DijsktraEntry {
    fn new() -> DijsktraEntry {
        DijsktraEntry {
            predecessor: u32::MAX,
            cost: u32::MAX,
        }
    }

    fn from(predecessor: u32, cost: u32) -> DijsktraEntry {
        DijsktraEntry { predecessor, cost }
    }
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Dijkstra {
        let max_edge_cost = graph.edges.iter().map(|edge| edge.cost).max().unwrap_or(0);
        Dijkstra {
            graph,
            max_edge_cost,
        }
    }

    fn dijkstra(&self, route_request: &RouteRequest) -> Option<Route> {
        let mut queue = BucketQueue::new(self.max_edge_cost + 1);

        let mut nodes = vec![DijsktraEntry::new(); self.graph.nodes.len()];
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];

        nodes[route_request.source as usize].cost = 0;
        queue.insert(0, route_request.source);

        while let Some(source) = queue.pop() {
            if is_expanded[source as usize] {
                continue;
            }
            is_expanded[source as usize] = true;
            if source == route_request.target as u32 {
                break;
            }

            (self.graph.edges_start_at[source as usize]
                ..self.graph.edges_start_at[source as usize + 1])
                .for_each(|edge_id| {
                    let edge = &self.graph.edges[edge_id as usize];
                    let alternative_cost = nodes[source as usize].cost + edge.cost;
                    if alternative_cost < nodes[edge.target as usize].cost {
                        nodes[edge.target as usize] = DijsktraEntry::from(source, alternative_cost);
                        queue.insert(alternative_cost, edge.target);
                    }
                });
        }

        if nodes[route_request.target as usize].cost != u32::MAX {
            let mut route = vec![route_request.target];
            let mut current = route_request.target;
            while nodes[current as usize].predecessor != u32::MAX {
                current = nodes[current as usize].predecessor;
                route.push(current);
            }
            route.reverse();
            return Some(Route {
                cost: nodes[route_request.target as usize].cost,
                nodes: route,
            });
        }
        None
    }
}
