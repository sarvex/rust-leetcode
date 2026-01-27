use std::collections::BinaryHeap;

impl Solution {
    /// Maximizes score by greedily selecting the largest element k times.
    ///
    /// # Intuition
    /// Always pick the maximum element for the highest score. After picking,
    /// replace it with ceil(value / 3) to model the reduction.
    ///
    /// # Approach
    /// Use a max-heap to efficiently extract and reinsert elements k times.
    ///
    /// # Complexity
    /// - Time: O(n + k log n) — heapify then k extract/insert operations
    /// - Space: O(n) — binary heap
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(nums);

        (0..k).fold(0i64, |score, _| {
            if let Some(v) = heap.pop() {
                heap.push((v + 2) / 3);
                score + v as i64
            } else {
                score
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_kelements(vec![1], 3), 3);
    }
}
