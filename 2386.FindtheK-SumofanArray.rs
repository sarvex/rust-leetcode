use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /// Finds the kth largest subsequence sum using min-heap with absolute value transformation
    ///
    /// # Intuition
    /// The maximum subsequence sum is achieved by taking all positive numbers. Any other
    /// subsequence sum can be expressed as this maximum minus some combination of absolute
    /// values (either excluding a positive or including a negative).
    ///
    /// # Approach
    /// 1. Calculate `max_sum` as the sum of all positive numbers (1st largest)
    /// 2. Convert all numbers to absolute values and sort them
    /// 3. Use a min-heap to explore subsequence sums in increasing order
    /// 4. Each heap state (sum, index) represents a sum to subtract from max_sum
    /// 5. For each popped state, generate next states by either:
    ///    - Adding the next absolute value (extend subsequence)
    ///    - Replacing current value with next (swap last element)
    ///
    /// # Complexity
    /// - Time: O(n log n + k log k) for sorting and k heap operations
    /// - Space: O(n + k) for sorted array and heap storage
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();

        let max_sum: i64 = nums.iter().filter(|&&x| x > 0).map(|&x| x as i64).sum();

        if k == 1 {
            return max_sum;
        }

        let mut abs_values: Vec<i64> = nums.iter().map(|&x| (x as i64).abs()).collect();
        abs_values.sort_unstable();

        let mut min_heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        min_heap.push(Reverse((abs_values[0], 0)));

        let k = k as usize;
        let mut count = 1;

        while let Some(Reverse((subtract_sum, idx))) = min_heap.pop() {
            count += 1;

            if count == k {
                return max_sum - subtract_sum;
            }

            let next_idx = idx + 1;
            if next_idx < n {
                min_heap.push(Reverse((subtract_sum + abs_values[next_idx], next_idx)));
                min_heap.push(Reverse((
                    subtract_sum - abs_values[idx] + abs_values[next_idx],
                    next_idx,
                )));
            }
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 4, -2];
        let k = 5;
        assert_eq!(Solution::k_sum(nums, k), 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, -2, 3, 4, -10, 12];
        let k = 16;
        assert_eq!(Solution::k_sum(nums, k), 10);
    }

    #[test]
    fn test_k_equals_one() {
        let nums = vec![5, -3, 2, -1];
        let k = 1;
        assert_eq!(Solution::k_sum(nums, k), 7);
    }

    #[test]
    fn test_all_negative() {
        let nums = vec![-5, -3, -2];
        let k = 1;
        assert_eq!(Solution::k_sum(nums, k), 0);
    }

    #[test]
    fn test_single_element_positive() {
        let nums = vec![10];
        let k = 1;
        assert_eq!(Solution::k_sum(nums, k), 10);
    }

    #[test]
    fn test_single_element_negative() {
        let nums = vec![-10];
        let k = 2;
        assert_eq!(Solution::k_sum(nums, k), -10);
    }

    #[test]
    fn test_all_positive() {
        let nums = vec![1, 2, 3];
        let k = 4;
        assert_eq!(Solution::k_sum(nums, k), 5);
    }
}
