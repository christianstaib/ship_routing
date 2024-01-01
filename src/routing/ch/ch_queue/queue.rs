use std::{collections::BinaryHeap, time::Instant};

use indicatif::ProgressIterator;
use rand::seq::SliceRandom;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::routing::{ch::ch_queue::deleted_neighbors::DeletedNeighbors, graph::Graph};

use super::{edge_difference::EdgeDifferencePriority, state::CHState};

pub trait PriorityTerm {
    /// Gets the priority of node v in the graph
    fn priority(&self, v: u32, graph: &Graph) -> i32;

    /// Gets called just before a v is contracted. Gives priority terms the oppernunity to updated
    /// neighboring nodes priorities.
    fn update_before_contraction(&mut self, v: u32);
}

pub struct CHQueue {
    queue: BinaryHeap<CHState>,
    priority_terms: Vec<(i32, Box<dyn PriorityTerm + Sync>)>,
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
        queue.register(1, DeletedNeighbors::new(graph.forward_edges.len() as u32));
        queue.initialize(graph);
        queue
    }

    fn register(&mut self, weight: i32, term: impl PriorityTerm + 'static + Sync) {
        self.priority_terms.push((weight, Box::new(term)));
    }

    pub fn lazy_pop(&mut self, graph: &Graph) -> Option<u32> {
        while let Some(state) = self.queue.pop() {
            let v = state.node_id;
            let new_priority = self.get_priority(v, graph);
            if new_priority > state.priority {
                self.queue.push(CHState::new(new_priority, v));
                continue;
            }
            self.update_before_contraction(v);
            return Some(v);
        }
        None
    }

    /// Gets called just before a v is contracted. Gives priority terms the oppernunity to updated
    /// neighboring nodes priorities.
    fn update_before_contraction(&mut self, v: u32) {
        self.priority_terms
            .iter_mut()
            .for_each(|priority_term| priority_term.1.update_before_contraction(v));
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

        self.queue = order
            .iter()
            .progress()
            .par_bridge()
            .map(|&v| CHState {
                priority: self.get_priority(v, graph),
                node_id: v,
            })
            .collect();
    }
}
