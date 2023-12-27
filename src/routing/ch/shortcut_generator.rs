use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::routing::graph::{Edge, Graph};

use super::dijkstra_helper::DijkstraHelper;

pub struct ShortcutGenerator<'a> {
    graph: &'a Graph,
}

impl<'a> ShortcutGenerator<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }

    pub fn naive_shortcuts(&self, v: u32) -> Vec<(Edge, Vec<Edge>)> {
        let dijkstra_helper = DijkstraHelper::new(self.graph);

        let uv_edges = &self.graph.backward_edges[v as usize];
        let vw_edges = &self.graph.forward_edges[v as usize];

        if uv_edges.len() < 40 {
            uv_edges
                .iter()
                .flat_map(|uv_edge| {
                    let mut shortcuts = Vec::new();
                    let u = uv_edge.source;
                    let uv_cost = uv_edge.cost;

                    let max_cost =
                        uv_cost + vw_edges.iter().map(|edge| edge.cost).max().unwrap_or(0);

                    let costs = dijkstra_helper.single_source_cost_without(u, v, max_cost);
                    vw_edges.iter().for_each(|vw_ede| {
                        let w = vw_ede.target;
                        let vw_cost = vw_ede.cost;
                        let cost = uv_cost + vw_cost;
                        if &cost < costs.get(&w).unwrap_or(&u32::MAX) {
                            let shortcut = Edge {
                                source: u,
                                target: w,
                                cost,
                            };
                            shortcuts.push((shortcut, vec![uv_edge.clone(), vw_ede.clone()]));
                        }
                    });
                    shortcuts
                })
                .collect()
        } else {
            uv_edges
                .iter()
                .par_bridge()
                .flat_map(|uv_edge| {
                    let mut shortcuts = Vec::new();
                    let u = uv_edge.source;
                    let uv_cost = uv_edge.cost;

                    let max_cost =
                        uv_cost + vw_edges.iter().map(|edge| edge.cost).max().unwrap_or(0);

                    let costs = dijkstra_helper.single_source_cost_without(u, v, max_cost);
                    vw_edges.iter().for_each(|vw_ede| {
                        let w = vw_ede.target;
                        let vw_cost = vw_ede.cost;
                        let cost = uv_cost + vw_cost;
                        if &cost < costs.get(&w).unwrap_or(&u32::MAX) {
                            let shortcut = Edge {
                                source: u,
                                target: w,
                                cost,
                            };
                            shortcuts.push((shortcut, vec![uv_edge.clone(), vw_ede.clone()]));
                        }
                    });
                    shortcuts
                })
                .collect()
        }
    }
}
