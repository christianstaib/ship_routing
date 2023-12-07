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
    pub longitude: f32,
    pub latitude: f32,
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
    pub fn new(id: u32, longitude: f32, latitude: f32) -> Node {
        Node {
            id,
            longitude,
            latitude,
        }
    }
}

pub struct NaiveGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<FastEdge>,
    pub edges_start_at: Vec<u32>,
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
                let latitude: f32 = values.next().unwrap().parse().unwrap();
                let longitude: f32 = values.next().unwrap().parse().unwrap();
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
        self.edges
            .iter()
            .flat_map(|edge| vec![edge.clone(), edge.get_inverted()])
            .for_each(|edge| {
                let key = (edge.source, edge.target);
                if &edge.cost < edge_map.get(&key).unwrap_or(&u32::MAX) {
                    edge_map.insert(key, edge.cost);
                }
            });
        self.edges = edge_map
            .iter()
            .map(|(&(source, target), &cost)| Edge::new(source, target, cost))
            .collect();
    }
}

impl Graph {
    pub fn outgoing_edges(&self, source: u32) -> &[FastEdge] {
        let start = self.edges_start_at[source as usize] as usize;
        let end = self.edges_start_at[source as usize + 1] as usize;

        &self.edges[start..end]
    }

    pub fn from_file(filename: &str) -> Graph {
        let mut naive_graph = NaiveGraph::from_file(filename);
        naive_graph.make_bidirectional();

        let mut edges = naive_graph.edges.clone();

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

        Graph {
            nodes: naive_graph.nodes.clone(),
            edges: edges.iter().map(|edge| edge.make_fast()).collect(),
            edges_start_at: edges_start_at.clone(),
        }
    }
}
