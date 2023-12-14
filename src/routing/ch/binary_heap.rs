use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MinimumItem {
    pub priority: u32,
    pub item: u32,
}

impl MinimumItem {
    pub fn new(priority: u32, item: u32) -> Self {
        Self { priority, item }
    }
}

impl Ord for MinimumItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.item.cmp(&other.item))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for MinimumItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
