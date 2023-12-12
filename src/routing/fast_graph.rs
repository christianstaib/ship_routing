use crate::sphere::geometry::point::Point;

use super::{graph::Edge, naive_graph::NaiveGraph};

#[derive(Clone)]
pub struct FastEdge {
    pub target: u32,
    pub cost: u32,
}

#[derive(Clone)]
pub struct Graph {
    pub nodes: Vec<Point>,
    pub forward_edges: FastEdgeAccess,
    pub backward_edges: FastEdgeAccess,
}

#[derive(Clone)]
pub struct FastEdgeAccess {
    pub edges: Vec<FastEdge>,
    pub edges_start_at: Vec<u32>,
}

impl FastEdgeAccess {
    pub fn new(edges: &Vec<Edge>) -> FastEdgeAccess {
        let mut edges = edges.clone();

        let mut edges_start_at: Vec<u32> = vec![0; edges.len() + 1];

        // temporarrly adding a node in order to generate the list
        edges.push(Edge {
            source: edges.len() as u32,
            target: 0,
            cost: 0,
        });
        edges.sort_unstable_by_key(|edge| edge.source);

        let mut current = 0;
        for (i, edge) in edges.iter().enumerate() {
            if edge.source != current {
                for index in (current + 1)..=edge.source {
                    edges_start_at[index as usize] = i as u32;
                }
                current = edge.source;
            }
        }
        edges.pop();
        let edges: Vec<_> = edges.iter().map(|edge| edge.make_fast()).collect();
        let edges_start_at = edges_start_at.clone();

        FastEdgeAccess {
            edges,
            edges_start_at,
        }
    }

    pub fn outgoing_edges(&self, source: u32) -> &[FastEdge] {
        let start = self.edges_start_at[source as usize] as usize;
        let end = self.edges_start_at[source as usize + 1] as usize;

        &self.edges[start..end]
    }
}

impl Graph {
    pub fn outgoing_edges(&self, source: u32) -> &[FastEdge] {
        self.forward_edges.outgoing_edges(source)
    }

    pub fn incoming_edges(&self, target: u32) -> &[FastEdge] {
        self.backward_edges.outgoing_edges(target)
    }

    pub fn new(graph: NaiveGraph) -> Graph {
        let mut graph = graph.clone();
        graph.make_bidirectional();

        let forward_edges = FastEdgeAccess::new(&graph.edges);

        let inverted_edges = graph.edges.iter().map(|edge| edge.get_inverted()).collect();
        let backward_edges = FastEdgeAccess::new(&inverted_edges);

        Graph {
            nodes: graph.nodes.clone(),
            forward_edges,
            backward_edges,
        }
    }
}
