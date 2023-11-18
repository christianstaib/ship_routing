pub struct BucketQueue {
    i: usize,
    buckets: Vec<Vec<u32>>,
}

impl BucketQueue {
    pub fn new(max: u32) -> BucketQueue {
        BucketQueue {
            i: 0,
            buckets: vec![Vec::with_capacity(50_000); max as usize],
        }
    }

    pub fn insert(&mut self, key: u32, value: u32) {
        let idx = key as usize % self.buckets.len();
        self.buckets[idx].push(value)
    }

    pub fn pop(&mut self) -> Option<u32> {
        for j in 0..self.buckets.len() {
            let idx = (self.i + j) % self.buckets.len();
            if let Some(value) = self.buckets[idx].pop() {
                self.i = idx;
                return Some(value);
            }
        }
        None
    }
}
