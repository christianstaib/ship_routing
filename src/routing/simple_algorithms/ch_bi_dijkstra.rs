use std::collections::BinaryHeap;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

use crate::routing::{
    ch::contractor::ContractedGraph,
    fast_graph::FastGraph,
    queue::heap_queue::State,
    route::{Route, RouteRequest},
};

#[derive(Clone)]
pub struct ChDijkstra<'a> {
    pub graph: &'a FastGraph,
    pub shortcuts: &'a HashMap<(u32, u32), Vec<(u32, u32)>>,
}

impl<'a> ChDijkstra<'a> {
    pub fn new(
        graph: &'a FastGraph,
        shortcuts: &'a HashMap<(u32, u32), Vec<(u32, u32)>>,
    ) -> ChDijkstra<'a> {
        ChDijkstra { graph, shortcuts }
    }

    ///
    /// (contact_node, cost)
    pub fn get_forward_label(&self, source: u32, depth_limit: u32) -> HashMap<u32, u32> {
        let mut costs = HashMap::with_capacity(5000);
        let mut open = BinaryHeap::with_capacity(5000);
        let mut expanded = HashSet::with_capacity(5000);

        open.push(State {
            key: 0,
            value: source,
        });
        costs.insert(source, 0);

        while let Some(state) = open.pop() {
            let current_node = state.value;
            if !expanded.contains(&current_node) {
                expanded.insert(current_node);

                let current_node_cost = *costs.get(&current_node).unwrap();

                let backward_search = self.backward_search(current_node, depth_limit);
                let incoming_min = backward_search
                    .iter()
                    .map(|(node, cost)| {
                        costs
                            .get(&node)
                            .unwrap_or(&u32::MAX)
                            .checked_add(*cost)
                            .unwrap_or(u32::MAX)
                    })
                    .min()
                    .unwrap_or(u32::MAX);

                if current_node_cost > incoming_min {
                    costs.remove(&current_node);
                    continue;
                }

                self.graph
                    .outgoing_edges(state.value)
                    .iter()
                    .for_each(|edge| {
                        let alternative_cost = current_node_cost + edge.cost;
                        let current_cost = *costs.get(&edge.target).unwrap_or(&u32::MAX);
                        if alternative_cost < current_cost {
                            costs.insert(edge.target, alternative_cost);
                            open.push(State {
                                key: alternative_cost,
                                value: edge.target,
                            });
                        }
                    });
            }
        }

        costs
    }

    ///
    /// (contact_node, cost)
    pub fn get_backward_label(&self, source: u32, depth_limit: u32) -> HashMap<u32, u32> {
        let mut costs = HashMap::with_capacity(5000);
        let mut open = BinaryHeap::with_capacity(5000);
        let mut expanded = HashSet::with_capacity(5000);

        open.push(State {
            key: 0,
            value: source,
        });
        costs.insert(source, 0);

        while let Some(state) = open.pop() {
            let current_node = state.value;
            if !expanded.contains(&current_node) {
                expanded.insert(current_node);

                let current_node_cost = *costs.get(&current_node).unwrap();

                let backward_search = self.forward_search(current_node, depth_limit);
                let incoming_min = backward_search
                    .iter()
                    .map(|(node, cost)| {
                        costs
                            .get(&node)
                            .unwrap_or(&u32::MAX)
                            .checked_add(*cost)
                            .unwrap_or(u32::MAX)
                    })
                    .min()
                    .unwrap_or(u32::MAX);

                if current_node_cost > incoming_min {
                    costs.remove(&current_node);
                    continue;
                }

                self.graph
                    .incoming_edges(state.value)
                    .iter()
                    .for_each(|edge| {
                        let alternative_cost = current_node_cost + edge.cost;
                        let current_cost = *costs.get(&edge.target).unwrap_or(&u32::MAX);
                        if alternative_cost < current_cost {
                            costs.insert(edge.target, alternative_cost);
                            open.push(State {
                                key: alternative_cost,
                                value: edge.target,
                            });
                        }
                    });
            }
        }

        costs
    }

    ///
    /// (contact_node, cost)
    pub fn forward_search(&self, target: u32, depth_limit: u32) -> HashMap<u32, u32> {
        let mut costs = HashMap::with_capacity(5000);
        let mut depth = HashMap::with_capacity(5000);
        let mut open = BinaryHeap::with_capacity(5000);
        let mut expanded = HashSet::with_capacity(5000);

        open.push(State {
            key: 0,
            value: target,
        });
        costs.insert(target, 0);
        depth.insert(target, 0);

        while let Some(state) = open.pop() {
            let current_node = state.value;
            let current_node_cost = *costs.get(&current_node).unwrap();
            let new_depth = depth.get(&current_node).unwrap() + 1;
            if !expanded.contains(&current_node) && (new_depth <= depth_limit) {
                expanded.insert(current_node);

                self.graph
                    .incoming_edges(state.value)
                    .iter()
                    .for_each(|edge| {
                        let alternative_cost = current_node_cost + edge.cost;
                        let current_cost = *costs.get(&edge.target).unwrap_or(&u32::MAX);
                        if alternative_cost < current_cost {
                            costs.insert(edge.target, alternative_cost);
                            depth.insert(edge.target, new_depth);
                            open.push(State {
                                key: alternative_cost,
                                value: edge.target,
                            });
                        }
                    });
            }
        }
        costs
    }

    ///
    /// (contact_node, cost)
    pub fn backward_search(&self, target: u32, depth_limit: u32) -> HashMap<u32, u32> {
        let mut costs = HashMap::with_capacity(5000);
        let mut depth = HashMap::with_capacity(5000);
        let mut open = BinaryHeap::with_capacity(5000);
        let mut expanded = HashSet::with_capacity(5000);

        open.push(State {
            key: 0,
            value: target,
        });
        costs.insert(target, 0);
        depth.insert(target, 0);

        while let Some(state) = open.pop() {
            let current_node = state.value;
            let current_node_cost = *costs.get(&current_node).unwrap();
            let new_depth = depth.get(&current_node).unwrap() + 1;
            if !expanded.contains(&current_node) && (new_depth <= depth_limit) {
                expanded.insert(current_node);

                self.graph
                    .incoming_edges(state.value)
                    .iter()
                    .for_each(|edge| {
                        let alternative_cost = current_node_cost + edge.cost;
                        let current_cost = *costs.get(&edge.target).unwrap_or(&u32::MAX);
                        if alternative_cost < current_cost {
                            costs.insert(edge.target, alternative_cost);
                            depth.insert(edge.target, new_depth);
                            open.push(State {
                                key: alternative_cost,
                                value: edge.target,
                            });
                        }
                    });
            }
        }
        costs
    }

    /// (contact_node, cost)
    pub fn get_route(&self, request: &RouteRequest) -> Option<Route> {
        let mut forward_costs = HashMap::with_capacity(5000);
        let mut backward_costs = HashMap::with_capacity(5000);

        let mut forward_predecessor = HashMap::with_capacity(5000);
        let mut backward_predecessor = HashMap::with_capacity(5000);

        let mut forward_open = BinaryHeap::with_capacity(5000);
        let mut backward_open = BinaryHeap::with_capacity(5000);

        let mut forward_expanded = HashSet::with_capacity(5000);
        let mut backward_expaned = HashSet::with_capacity(5000);

        forward_open.push(State {
            key: 0,
            value: request.source,
        });
        forward_costs.insert(request.source, 0);

        backward_open.push(State {
            key: 0,
            value: request.target,
        });
        backward_costs.insert(request.target, 0);

        let mut minimal_cost = u32::MAX;
        let mut meeting_node = u32::MAX;

        while !forward_open.is_empty() || !backward_open.is_empty() {
            if let Some(forward_state) = forward_open.pop() {
                let current_node = forward_state.value;
                if !forward_expanded.contains(&current_node) {
                    forward_expanded.insert(current_node);

                    if backward_expaned.contains(&current_node) {
                        let contact_cost = forward_costs.get(&current_node).unwrap()
                            + backward_costs.get(&current_node).unwrap();
                        if contact_cost < minimal_cost {
                            minimal_cost = contact_cost;
                            meeting_node = forward_state.value;
                        }
                    }

                    self.graph
                        .outgoing_edges(forward_state.value)
                        .iter()
                        .for_each(|edge| {
                            let alternative_cost =
                                forward_costs.get(&current_node).unwrap() + edge.cost;
                            let current_cost =
                                *forward_costs.get(&edge.target).unwrap_or(&u32::MAX);
                            if alternative_cost < current_cost {
                                forward_costs.insert(edge.target, alternative_cost);
                                forward_predecessor.insert(edge.target, current_node);
                                forward_open.push(State {
                                    key: alternative_cost,
                                    value: edge.target,
                                });
                            }
                        });
                }
            }

            if let Some(backward_state) = backward_open.pop() {
                let current_node = backward_state.value;
                if !backward_expaned.contains(&current_node) {
                    backward_expaned.insert(current_node);

                    if forward_expanded.contains(&current_node) {
                        let contact_cost = forward_costs.get(&current_node).unwrap()
                            + backward_costs.get(&current_node).unwrap();
                        if contact_cost < minimal_cost {
                            minimal_cost = contact_cost;
                            meeting_node = backward_state.value;
                        }
                    }

                    self.graph
                        .incoming_edges(backward_state.value)
                        .iter()
                        .for_each(|edge| {
                            let alternative_cost =
                                backward_costs.get(&current_node).unwrap() + edge.cost;
                            let current_cost =
                                *backward_costs.get(&edge.target).unwrap_or(&u32::MAX);
                            if alternative_cost < current_cost {
                                backward_costs.insert(edge.target, alternative_cost);
                                backward_predecessor.insert(edge.target, current_node);
                                backward_open.push(State {
                                    key: alternative_cost,
                                    value: edge.target,
                                });
                            }
                        });
                }
            }
        }

        get_route(
            meeting_node,
            minimal_cost,
            forward_predecessor,
            backward_predecessor,
        )
    }
}

fn get_route(
    meeting_node: u32,
    meeting_cost: u32,
    forward_predecessor: HashMap<u32, u32>,
    backward_predecessor: HashMap<u32, u32>,
) -> Option<Route> {
    if meeting_cost == u32::MAX {
        return None;
    }
    let mut route = Vec::new();
    let mut current = meeting_node;
    route.push(current);
    while let Some(new_current) = forward_predecessor.get(&current) {
        current = *new_current;
        // route.insert(0, current);
    }
    current = meeting_node;
    while let Some(new_current) = backward_predecessor.get(&current) {
        current = *new_current;
        // route.push(current);
    }
    let route = Route {
        nodes: route,
        cost: meeting_cost,
    };
    Some(route)
}
