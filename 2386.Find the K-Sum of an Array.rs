use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /// Finds the kth largest subsequence sum using min-heap exploration.
    ///
    /// # Intuition
    /// The maximum subsequence sum is the sum of all positive numbers. Any other
    /// sum equals this maximum minus some combination of absolute values (excluding
    /// a positive or including a negative).
    ///
    /// # Approach
    /// 1. Compute `max_sum` as the sum of all positive numbers
    /// 2. Convert all numbers to absolute values and sort
    /// 3. Use a min-heap to explore subtraction sums in increasing order
    /// 4. Each state (sum, index) branches into extend and swap transitions
    ///
    /// # Complexity
    /// - Time: O(n log n + k log k)
    /// - Space: O(n + k)
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let max_sum: i64 = nums.iter().filter(|x| **x > 0).map(|x| *x as i64).sum();

        if k == 1 {
            return max_sum;
        }

        let mut abs_values: Vec<i64> = nums.iter().map(|x| (*x as i64).abs()).collect();
        abs_values.sort_unstable();

        let n = abs_values.len();
        let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        heap.push(Reverse((abs_values[0], 0)));

        let k = k as usize;
        let mut count = 1;

        while let Some(Reverse((subtract_sum, idx))) = heap.pop() {
            count += 1;
            if count == k {
                return max_sum - subtract_sum;
            }

            let next = idx + 1;
            if next < n {
                heap.push(Reverse((subtract_sum + abs_values[next], next)));
                heap.push(Reverse((
                    subtract_sum - abs_values[idx] + abs_values[next],
                    next,
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
        assert_eq!(Solution::k_sum(vec![2, 4, -2], 5), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::k_sum(vec![1, -2, 3, 4, -10, 12], 16), 10);
    }

    #[test]
    fn test_k_equals_one() {
        assert_eq!(Solution::k_sum(vec![5, -3, 2, -1], 1), 7);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(Solution::k_sum(vec![-5, -3, -2], 1), 0);
    }

    #[test]
    fn test_single_positive() {
        assert_eq!(Solution::k_sum(vec![10], 1), 10);
    }

    #[test]
    fn test_single_negative() {
        assert_eq!(Solution::k_sum(vec![-10], 2), -10);
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(Solution::k_sum(vec![1, 2, 3], 4), 5);
    }
}
