use std::collections::{BinaryHeap, HashMap};

use crate::routing::graph::Graph;

use super::binary_heap::MinimumItem;

pub struct DijkstraHelper<'a> {
    graph: &'a Graph,
}

impl<'a> DijkstraHelper<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }

    /// Performs a forward search from `source`.
    ///
    /// The search will not scan nodes
    /// * with id == `without`
    /// * with cost > `max_cost`
    /// * with hops > `max_hops`
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
