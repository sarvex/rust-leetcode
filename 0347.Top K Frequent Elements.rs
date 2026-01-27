use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    /// Finds the k most frequent elements using a min-heap of size k.
    ///
    /// # Intuition
    /// A min-heap of size k keeps the top-k frequent elements. Elements with
    /// lower frequency are evicted as the heap overflows.
    ///
    /// # Approach
    /// 1. Count frequencies with a HashMap.
    /// 2. Push (frequency, element) pairs into a min-heap.
    /// 3. If the heap exceeds size k, pop the minimum.
    /// 4. Collect remaining heap elements.
    ///
    /// # Complexity
    /// - Time: O(n log k)
    /// - Space: O(n) for the frequency map
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();
        for x in nums {
            *counts.entry(x).or_insert(0) += 1;
        }
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for (&val, &freq) in &counts {
            heap.push(Reverse((freq, val)));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|Reverse((_, x))| x).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_two() {
        let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
