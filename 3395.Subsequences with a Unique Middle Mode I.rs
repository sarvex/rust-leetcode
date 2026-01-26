use std::collections::HashMap;

impl Solution {
    /// Counts subsequences of size 5 where middle element is the unique mode
    ///
    /// # Intuition
    /// For each position i as the middle of a 5-element subsequence, we pick 2
    /// elements from positions before i and 2 from positions after i. The middle
    /// element c is a unique mode if it appears more times than any other element.
    ///
    /// # Approach
    /// Count by frequency of middle element c in the subsequence:
    /// - freq >= 3: Always valid (other elements can't exceed c's count)
    /// - freq = 2: Valid only if all 3 other elements are distinct
    /// - freq = 1: Never valid (ties at best)
    ///
    /// For each middle position, maintain left/right counts and compute valid
    /// combinations using combinatorics with inclusion-exclusion for freq=2 cases.
    ///
    /// # Complexity
    /// - Time: O(n * m) where n is array length, m is unique values count
    /// - Space: O(m) for frequency arrays
    pub fn subsequences_with_middle_mode(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();

        // Coordinate compression for values up to 10^9
        let mut sorted_unique = nums.clone();
        sorted_unique.sort_unstable();
        sorted_unique.dedup();
        let val_to_idx: HashMap<i32, usize> = sorted_unique
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();
        let nums: Vec<usize> = nums.iter().map(|v| val_to_idx[v]).collect();
        let m = sorted_unique.len();

        // right[v] = count of value v in positions > current
        let mut right: Vec<i64> = vec![0; m];
        for &v in &nums {
            right[v] += 1;
        }

        // left[v] = count of value v in positions < current
        let mut left: Vec<i64> = vec![0; m];

        let comb2 = |x: i64| -> i64 {
            if x < 2 {
                0
            } else {
                x * (x - 1) / 2
            }
        };

        let mut result: i64 = 0;

        for i in 0..n {
            let c = nums[i];
            right[c] -= 1; // Current position moves out of right

            let left_c = left[c];
            let right_c = right[c];
            let l_total = i as i64;
            let r_total = (n - i - 1) as i64;
            let l_other = l_total - left_c;
            let r_other = r_total - right_c;

            // freq = 5: all elements are c
            result += comb2(left_c) * comb2(right_c);

            // freq = 4: 3 c's besides middle + 1 other
            result += comb2(left_c) * right_c * r_other;
            result += left_c * l_other * comb2(right_c);

            // freq = 3: 2 c's besides middle + 2 others
            result += comb2(left_c) * comb2(r_other);
            result += left_c * l_other * right_c * r_other;
            result += comb2(l_other) * comb2(right_c);

            // freq = 2: 1 c besides middle + 3 others (all must be distinct)
            // Compute sums for invalid case detection
            let mut sum_left_sq: i64 = 0;
            let mut sum_right_sq: i64 = 0;
            let mut sum_lr_r_other: i64 = 0;
            let mut sum_lr_l_other: i64 = 0;

            for v in 0..m {
                if v != c {
                    sum_left_sq += comb2(left[v]);
                    sum_right_sq += comb2(right[v]);
                    sum_lr_r_other += left[v] * right[v] * (r_other - right[v]);
                    sum_lr_l_other += left[v] * right[v] * (l_other - left[v]);
                }
            }

            // Case (l=1, r=0): 1 c from left, 1 non-c from left, 2 non-c from right
            let total_1_0 = left_c * l_other * comb2(r_other);
            // Invalid: 2 from right same value OR left matches one of right
            let invalid_1_0 = left_c * l_other * sum_right_sq + left_c * sum_lr_r_other;
            result += total_1_0 - invalid_1_0;

            // Case (l=0, r=1): 2 non-c from left, 1 c from right, 1 non-c from right
            let total_0_1 = comb2(l_other) * right_c * r_other;
            // Invalid: 2 from left same value OR right non-c matches one of left
            let invalid_0_1 = right_c * r_other * sum_left_sq + right_c * sum_lr_l_other;
            result += total_0_1 - invalid_0_1;

            result = result.rem_euclid(MOD);

            left[c] += 1; // Current position moves into left
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_same_elements() {
        // [1,1,1,1,1] is the only subsequence, can be formed 6 ways (C(6,5))
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![1, 1, 1, 1, 1, 1]),
            6
        );
    }

    #[test]
    fn test_mixed_elements() {
        // [1,2,2,3,4] and [1,2,3,3,4] have unique middle modes
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![1, 2, 2, 3, 3, 4]),
            4
        );
    }

    #[test]
    fn test_all_distinct() {
        // No subsequence of length 5 with distinct elements has unique middle mode
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            0
        );
    }

    #[test]
    fn test_minimum_length() {
        // Exactly 5 elements, all same
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![1, 1, 1, 1, 1]),
            1
        );
    }

    #[test]
    fn test_minimum_distinct() {
        // Exactly 5 distinct elements - no valid subsequence
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![1, 2, 3, 4, 5]),
            0
        );
    }
}
