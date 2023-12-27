use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelEntry {
    pub id: u32,
    pub cost: u32,
}

pub struct Label {
    labels: Vec<LabelEntry>,
}

impl Label {
    pub fn new(map: &HashMap<u32, u32>) -> Label {
        let mut labels: Vec<_> = map
            .iter()
            .map(|(id, cost)| LabelEntry {
                id: *id,
                cost: *cost,
            })
            .collect();
        labels.sort_unstable();

        Label { labels }
    }

    pub fn minimal_overlapp(&self, other: &Label) -> Option<LabelEntry> {
        let mut i_self = 0;
        let mut i_other = 0;

        let mut id = u32::MAX;
        let mut cost = u32::MAX;

        while i_self < self.labels.len() && i_other < other.labels.len() {
            let self_entry = &self.labels[i_self];
            let other_entry = &self.labels[i_other];

            match self_entry.cmp(other_entry) {
                std::cmp::Ordering::Less => i_self += 1,
                std::cmp::Ordering::Equal => {
                    i_self += 1;
                    i_other += 1;

                    let alternative_cost = self_entry.cost + other_entry.cost;
                    if alternative_cost < cost {
                        id = self_entry.id;
                        cost = alternative_cost;
                    }
                }
                std::cmp::Ordering::Greater => i_other += 1,
            }
        }

        if cost != u32::MAX {
            return Some(LabelEntry { id, cost });
        }

        None
    }
}
