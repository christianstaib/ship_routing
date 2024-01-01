use indicatif::{ProgressBar, ProgressStyle};
use serde_derive::{Deserialize, Serialize};
use speedy::{Readable, Writable};

use crate::routing::graph::{Edge, Graph};

use super::{ch_queue::queue::CHQueue, contraction_helper::ContractionHelper};

#[derive(Clone, Serialize, Deserialize, Readable, Writable)]
pub struct ContractedGraph {
    pub graph: Graph,
    pub map: Vec<((u32, u32), Vec<(u32, u32)>)>,
}

pub struct Contractor {
    graph: Graph,
    queue: CHQueue,
    levels: Vec<u32>,
}

impl Contractor {
    pub fn new(graph: &Graph) -> Self {
        let levels = vec![0; graph.forward_edges.len()];
        let graph = graph.clone();
        let queue = CHQueue::new(&graph);

        Contractor {
            graph,
            queue,
            levels,
        }
    }

    pub fn get_graph_2(graph: &Graph) -> ContractedGraph {
        let mut contractor = Contractor::new(graph);
        contractor.get_graph()
    }

    pub fn get_graph(&mut self) -> ContractedGraph {
        let shortcuts = self.contract();

        let map = shortcuts
            .into_iter()
            .map(|(shortcut, edges)| {
                (
                    (shortcut.source, shortcut.target),
                    edges
                        .iter()
                        .map(|edge| (edge.source, edge.target))
                        .collect(),
                )
            })
            .collect();

        ContractedGraph {
            graph: self.graph.clone(),
            map,
        }
    }

    pub fn contract(&mut self) -> Vec<(Edge, Vec<Edge>)> {
        let outgoing_edges = self.graph.forward_edges.clone();
        let incoming_edges = self.graph.backward_edges.clone();

        let mut shortcuts = Vec::new();

        let bar = ProgressBar::new(self.graph.forward_edges.len() as u64);
        let style =
            ProgressStyle::with_template("{wide_bar} {human_pos}/{human_len} {eta_precise}")
                .unwrap();
        bar.set_style(style);
        let mut level = 0;
        while let Some(v) = self.queue.lazy_pop(&self.graph) {
            shortcuts.append(&mut self.contract_node(v));
            self.levels[v as usize] = level;

            level += 1;
            bar.inc(1);
        }
        bar.finish();

        self.graph.forward_edges = outgoing_edges;
        self.graph.backward_edges = incoming_edges;
        for (shortcut, _) in &shortcuts {
            self.graph.forward_edges[shortcut.source as usize].push(shortcut.clone());
            self.graph.backward_edges[shortcut.target as usize].push(shortcut.clone());
        }

        self.removing_level_property();

        shortcuts
    }

    fn contract_node(&mut self, v: u32) -> Vec<(Edge, Vec<Edge>)> {
        // U --> v --> W
        let shortcut_generator = ContractionHelper::new(&self.graph);
        let shortcuts = shortcut_generator.generate_shortcuts(v, 10);
        self.add_shortcuts(&shortcuts);
        self.disconnect(v);
        shortcuts
    }

    fn add_shortcuts(&mut self, shortcuts: &Vec<(Edge, Vec<Edge>)>) {
        for (shortcut, _) in shortcuts {
            self.graph.forward_edges[shortcut.source as usize].push(shortcut.clone());
            self.graph.backward_edges[shortcut.target as usize].push(shortcut.clone());
        }
    }

    pub fn removing_level_property(&mut self) {
        println!("removing edges that violated level property");
        self.graph.forward_edges.iter_mut().for_each(|edges| {
            edges.retain(|edge| {
                self.levels[edge.source as usize] < self.levels[edge.target as usize]
            });
        });

        self.graph.backward_edges.iter_mut().for_each(|edges| {
            edges.retain(|edge| {
                self.levels[edge.source as usize] > self.levels[edge.target as usize]
            });
        });
    }

    pub fn disconnect(&mut self, node_id: u32) {
        while let Some(incoming_edge) = self.graph.backward_edges[node_id as usize].pop() {
            self.graph.forward_edges[incoming_edge.source as usize]
                .retain(|outgoing_edge| outgoing_edge.target != node_id);
        }

        while let Some(outgoing_edge) = self.graph.forward_edges[node_id as usize].pop() {
            self.graph.backward_edges[outgoing_edge.target as usize]
                .retain(|incoming_edge| incoming_edge.source != node_id);
        }
    }
}
