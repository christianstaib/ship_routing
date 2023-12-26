use std::collections::HashMap;

use crate::routing::graph::Graph;

pub fn remove_edge_to_self(graph: &mut Graph) {
    for i in 0..graph.backward_edges.len() {
        graph.backward_edges[i].retain(|edge| edge.source != i as u32);
    }

    for i in 0..graph.forward_edges.len() {
        graph.forward_edges[i].retain(|edge| edge.target != i as u32);
    }
}

pub fn removing_double_edges(graph: &mut Graph) {
    for i in 0..graph.backward_edges.len() {
        let mut edge_map = HashMap::new();
        for edge in &graph.backward_edges[i] {
            let edge_tuple = (edge.source, edge.target);
            let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
            if &edge.cost < current_cost {
                edge_map.insert(edge_tuple, edge.cost);
            }
        }
        graph.backward_edges[i]
            .retain(|edge| edge.cost <= *edge_map.get(&(edge.source, edge.target)).unwrap());
    }

    for i in 0..graph.forward_edges.len() {
        let mut edge_map = HashMap::new();
        for edge in &graph.forward_edges[i] {
            let edge_tuple = (edge.source, edge.target);
            let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
            if &edge.cost < current_cost {
                edge_map.insert(edge_tuple, edge.cost);
            }
        }
        graph.forward_edges[i]
            .retain(|edge| edge.cost <= *edge_map.get(&(edge.source, edge.target)).unwrap());
    }
}
