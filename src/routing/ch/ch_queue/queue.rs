use std::collections::BinaryHeap;

use indicatif::ProgressIterator;
use rand::seq::SliceRandom;

use crate::routing::graph::Graph;

use super::{edge_difference::EdgeDifferencePriority, state::CHState};

pub trait PriorityTerm {
    fn priority(&self, v: u32, graph: &Graph) -> i32;
    fn update(&mut self, v: u32);
}

pub struct CHQueue {
    queue: BinaryHeap<CHState>,
    priority_terms: Vec<(i32, Box<dyn PriorityTerm>)>,
}

impl CHQueue {
    pub fn new(graph: &Graph) -> Self {
        let queue = BinaryHeap::new();
        let priority_terms = Vec::new();
        let mut queue = Self {
            queue,
            priority_terms,
        };
        queue.register(1, EdgeDifferencePriority::new());
        queue.initialize(graph);
        queue
    }

    fn register(&mut self, weight: i32, term: impl PriorityTerm + 'static) {
        self.priority_terms.push((weight, Box::new(term)));
    }

    pub fn lazy_pop(&mut self, graph: &Graph) -> Option<u32> {
        while let Some(state) = self.queue.pop() {
            let v = state.node_id;
            if self.get_priority(v, graph) > state.priority {
                self.queue
                    .push(CHState::new(self.get_priority(v, graph), v));
                continue;
            }
            self.update_priority(v);
            return Some(v);
        }
        None
    }

    fn update_priority(&mut self, v: u32) {
        self.priority_terms
            .iter_mut()
            .for_each(|priority_term| priority_term.1.update(v));
    }

    pub fn get_priority(&self, v: u32, graph: &Graph) -> i32 {
        let priorities: Vec<i32> = self
            .priority_terms
            .iter()
            .map(|priority_term| priority_term.0 * priority_term.1.priority(v, graph))
            .collect();

        priorities.iter().sum()
    }

    fn initialize(&mut self, graph: &Graph) {
        let mut order: Vec<u32> = (0..graph.forward_edges.len()).map(|x| x as u32).collect();
        order.shuffle(&mut rand::thread_rng());

        for &v in order.iter().progress() {
            self.queue.push(CHState {
                priority: self.get_priority(v, graph),
                node_id: v,
            });
        }
    }
}
