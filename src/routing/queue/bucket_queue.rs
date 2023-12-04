pub struct BucketQueue {
    current_index: usize,
    buckets: Vec<Vec<u32>>,
}

impl BucketQueue {
    pub fn new(max_diff: u32) -> BucketQueue {
        BucketQueue {
            current_index: 0,
            buckets: vec![Vec::new(); max_diff as usize],
        }
    }

    pub fn insert(&mut self, key: u32, value: u32) {
        let key_index = key as usize % self.buckets.len();
        self.buckets[key_index].push(value)
    }

    pub fn pop(&mut self) -> Option<u32> {
        for bucket_index in 0..self.buckets.len() {
            let key_index = (self.current_index + bucket_index) % self.buckets.len();
            if let Some(value) = self.buckets[key_index].pop() {
                self.current_index = key_index;
                return Some(value);
            }
        }
        None
    }
}
