use std::collections::HashMap;

use indicatif::ProgressIterator;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde_derive::{Deserialize, Serialize};

use crate::routing::{route::RouteRequest, simple_algorithms::ch_bi_dijkstra::ChDijkstra};

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LabelEntry {
    pub id: u32,
    pub cost: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    label: Vec<LabelEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct HubGraph {
    forward_labels: Vec<Label>,
    backward_labels: Vec<Label>,
}

impl Label {
    pub fn new(map: &HashMap<u32, u32>) -> Label {
        let mut labels: Vec<_> = map
            .iter()
            .map(|(id, cost)| LabelEntry {
                id: *id,
                cost: *cost,
            })
            .collect();
        labels.sort_unstable();
        labels.shrink_to_fit();

        Label { label: labels }
    }

    pub fn minimal_overlapp(&self, other: &Label) -> Option<LabelEntry> {
        let mut i_self = 0;
        let mut i_other = 0;

        let mut id = u32::MAX;
        let mut cost = u32::MAX;

        while i_self < self.label.len() && i_other < other.label.len() {
            let self_entry = &self.label[i_self];
            let other_entry = &self.label[i_other];

            match self_entry.cmp(other_entry) {
                std::cmp::Ordering::Less => i_self += 1,
                std::cmp::Ordering::Equal => {
                    i_self += 1;
                    i_other += 1;

                    let alternative_cost = self_entry.cost + other_entry.cost;
                    if alternative_cost < cost {
                        id = self_entry.id;
                        cost = alternative_cost;
                    }
                }
                std::cmp::Ordering::Greater => i_other += 1,
            }
        }

        if cost != u32::MAX {
            return Some(LabelEntry { id, cost });
        }

        None
    }
}

impl HubGraph {
    pub fn new(dijkstra: &ChDijkstra, depth_limit: u32) -> HubGraph {
        let mut forward_labels: Vec<_> = (0..dijkstra.graph.num_nodes)
            .progress()
            .par_bridge()
            .map(|id| Label::new(&dijkstra.get_forward_label(id, depth_limit)))
            .collect();
        let mut backward_labels: Vec<_> = (0..dijkstra.graph.num_nodes)
            .progress()
            .par_bridge()
            .map(|id| Label::new(&dijkstra.get_backward_label(id, depth_limit)))
            .collect();
        forward_labels.shrink_to_fit();
        backward_labels.shrink_to_fit();
        HubGraph {
            forward_labels,
            backward_labels,
        }
    }

    pub fn get_route(&self, request: &RouteRequest) -> Option<LabelEntry> {
        let forward_label = self.forward_labels.get(request.source as usize)?;
        let backward_label = self.backward_labels.get(request.target as usize)?;
        forward_label.minimal_overlapp(&backward_label)
    }
}
