use std::collections::BinaryHeap;

impl Solution {
    /// Minimum operations to make all elements at least k using combine-two-smallest.
    ///
    /// # Intuition
    /// By always combining the two smallest elements (2*min + second_min), we
    /// optimally raise the minimum value. A max-heap with negated values serves
    /// as a min-heap.
    ///
    /// # Approach
    /// 1. Push all elements as negated values into a binary heap (min-heap simulation).
    /// 2. While the smallest element is below k, pop two smallest, combine, and push back.
    /// 3. Count each combine operation.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut pq = BinaryHeap::with_capacity(nums.len());
        nums.iter().for_each(|&x| pq.push(-(x as i64)));

        let mut ans = 0;
        let target = k as i64;

        while pq.len() > 1 && -pq.peek().unwrap() < target {
            let x = -pq.pop().unwrap();
            let y = -pq.pop().unwrap();
            pq.push(-(x * 2 + y));
            ans += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_operations_needed() {
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 2);
    }

    #[test]
    fn already_above_threshold() {
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 1), 0);
    }
}
