use ahash::HashMap;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde_derive::{Deserialize, Serialize};
use speedy::{Readable, Writable};

use crate::routing::{route::RouteRequest, simple_algorithms::ch_bi_dijkstra::ChDijkstra};

#[derive(Serialize, Deserialize, Clone, Readable, Writable)]
pub struct LabelEntry {
    pub id: u32,
    pub cost: u32,
}

#[derive(Serialize, Deserialize, Readable, Writable)]
pub struct Label {
    pub label: Vec<LabelEntry>,
}

#[derive(Serialize, Deserialize, Readable, Writable)]
pub struct HubGraph {
    pub forward_labels: Vec<Label>,
    pub backward_labels: Vec<Label>,
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
        labels.sort_unstable_by_key(|entry| entry.id);
        labels.shrink_to_fit();

        Label { label: labels }
    }

    pub fn prune_forward(&mut self, source: u32, dijkstra: &ChDijkstra) {
        self.label = self
            .label
            .par_iter()
            .progress()
            .filter(|entry| {
                let request = RouteRequest {
                    source,
                    target: entry.id,
                };
                let true_cost = dijkstra.get_cost(&request).unwrap();
                entry.cost == true_cost
            })
            .cloned()
            .collect();
    }

    pub fn prune_backward(&mut self, target: u32, dijkstra: &ChDijkstra) {
        self.label = self
            .label
            .par_iter()
            .progress()
            .filter(|entry| {
                let request = RouteRequest {
                    source: entry.id,
                    target,
                };
                let true_cost = dijkstra.get_cost(&request).unwrap();
                entry.cost == true_cost
            })
            .cloned()
            .collect();
    }

    pub fn minimal_overlapp(&self, other: &Label) -> Option<LabelEntry> {
        let mut i_self = 0;
        let mut i_other = 0;

        let mut id = u32::MAX;
        let mut cost = u32::MAX;

        while i_self < self.label.len() && i_other < other.label.len() {
            let self_entry = &self.label[i_self];
            let other_entry = &other.label[i_other];

            match self_entry.id.cmp(&other_entry.id) {
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
        let style =
            ProgressStyle::with_template("{wide_bar} {human_pos}/{human_len} {eta_precise}")
                .unwrap();
        let pb = ProgressBar::new((dijkstra.graph.num_nodes * 2) as u64);
        pb.set_style(style);
        let forward_labels = (0..dijkstra.graph.num_nodes)
            .into_par_iter()
            .progress_with(pb.clone())
            .map(|id| Label::new(&dijkstra.get_forward_label(id, depth_limit)))
            .collect();
        pb.set_position(dijkstra.graph.num_nodes as u64);
        let backward_labels = (0..dijkstra.graph.num_nodes)
            .into_par_iter()
            .progress_with(pb)
            .map(|id| Label::new(&dijkstra.get_backward_label(id, depth_limit)))
            .collect();
        HubGraph {
            forward_labels,
            backward_labels,
        }
    }

    pub fn get_avg_label_size(&self) -> f32 {
        let summed_label_size: u32 = self
            .forward_labels
            .iter()
            .map(|label| label.label.len() as u32)
            .sum::<u32>()
            + self
                .backward_labels
                .iter()
                .map(|label| label.label.len() as u32)
                .sum::<u32>();
        summed_label_size as f32 / (2 * self.forward_labels.len()) as f32
    }

    pub fn prune(&mut self) {
        for source in (0..self.forward_labels.len()).progress() {
            self.forward_labels[source].label = self.forward_labels[source]
                .label
                .par_iter()
                .filter(|entry| {
                    let request = RouteRequest {
                        source: source as u32,
                        target: entry.id,
                    };
                    let true_cost = self.get_cost(&request).unwrap();
                    entry.cost == true_cost
                })
                .cloned()
                .collect();
        }
        for target in (0..self.backward_labels.len()).progress() {
            self.backward_labels[target].label = self.backward_labels[target]
                .label
                .par_iter()
                .filter(|entry| {
                    let request = RouteRequest {
                        source: entry.id,
                        target: target as u32,
                    };
                    let true_cost = self.get_cost(&request).unwrap();
                    entry.cost == true_cost
                })
                .cloned()
                .collect();
        }
    }

    pub fn get_cost(&self, request: &RouteRequest) -> Option<u32> {
        let forward_label = self.forward_labels.get(request.source as usize)?;
        let backward_label = self.backward_labels.get(request.target as usize)?;
        Some(forward_label.minimal_overlapp(backward_label)?.cost)
    }
}
