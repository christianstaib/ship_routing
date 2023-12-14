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

    pub fn single_source_cost_without(
        &self,
        source: u32,
        without: u32,
        max_cost: u32,
    ) -> HashMap<u32, u32> {
        // get costs for routes from v to a set of nodes W defined as u -> v -> W where the routes
        // are not going through v.

        let graph = self.graph;

        let mut queue = BinaryHeap::new();
        // I use a HashMap as only a small number of nodes compared to the whole graph are relaxed.
        // Therefore the overhead of initatlizing a vector is not worth it.
        let mut cost = HashMap::new();
        queue.push(MinimumItem {
            priority: 0,
            item: source,
        });
        cost.insert(source, 0);
        while let Some(state) = queue.pop() {
            let current_node = state.item;
            for edge in &graph.forward_edges[current_node as usize] {
                let alternative_cost = cost[&current_node] + edge.cost;
                if (edge.target != without) & (alternative_cost <= max_cost) {
                    let current_cost = *cost.get(&edge.target).unwrap_or(&u32::MAX);
                    if alternative_cost < current_cost {
                        cost.insert(edge.target, alternative_cost);
                        queue.push(MinimumItem {
                            priority: alternative_cost,
                            item: edge.target,
                        });
                    }
                }
            }
        }

        cost
    }
}
