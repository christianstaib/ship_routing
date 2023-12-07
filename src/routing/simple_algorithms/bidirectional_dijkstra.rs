use crate::routing::{
    queue::BucketQueue,
    route::{Route, RouteRequest, Routing},
    FastEdge, Graph,
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
    pub is_expanded: bool,
}

impl DijsktraEntry {
    fn new() -> DijsktraEntry {
        DijsktraEntry {
            predecessor: u32::MAX,
            cost: u32::MAX,
            is_expanded: false,
        }
    }
}

struct DijkstraData {
    pub queue: BucketQueue,
    pub nodes: Vec<DijsktraEntry>,
}

impl DijkstraData {
    pub fn new(max_edge_cost: u32, num_nodes: usize, source: u32) -> DijkstraData {
        let mut queue = BucketQueue::new(max_edge_cost + 1);
        let mut nodes = vec![DijsktraEntry::new(); num_nodes];
        nodes[source as usize].cost = 0;
        queue.insert(0, source);
        DijkstraData { queue, nodes }
    }

    pub fn pop(&mut self) -> Option<u32> {
        while let Some(source) = self.queue.pop() {
            if !self.nodes[source as usize].is_expanded {
                self.nodes[source as usize].is_expanded = true;
                return Some(source);
            }
        }

        None
    }

    pub fn update(&mut self, source: u32, edge: &FastEdge) {
        let alternative_cost = self.nodes[source as usize].cost + edge.cost;
        if alternative_cost < self.nodes[edge.target as usize].cost {
            self.nodes[edge.target as usize].predecessor = source;
            self.nodes[edge.target as usize].cost = alternative_cost;
            self.queue.insert(alternative_cost, edge.target);
        }
    }

    pub fn get_route(&self, target: u32) -> Option<Route> {
        if self.nodes[target as usize].cost != u32::MAX {
            let mut route = vec![target];
            let mut current = target;
            while self.nodes[current as usize].predecessor != u32::MAX {
                current = self.nodes[current as usize].predecessor;
                route.push(current);
            }
            route.reverse();
            return Some(Route {
                cost: self.nodes[target as usize].cost,
                nodes: route,
            });
        }
        None
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

    fn dijkstra(&self, request: &RouteRequest) -> Option<Route> {
        let mut data =
            DijkstraData::new(self.max_edge_cost, self.graph.nodes.len(), request.source);

        while let Some(source) = data.pop() {
            if source == request.target {
                break;
            }

            self.graph
                .outgoing_edges(source)
                .iter()
                .for_each(|edge| data.update(source, edge));
        }

        data.get_route(request.target)
    }
}
