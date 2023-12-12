use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Eq)]
struct State {
    key: i32,
    value: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .key
            .cmp(&self.key)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct LazyQueue {
    heap: BinaryHeap<State>,
    current_min: i32,
}

impl LazyQueue {
    pub fn pop(&mut self) -> Option<u32> {
        while let Some(state) = self.heap.pop() {}

        None
    }
}
