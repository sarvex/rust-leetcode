use rand::{thread_rng, Rng};

/// Weighted random index picker using prefix sums and binary search.
///
/// Precomputes a prefix sum array. Each pick generates a random target
/// in [1, total_weight] and binary-searches for the corresponding index.
struct Solution {
    prefix: Vec<i32>,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        let mut sum = 0;
        for &weight in &w {
            sum += weight;
            prefix.push(sum);
        }
        Self { prefix }
    }

    fn pick_index(&self) -> i32 {
        let total = *self.prefix.last().unwrap();
        let target = thread_rng().gen_range(1, total + 1);
        match self.prefix.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}
