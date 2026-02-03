impl Solution {
    /// Dynamic programming over reset order by growth rate.
    ///
    /// # Intuition
    /// Resetting index `i` at second `s` removes `nums1[i] + nums2[i] * s` from the final sum, so
    /// larger `nums2[i]` should get larger `s` to maximize the removed value.
    ///
    /// # Approach
    /// Sort pairs by `nums2` ascending. Let `dp[k]` be the maximum removable sum using exactly `k`
    /// indices among processed items, assuming the chosen items are reset in that sorted order at
    /// seconds `1..k`. For each item `(a, b)`, update `dp[k] = max(dp[k], dp[k-1] + a + b * k)`.
    /// For each time `t`, the minimum achievable sum after `t` seconds is
    /// `sum(nums1) + sum(nums2) * t - dp[t]`. Return the smallest `t` where this is `<= x`.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n)
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let n = nums1.len();
        let total1: i64 = nums1.iter().map(|&value| value as i64).sum();
        let total2: i64 = nums2.iter().map(|&value| value as i64).sum();

        if total1 <= x as i64 {
            return 0;
        }

        let mut items: Vec<(i64, i64)> = nums1
            .into_iter()
            .map(|value| value as i64)
            .zip(nums2.into_iter().map(|value| value as i64))
            .collect();
        items.sort_by(|left, right| left.1.cmp(&right.1));

        let mut dp = vec![i64::MIN / 4; n + 1];
        dp[0] = 0;

        for (base, growth) in items {
            for k in (1..=n).rev() {
                let candidate = dp[k - 1] + base + growth * k as i64;
                if candidate > dp[k] {
                    dp[k] = candidate;
                }
            }
        }

        for t in 1..=n {
            let min_sum = total1 + total2 * t as i64 - dp[t];
            if min_sum <= x as i64 {
                return t as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![1, 2, 3];
        assert_eq!(Solution::minimum_time(nums1, nums2, 4), 3);
    }

    #[test]
    fn example_two() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![3, 3, 3];
        assert_eq!(Solution::minimum_time(nums1, nums2, 4), -1);
    }

    #[test]
    fn already_within_limit() {
        let nums1 = vec![2, 1];
        let nums2 = vec![3, 4];
        assert_eq!(Solution::minimum_time(nums1, nums2, 3), 0);
    }

    #[test]
    fn single_reset_is_enough() {
        let nums1 = vec![5, 1];
        let nums2 = vec![1, 0];
        assert_eq!(Solution::minimum_time(nums1, nums2, 3), 1);
    }
}
