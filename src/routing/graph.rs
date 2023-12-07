use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FastEdge {
    pub target: u32,
    pub cost: u32,
}

#[derive(Clone)]
pub struct Node {
    pub id: u32,
    pub longitude: f64,
    pub latitude: f64,
}

impl Edge {
    pub fn make_fast(&self) -> FastEdge {
        FastEdge {
            target: self.target,
            cost: self.cost,
        }
    }
}

impl Node {
    pub fn new(id: u32, longitude: f64, latitude: f64) -> Node {
        Node {
            id,
            longitude,
            latitude,
        }
    }
}

#[derive(Clone)]
pub struct NaiveGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
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
        let edges = edges.iter().map(|edge| edge.make_fast()).collect();
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

impl NaiveGraph {
    pub fn from_file(filename: &str) -> NaiveGraph {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines();
        let number_of_nodes: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let number_of_edges: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let nodes: Vec<_> = lines
            .by_ref()
            .take(number_of_nodes)
            .map(|node_line| {
                let node_line = node_line.unwrap();
                let mut values = node_line.split_whitespace();
                let id: u32 = values.next().unwrap().parse().unwrap();
                let latitude: f64 = values.next().unwrap().parse().unwrap();
                let longitude: f64 = values.next().unwrap().parse().unwrap();
                Node::new(id, latitude, longitude)
            })
            .collect();

        let edges: Vec<_> = lines
            .by_ref()
            .take(number_of_edges)
            .map(|edge_line| {
                let line = edge_line.unwrap();
                let mut values = line.split_whitespace();
                let source: u32 = values.next().unwrap().parse().unwrap();
                let target: u32 = values.next().unwrap().parse().unwrap();
                let cost: u32 = values.next().unwrap().parse().unwrap();
                Edge::new(source, target, cost)
            })
            .collect();

        NaiveGraph { nodes, edges }
    }

    pub fn make_bidirectional(&mut self) {
        let mut edge_map = HashMap::new();
        self.edges.iter().for_each(|edge| {
            let key = (edge.source, edge.target);
            let key = (std::cmp::min(key.0, key.1), std::cmp::max(key.0, key.1));
            if &edge.cost < edge_map.get(&key).unwrap_or(&u32::MAX) {
                edge_map.insert(key, edge.cost);
            }
        });
        self.edges = edge_map
            .iter()
            .map(|(&(source, target), &cost)| Edge::new(source, target, cost))
            .flat_map(|edge| vec![edge.clone(), edge.get_inverted()])
            .collect();
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
