use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /// Finds minimum time to build all blocks using Huffman-like merging.
    ///
    /// # Intuition
    /// Splitting a worker costs `split` time. Optimally, pair the two smallest
    /// tasks and add the split cost, similar to Huffman coding.
    ///
    /// # Approach
    /// Use a min-heap. Repeatedly pop the two smallest, combine them with
    /// the split cost, and push back. The last element is the answer.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n) for the heap
    pub fn min_build_time(blocks: Vec<i32>, split: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<i32>> = blocks.into_iter().map(Reverse).collect();
        while heap.len() > 1 {
            heap.pop();
            let second = heap.pop().unwrap().0;
            heap.push(Reverse(second + split));
        }
        heap.pop().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_build_time(vec![1], 1), 1);
    }

    #[test]
    fn test_two_blocks() {
        assert_eq!(Solution::min_build_time(vec![1, 2], 5), 7);
    }

    #[test]
    fn test_three_blocks() {
        assert_eq!(Solution::min_build_time(vec![1, 2, 3], 1), 4);
    }
}
