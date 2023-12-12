use std::collections::HashMap;

use crate::routing::{graph::Graph, queue::heap_queue::HeapQueue};

pub struct ChHelper<'a> {
    pub graph: &'a Graph,
}

impl<'a> ChHelper<'a> {
    pub fn new(graph: &'a Graph) -> ChHelper<'a> {
        ChHelper { graph }
    }

    pub fn costs_without(&self, node: u32, max_cost: u32, without: u32) -> HashMap<u32, u32> {
        let mut queue = HeapQueue::new();
        let mut cost = HashMap::new();
        queue.insert(0, node);
        cost.insert(node, 0);

        while let Some(current_state) = queue.pop() {
            let current_node_cost = *cost.get(&current_state.value).unwrap();
            self.graph.forward_edges[current_state.value as usize]
                .iter()
                .for_each(|edge| {
                    if edge.target != without {
                        let alternative_cost = current_node_cost + edge.cost;
                        let current_cost = cost.get(&edge.target).unwrap_or(&u32::MAX);
                        if (&alternative_cost < current_cost) && (alternative_cost <= max_cost) {
                            queue.insert(alternative_cost, edge.target);
                            cost.insert(edge.target, alternative_cost);
                        }
                    }
                });
        }

        cost
    }
}
