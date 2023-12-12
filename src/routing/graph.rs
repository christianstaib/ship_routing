use super::fast_graph::FastEdge;

#[derive(Clone)]
pub struct Edge {
    pub source: u32,
    pub target: u32,
    pub cost: u32,
}

impl Edge {
    pub fn new(source: u32, target: u32, cost: u32) -> Edge {
        Edge {
            source,
            target,
            cost,
        }
    }

    pub fn get_inverted(&self) -> Edge {
        Edge {
            source: self.target,
            target: self.source,
            cost: self.cost,
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub id: u32,
    pub longitude: f64,
    pub latitude: f64,
}

impl Edge {
    pub fn make_fast(&self) -> FastEdge {
        FastEdge {
            target: self.target,
            cost: self.cost,
        }
    }
}

impl Node {
    pub fn new(id: u32, longitude: f64, latitude: f64) -> Node {
        Node {
            id,
            longitude,
            latitude,
        }
    }
}
