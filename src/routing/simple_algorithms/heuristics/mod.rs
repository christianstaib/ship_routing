pub trait Heuristic {
    /// Lower bound for the cost from node some target node.
    fn lower_bound(&self, node: u32) -> u32;
}

pub mod distance;
pub mod landmark;
pub mod zero;
