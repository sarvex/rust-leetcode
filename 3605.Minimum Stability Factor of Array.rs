impl Solution {
    /// Binary search on answer with sparse table for GCD queries.
    ///
    /// # Intuition
    /// A stable subarray has GCD ≥ 2. Changing an element to 1 breaks all stable
    /// subarrays containing it. We binary search on the minimum achievable max length
    /// and greedily check if it's achievable.
    ///
    /// # Approach
    /// 1. Build sparse table for O(1) GCD range queries
    /// 2. For each j, find min start where GCD([start, j]) ≥ 2
    /// 3. Binary search: for each candidate L, greedily place blockers
    /// 4. Place blocker at rightmost position when segment exceeds L
    ///
    /// # Complexity
    /// - Time: O(n log n log n) - sparse table + binary search with linear checks
    /// - Space: O(n log n) for sparse table
    pub fn min_stable(nums: Vec<i32>, max_c: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        // Precompute log2 values
        let mut lg = vec![0usize; n + 1];
        for i in 2..=n {
            lg[i] = lg[i / 2] + 1;
        }
        let k_max = lg[n] + 1;

        // Build sparse table for GCD queries
        let mut st = vec![vec![0i64; n]; k_max];
        for i in 0..n {
            st[0][i] = nums[i] as i64;
        }
        for k in 1..k_max {
            for i in 0..n {
                let j = i + (1 << (k - 1));
                st[k][i] = if j < n {
                    Self::gcd(st[k - 1][i], st[k - 1][j])
                } else {
                    st[k - 1][i]
                };
            }
        }

        // GCD range query [l, r]
        let query = |l: usize, r: usize| -> i64 {
            let k = lg[r - l + 1];
            Self::gcd(st[k][l], st[k][r + 1 - (1 << k)])
        };

        // For each j, find minimum start index where GCD([start, j]) >= 2
        let mut start = vec![n; n];
        for j in 0..n {
            if nums[j] < 2 {
                continue;
            }
            if query(0, j) >= 2 {
                start[j] = 0;
            } else {
                let (mut lo, mut hi) = (0, j);
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if query(mid, j) >= 2 {
                        hi = mid;
                    } else {
                        lo = mid + 1;
                    }
                }
                start[j] = lo;
            }
        }

        // Check if max stable length <= l is achievable with <= max_c changes
        let can_achieve = |l: usize| -> bool {
            if l == 0 {
                return nums.iter().filter(|&&x| x >= 2).count() as i32 <= max_c;
            }

            let (mut changes, mut last_blocker) = (0i32, -1i64);

            for j in 0..n {
                if start[j] == n {
                    continue;
                }
                let eff_start = start[j].max((last_blocker + 1) as usize);
                if eff_start <= j && j - eff_start + 1 > l {
                    changes += 1;
                    if changes > max_c {
                        return false;
                    }
                    last_blocker = j as i64;
                }
            }
            true
        };

        // Binary search for minimum achievable L
        let (mut lo, mut hi) = (0, n);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if can_achieve(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo as i32
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_stable(vec![3, 5, 10], 1), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_stable(vec![2, 6, 8], 2), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_stable(vec![2, 4, 9, 6], 1), 2);
    }

    #[test]
    fn test_all_coprime() {
        // [2, 3, 5, 7] - all adjacent pairs have GCD 1
        assert_eq!(Solution::min_stable(vec![2, 3, 5, 7], 0), 1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_stable(vec![5], 0), 1);
        assert_eq!(Solution::min_stable(vec![5], 1), 0);
        assert_eq!(Solution::min_stable(vec![1], 0), 0);
    }

    #[test]
    fn test_all_same() {
        // [4, 4, 4, 4] - entire array is stable
        assert_eq!(Solution::min_stable(vec![4, 4, 4, 4], 0), 4);
        assert_eq!(Solution::min_stable(vec![4, 4, 4, 4], 1), 2);
        assert_eq!(Solution::min_stable(vec![4, 4, 4, 4], 2), 1);
    }

    #[test]
    fn test_zero_changes() {
        assert_eq!(Solution::min_stable(vec![2, 3, 4], 0), 1);
    }

    #[test]
    fn test_break_all() {
        // Need to change all elements >= 2 to get stability factor 0
        assert_eq!(Solution::min_stable(vec![2, 4], 2), 0);
    }
}
