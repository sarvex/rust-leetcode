use std::collections::HashMap;

impl Solution {
    /// Counts length-5 subsequences whose middle element is the unique mode.
    ///
    /// # Intuition
    /// For each position i as the middle, pick 2 elements from each side.
    /// The middle value c is a unique mode when it appears more than any other
    /// value in the subsequence. We enumerate by c's frequency (2–5) and use
    /// inclusion-exclusion for the frequency-2 case.
    ///
    /// # Approach
    /// 1. Coordinate-compress values; maintain left/right frequency arrays.
    /// 2. For each middle position, compute contributions for c appearing 3–5
    ///    times (always valid) and 2 times (valid only if the other 3 elements
    ///    are mutually distinct — subtract invalid arrangements).
    /// 3. Accumulate modular result.
    ///
    /// # Complexity
    /// - Time: O(n × m) where m is the number of unique values
    /// - Space: O(m) for frequency arrays
    pub fn subsequences_with_middle_mode(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();

        let mut sorted_unique = nums.clone();
        sorted_unique.sort_unstable();
        sorted_unique.dedup();
        let val_to_idx: HashMap<i32, usize> = sorted_unique
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect();
        let nums: Vec<usize> = nums.iter().map(|v| val_to_idx[v]).collect();
        let m = sorted_unique.len();

        let mut right: Vec<i64> = nums.iter().fold(vec![0; m], |mut acc, &v| {
            acc[v] += 1;
            acc
        });

        let mut left: Vec<i64> = vec![0; m];
        let comb2 = |x: i64| -> i64 { if x < 2 { 0 } else { x * (x - 1) / 2 } };

        let mut result: i64 = 0;

        for i in 0..n {
            let c = nums[i];
            right[c] -= 1;

            let left_c = left[c];
            let right_c = right[c];
            let l_total = i as i64;
            let r_total = (n - i - 1) as i64;
            let l_other = l_total - left_c;
            let r_other = r_total - right_c;

            result += comb2(left_c) * comb2(right_c);
            result += comb2(left_c) * right_c * r_other;
            result += left_c * l_other * comb2(right_c);
            result += comb2(left_c) * comb2(r_other);
            result += left_c * l_other * right_c * r_other;
            result += comb2(l_other) * comb2(right_c);

            let (sum_left_sq, sum_right_sq, sum_lr_r_other, sum_lr_l_other) = (0..m)
                .filter(|&v| v != c)
                .fold((0i64, 0i64, 0i64, 0i64), |(lsq, rsq, lr_r, lr_l), v| {
                    (
                        lsq + comb2(left[v]),
                        rsq + comb2(right[v]),
                        lr_r + left[v] * right[v] * (r_other - right[v]),
                        lr_l + left[v] * right[v] * (l_other - left[v]),
                    )
                });

            let total_1_0 = left_c * l_other * comb2(r_other);
            let invalid_1_0 = left_c * l_other * sum_right_sq + left_c * sum_lr_r_other;
            result += total_1_0 - invalid_1_0;

            let total_0_1 = comb2(l_other) * right_c * r_other;
            let invalid_0_1 = right_c * r_other * sum_left_sq + right_c * sum_lr_l_other;
            result += total_0_1 - invalid_0_1;

            result = result.rem_euclid(MOD);

            left[c] += 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_same_elements_counted_correctly() {
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![1, 1, 1, 1, 1, 1]),
            6
        );
    }

    #[test]
    fn mixed_duplicates_yield_valid_subsequences() {
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![1, 2, 2, 3, 3, 4]),
            4
        );
    }

    #[test]
    fn all_distinct_has_no_unique_mode() {
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            0
        );
    }

    #[test]
    fn exactly_five_identical_elements() {
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![1, 1, 1, 1, 1]),
            1
        );
    }

    #[test]
    fn five_distinct_has_no_valid_subsequence() {
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![1, 2, 3, 4, 5]),
            0
        );
    }
}
