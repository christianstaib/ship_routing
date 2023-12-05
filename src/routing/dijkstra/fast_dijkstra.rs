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

        let mut predecessor = vec![u32::MAX; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];
        predecessor.shrink_to_fit();
        node_cost.shrink_to_fit();
        // let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];
        // is_expanded.shrink_to_fit();

        node_cost[route_request.source as usize] = 0;
        queue.insert(0, route_request.source);

        while let Some(node_id) = queue.pop() {
            // if is_expanded[node_id as usize] {
            //     continue;
            // }
            // is_expanded[node_id as usize] = true;
            if node_id == route_request.target as u32 {
                break;
            }

            (self.graph.edges_start_at[node_id as usize]
                ..self.graph.edges_start_at[node_id as usize + 1])
                .for_each(|edge_id| {
                    let edge = &self.graph.edges[edge_id as usize];
                    let alternative_cost = node_cost[node_id as usize] + edge.cost;
                    if alternative_cost < node_cost[edge.target as usize] {
                        predecessor[edge.target as usize] = node_id;
                        node_cost[edge.target as usize] = alternative_cost;
                        queue.insert(alternative_cost, edge.target);
                    }
                });
        }

        if node_cost[route_request.target as usize] != (u32::MAX) {
            let mut route = vec![route_request.target];
            let mut current = route_request.target;
            while predecessor[current as usize] != u32::MAX {
                current = predecessor[current as usize];
                route.push(current);
            }
            route.reverse();
            return Some(Route {
                cost: node_cost[route_request.target as usize],
                nodes: route,
            });
        }
        None
    }
}
