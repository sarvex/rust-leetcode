impl Solution {
    /// Maximum Bitwise AND After Increment Operations
    ///
    /// # Intuition
    /// Process bits high to low. Mask values to current window and use prefix sums
    /// on sorted array for O(1) cost queries.
    ///
    /// # Approach
    /// 1. Sort array once
    /// 2. For each bit, clamp high values and maintain sorted order via merge
    /// 3. Use prefix sums: cost = target * count - sum(elements below target)
    ///
    /// # Complexity
    /// - Time: O(n log n + n * B) amortized
    /// - Space: O(n)
    pub fn maximum_and(nums: Vec<i32>, k: i32, m: i32) -> i32 {
        let n = nums.len();
        let m = m as usize;
        let k = k as i64;

        if m == 1 {
            return nums.iter().max().unwrap() + k as i32;
        }

        let mut a: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        a.sort_unstable();

        let max_val = a[n - 1] + k;
        if max_val == 0 {
            return 0;
        }

        let max_bit = 64 - (max_val as u64).leading_zeros() as i32;
        let mut result = 0i64;

        let mut buf = vec![0i64; n];
        let mut psum = vec![0i64; n + 1];
        let mut psum_valid = false;

        for bit in (0..max_bit).rev() {
            let target = result | (1i64 << bit);
            let mask = result | ((1i64 << (bit + 1)) - 1);

            // Clamp values above mask
            let clamp_idx = a.partition_point(|&x| x <= mask);
            if clamp_idx < n {
                for v in &mut a[clamp_idx..] {
                    *v &= mask;
                }

                // Sort suffix and merge with prefix
                let suffix_len = n - clamp_idx;
                if suffix_len <= 64 || suffix_len * 4 <= n {
                    // Small suffix: sort and merge
                    a[clamp_idx..].sort_unstable();
                    Self::merge(&a[..clamp_idx], &a[clamp_idx..], &mut buf);
                    a.copy_from_slice(&buf);
                } else {
                    // Large suffix: full sort is simpler
                    a.sort_unstable();
                }
                psum_valid = false;
            }

            if !psum_valid {
                psum[0] = 0;
                for i in 0..n {
                    psum[i + 1] = psum[i] + a[i];
                }
                psum_valid = true;
            }

            // Cost for top m elements to reach target
            let start = n - m;
            let target_idx = start + a[start..].partition_point(|&x| x < target);
            let below_count = (target_idx - start) as i64;

            let cost = target * below_count - (psum[target_idx] - psum[start]);

            if cost <= k {
                result = target;
            }
        }

        result as i32
    }

    #[inline]
    fn merge(left: &[i64], right: &[i64], out: &mut [i64]) {
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                out[k] = left[i];
                i += 1;
            } else {
                out[k] = right[j];
                j += 1;
            }
            k += 1;
        }
        out[k..k + left.len() - i].copy_from_slice(&left[i..]);
        k += left.len() - i;
        out[k..k + right.len() - j].copy_from_slice(&right[j..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::maximum_and(vec![3, 1, 2], 8, 2), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::maximum_and(vec![1, 2, 8, 4], 7, 3), 4);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::maximum_and(vec![1, 1], 3, 2), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximum_and(vec![5], 3, 1), 8);
    }

    #[test]
    fn test_no_operations_needed() {
        assert_eq!(Solution::maximum_and(vec![7, 7, 7], 0, 2), 7);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::maximum_and(vec![4, 4, 4], 0, 3), 4);
    }

    #[test]
    fn test_large_values() {
        assert_eq!(
            Solution::maximum_and(vec![1000000000], 1000000000, 1),
            2000000000
        );
    }
}
