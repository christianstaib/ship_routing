use std::collections::{BinaryHeap, HashMap};

use crate::routing::graph::Graph;

use super::binary_heap::MinimumItem;

pub struct WitnessDijkstra<'a> {
    graph: &'a Graph,
}

impl<'a> WitnessDijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }

    /// Performs a forward search from `source` node.
    ///
    /// Returns a `HashMap` where each entry consists of a node identifier (u32) and the associated cost (u32) to reach that node from the `source`.
    ///
    /// Parameters:
    /// - `source`: The starting node for the search.
    /// - `without`: A node identifier to be excluded from the search. The search will ignore paths through this node.
    /// - `max_cost`: The maximum allowable cost. Nodes that can only be reached with a cost higher than this value will not be included in the results.
    /// - `max_hops`: The maximum number of hops (edges traversed) allowed. Nodes that require more hops to reach than this limit will not be included in the results.
    ///
    /// Note: The search algorithm takes into account the cost and number of hops to reach each node. Nodes are included in the resulting map only if they meet the specified conditions regarding cost and hop count, and are not the `without` node.
    pub fn witness_search(
        &self,
        source: u32,
        without: u32,
        max_cost: u32,
        max_hops: u32,
    ) -> HashMap<u32, u32> {
        let mut queue = BinaryHeap::new();
        let mut cost = HashMap::new();
        let mut hops = HashMap::new();

        queue.push(MinimumItem {
            cost: 0,
            node: source,
        });
        cost.insert(source, 0);
        hops.insert(source, 0);

        while let Some(MinimumItem { node, .. }) = queue.pop() {
            for edge in &self.graph.forward_edges[node as usize] {
                let alternative_cost = cost[&node] + edge.cost;
                let new_hops = hops[&node] + 1;
                if (edge.target != without)
                    && (alternative_cost <= max_cost)
                    && (new_hops <= max_hops)
                {
                    let current_cost = *cost.get(&edge.target).unwrap_or(&u32::MAX);
                    if alternative_cost < current_cost {
                        queue.push(MinimumItem {
                            cost: alternative_cost,
                            node: edge.target,
                        });
                        cost.insert(edge.target, alternative_cost);
                        hops.insert(edge.target, new_hops);
                    }
                }
            }
        }

        cost
    }
}
