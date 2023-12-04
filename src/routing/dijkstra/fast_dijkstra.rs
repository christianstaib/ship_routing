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

        let mut predecessor = vec![None; self.graph.nodes.len()];
        let mut node_cost: Vec<Option<u32>> = vec![None; self.graph.nodes.len()];
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];

        node_cost[route_request.source as usize] = Some(0);
        queue.insert(0, route_request.source);

        while let Some(node_id) = queue.pop() {
            if is_expanded[node_id as usize] {
                continue;
            }
            if node_id == route_request.target as u32 {
                break;
            }
            is_expanded[node_id as usize] = true;

            (self.graph.edges_start_at[node_id as usize]
                ..self.graph.edges_start_at[node_id as usize + 1])
                .for_each(|edge_id| {
                    let edge = &self.graph.edges[edge_id as usize];
                    let alternative_cost = node_cost[node_id as usize].unwrap() + edge.cost;
                    if alternative_cost < node_cost[edge.target_id as usize].unwrap_or(u32::MAX) {
                        predecessor[edge.target_id as usize] = Some(edge.source_id);
                        node_cost[edge.target_id as usize] = Some(alternative_cost);
                        queue.insert(alternative_cost, edge.target_id);
                    }
                });
        }

        match node_cost[route_request.target as usize] {
            Some(cost) => {
                let mut nodes = vec![route_request.target];
                let mut current = route_request.target;
                while let Some(new_current) = predecessor[current as usize] {
                    current = new_current;
                    nodes.push(current);
                }
                nodes.reverse();
                Some(Route { cost, nodes })
            }
            _ => None,
        }
    }
}
