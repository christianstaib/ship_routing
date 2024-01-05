use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde_derive::{Deserialize, Serialize};
use speedy::{Readable, Writable};

use crate::routing::{route::RouteRequest, simple_algorithms::ch_bi_dijkstra::ChDijkstra};

use super::label::Label;

#[derive(Serialize, Deserialize, Readable, Writable)]
pub struct HubGraph {
    pub forward_labels: Vec<Label>,
    pub backward_labels: Vec<Label>,
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
        let summed_label_size: u64 = self
            .forward_labels
            .iter()
            .map(|label| label.label.len() as u64)
            .sum::<u64>()
            + self
                .backward_labels
                .iter()
                .map(|label| label.label.len() as u64)
                .sum::<u64>();
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
        forward_label.get_cost(backward_label)
    }
}
