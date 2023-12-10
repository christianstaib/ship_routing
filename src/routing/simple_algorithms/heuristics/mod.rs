pub trait Heuristic {
    fn lower_bound(&self, source: u32, target: u32) -> u32;
}

pub mod distance;
pub mod landmark;
pub mod zero;
