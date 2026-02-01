impl Solution {
    /// Maximize the OR/XOR value across a length-`2k` subsequence.
    ///
    /// # Intuition
    /// Each `nums[i]` fits in 7 bits, so every OR result is in `[0, 127]`. We can track all
    /// achievable OR values when selecting `j` elements from a prefix or suffix, then combine the
    /// two sides to maximize XOR.
    ///
    /// # Approach
    /// - Maintain a DP bitset `dp[j]` for each prefix/suffix: a 1 at position `v` means OR value
    ///   `v` is achievable with exactly `j` selected elements.
    /// - Sweep left-to-right to record `prefix_sets[i]` for selecting `k` elements in `nums[0..=i]`.
    /// - Sweep right-to-left to record `suffix_sets[i]` for selecting `k` elements in `nums[i..]`.
    /// - For every valid split `i`, compute the maximum XOR between values in
    ///   `prefix_sets[i]` and `suffix_sets[i + 1]`.
    ///
    /// # Complexity
    /// - Time: O(n * k * 128 + n * 128^2)
    /// - Space: O(n * 128 + k * 128)
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        fn or_with_value_set(set: u128, value: usize) -> u128 {
            let mut result = 0u128;
            let mut remaining = set;
            while remaining != 0 {
                let idx = remaining.trailing_zeros() as usize;
                result |= 1u128 << (idx | value);
                remaining &= remaining - 1;
            }
            result
        }

        let n = nums.len();
        let k = k as usize;

        let mut prefix_sets = vec![0u128; n];
        let mut dp = vec![0u128; k + 1];
        dp[0] = 1u128;

        for (i, &num) in nums.iter().enumerate() {
            let value = num as usize;
            let max_j = (i + 1).min(k);
            for j in (1..=max_j).rev() {
                let added = or_with_value_set(dp[j - 1], value);
                dp[j] |= added;
            }
            if i + 1 >= k {
                prefix_sets[i] = dp[k];
            }
        }

        let mut suffix_sets = vec![0u128; n];
        dp.fill(0);
        dp[0] = 1u128;

        for i in (0..n).rev() {
            let value = nums[i] as usize;
            let max_j = (n - i).min(k);
            for j in (1..=max_j).rev() {
                let added = or_with_value_set(dp[j - 1], value);
                dp[j] |= added;
            }
            if n - i >= k {
                suffix_sets[i] = dp[k];
            }
        }

        let mut best = 0i32;
        let min_split = k - 1;
        let max_split = n - k - 1;
        for i in min_split..=max_split {
            let left = prefix_sets[i];
            let right = suffix_sets[i + 1];
            if left == 0 || right == 0 {
                continue;
            }
            let mut left_bits = left;
            while left_bits != 0 {
                let v1 = left_bits.trailing_zeros() as usize;
                left_bits &= left_bits - 1;
                let mut right_bits = right;
                while right_bits != 0 {
                    let v2 = right_bits.trailing_zeros() as usize;
                    right_bits &= right_bits - 1;
                    let value = (v1 ^ v2) as i32;
                    if value > best {
                        best = value;
                    }
                }
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::max_value(vec![2, 6, 7], 1), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::max_value(vec![4, 2, 5, 6, 7], 2), 2);
    }

    #[test]
    fn all_elements_used() {
        assert_eq!(Solution::max_value(vec![1, 2, 4, 8], 2), 15);
    }

    #[test]
    fn all_same_values() {
        assert_eq!(Solution::max_value(vec![3, 3, 3, 3], 2), 0);
    }

    #[test]
    fn k_one_non_adjacent_pick() {
        assert_eq!(Solution::max_value(vec![1, 2, 4], 1), 6);
    }
}
