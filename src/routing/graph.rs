use crate::sphere::geometry::point::Point;

use super::{fast_graph::FastEdge, naive_graph::NaiveGraph};

#[derive(Clone, PartialEq, Eq)]
pub struct Edge {
    pub source: u32,
    pub target: u32,
    pub cost: u32,
}

impl Edge {
    pub fn new(source: u32, target: u32, cost: u32) -> Edge {
        Edge {
            source,
            target,
            cost,
        }
    }

    pub fn get_inverted(&self) -> Edge {
        Edge {
            source: self.target,
            target: self.source,
            cost: self.cost,
        }
    }
}

impl Edge {
    pub fn make_fast(&self) -> FastEdge {
        FastEdge {
            target: self.target,
            cost: self.cost,
        }
    }
}

#[derive(Clone)]
pub struct Graph {
    pub nodes: Vec<Point>,
    pub forward_edges: Vec<Vec<Edge>>,
    pub backward_edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn from_naive_graph(graph: &NaiveGraph) -> Graph {
        let mut forward_edges = vec![Vec::new(); graph.nodes.len()];
        let mut backward_edges = vec![Vec::new(); graph.nodes.len()];
        graph.edges.iter().for_each(|edge| {
            forward_edges[edge.source as usize].push(edge.clone());
            backward_edges[edge.target as usize].push(edge.clone());
        });

        Graph {
            nodes: graph.nodes.clone(),
            forward_edges,
            backward_edges,
        }
    }
}
