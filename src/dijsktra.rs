use crate::bucket_queue::BucketQueue;
use crate::graph::*;

pub fn dijkstra(graph: &Graph, from_node_id: u32, to_node_id: u32) -> (Vec<Option<u32>>, u32) {
    let mut queue = BucketQueue::new(60_000);

    let mut edge_from_predecessor = vec![None; graph.nodes.len()];
    let mut node_cost: Vec<Option<u32>> = vec![None; graph.nodes.len()];
    let mut is_expanded: Vec<bool> = vec![false; graph.nodes.len()];

    node_cost[from_node_id as usize] = Some(0);
    queue.insert(0, from_node_id);

    while let Some(node_id) = queue.pop() {
        if is_expanded[node_id as usize] {
            continue;
        }
        if node_id == to_node_id as u32 {
            break;
        }
        is_expanded[node_id as usize] = true;

        (graph.edges_start_at[node_id as usize]..graph.edges_start_at[node_id as usize + 1])
            .for_each(|edge_id| {
                let edge = &graph.edges[edge_id as usize];
                let alternative_cost = node_cost[node_id as usize].unwrap() + edge.cost;
                if alternative_cost < node_cost[edge.target_id as usize].unwrap_or(u32::MAX) {
                    edge_from_predecessor[edge.target_id as usize] = Some(edge_id);
                    node_cost[edge.target_id as usize] = Some(alternative_cost);
                    queue.insert(alternative_cost, edge.target_id);
                }
            });
    }

    (
        edge_from_predecessor,
        node_cost[to_node_id as usize].unwrap_or(u32::MAX),
    )
}
