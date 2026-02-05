impl Solution {
    /// Simulates the prescribed pair-merge process and stops once sorted.
    ///
    /// # Intuition
    /// Each operation is fully deterministic: pick the leftmost adjacent pair
    /// with the minimum sum and replace it by that sum. We can simulate the
    /// process step by step and check after every merge whether the array has
    /// become non-decreasing.
    ///
    /// # Approach
    /// - If the array is already non-decreasing, return 0.
    /// - Otherwise, repeat while more than one element remains:
    ///   - Find the leftmost adjacent pair with the minimum sum.
    ///   - Replace the pair with their sum (shrinking the array by one).
    ///   - After the merge, if the array is non-decreasing, return the number
    ///     of operations performed so far.
    /// - Eventually a single element remains, which is trivially sorted.
    ///
    /// # Complexity
    /// - Time: O(n²) in the worst case (n ≤ 50), because each of up to n merges
    ///   scans the current array.
    /// - Space: O(1) beyond the input buffer, mutating in place.
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        fn is_sorted(values: &[i32]) -> bool {
            values.windows(2).all(|w| w[0] <= w[1])
        }

        if is_sorted(&nums) {
            return 0;
        }

        let mut ops = 0;
        while nums.len() > 1 {
            let (idx, min_sum) = nums
                .windows(2)
                .enumerate()
                .map(|(i, w)| (i, w[0] + w[1]))
                .min_by_key(|&(_, sum)| sum)
                .unwrap();

            nums[idx] = min_sum;
            nums.remove(idx + 1);
            ops += 1;

            if is_sorted(&nums) {
                return ops;
            }
        }

        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
    }

    #[test]
    fn already_sorted_single() {
        assert_eq!(Solution::minimum_pair_removal(vec![7]), 0);
    }

    #[test]
    fn strictly_decreasing() {
        assert_eq!(Solution::minimum_pair_removal(vec![3, 2, 1]), 1);
    }

    #[test]
    fn negatives_and_mixed() {
        assert_eq!(Solution::minimum_pair_removal(vec![-1, -2, 3]), 1);
    }
}
