use serde_derive::{Deserialize, Serialize};
use speedy::{Readable, Writable};

#[derive(Serialize, Deserialize, Clone, Readable, Writable)]
pub struct LabelEntry {
    pub id: u32,
    pub cost: u32,
}
