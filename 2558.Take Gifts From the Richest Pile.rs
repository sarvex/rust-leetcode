use std::collections::BinaryHeap;

impl Solution {
    /// Repeatedly take sqrt of the richest pile using a max-heap.
    ///
    /// # Intuition
    /// A max-heap efficiently retrieves the largest pile each round.
    /// After k operations, the total remaining gifts is the heap sum.
    ///
    /// # Approach
    /// 1. Build a max-heap from gifts
    /// 2. For k rounds, pop the max, push back its floor-sqrt
    /// 3. Sum all remaining elements
    ///
    /// # Complexity
    /// - Time: O(n + k log n)
    /// - Space: O(n)
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(gifts);

        for _ in 0..k {
            if let Some(max_gift) = heap.pop() {
                heap.push((max_gift as f64).sqrt().floor() as i32);
            }
        }

        heap.into_iter().map(|x| x as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
    }

    #[test]
    fn test_single_pile() {
        assert_eq!(Solution::pick_gifts(vec![1], 1), 1);
    }

    #[test]
    fn test_zero_operations() {
        assert_eq!(Solution::pick_gifts(vec![10, 20], 0), 30);
    }
}
