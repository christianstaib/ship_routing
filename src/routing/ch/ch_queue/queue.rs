use std::{collections::BinaryHeap, time::Instant};

use indicatif::ProgressIterator;
use rand::seq::SliceRandom;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::routing::{ch::ch_queue::deleted_neighbors::DeletedNeighbors, graph::Graph};

use super::{edge_difference::EdgeDifferencePriority, state::CHState};

pub trait PriorityTerm {
    fn priority(&self, v: u32, graph: &Graph) -> i32;
    fn update(&mut self, v: u32);
}

pub struct CHQueue {
    i: u32,
    queue: BinaryHeap<CHState>,
    priority_terms: Vec<(i32, Box<dyn PriorityTerm + Sync>)>,
}

impl CHQueue {
    pub fn new(graph: &Graph) -> Self {
        let queue = BinaryHeap::new();
        let priority_terms = Vec::new();
        let mut queue = Self {
            i: 0,
            queue,
            priority_terms,
        };
        queue.register(1, EdgeDifferencePriority::new());
        queue.register(1, DeletedNeighbors::new(graph.forward_edges.len() as u32));
        let start = Instant::now();
        queue.initialize(graph);
        println!("took {:?} to initialize", start.elapsed());
        queue
    }

    fn register(&mut self, weight: i32, term: impl PriorityTerm + 'static + Sync) {
        self.priority_terms.push((weight, Box::new(term)));
    }

    pub fn lazy_pop(&mut self, graph: &Graph) -> Option<u32> {
        if self.i > 100_000 {
            self.update_queue(graph);
            self.i = 0;
        }
        self.i += 1;
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

    fn update_queue(&mut self, graph: &Graph) {
        self.queue = self
            .queue
            .iter()
            .par_bridge()
            .map(|&state| CHState {
                priority: self.get_priority(state.node_id, graph),
                node_id: state.node_id,
            })
            .collect();
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
