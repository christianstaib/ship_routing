use super::Heuristic;

pub struct Zero {}

impl Heuristic for Zero {
    fn lower_bound(&self, _: u32, _: u32) -> u32 {
        0
    }
}
