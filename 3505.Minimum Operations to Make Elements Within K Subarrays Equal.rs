use std::collections::BTreeMap;

impl Solution {
    /// Uses DP with sliding window median to find minimum operations for k non-overlapping subarrays.
    ///
    /// # Intuition
    /// To make all elements in a subarray equal with minimum operations, we should make them
    /// equal to the median. For each position, we compute the cost to equalize its x-sized
    /// subarray, then use DP to select k non-overlapping subarrays with minimum total cost.
    ///
    /// # Approach
    /// 1. Use two BTreeMaps (lo/hi) to maintain a sliding window that tracks the median
    /// 2. Compute cost[i] = minimum operations to equalize subarray [i-x+1..i]
    /// 3. DP transition: dp[i][j] = min cost to have j subarrays ending at or before index i
    ///    - Either skip position i: dp[i-1][j]
    ///    - Or use subarray ending at i: dp[i-x][j-1] + cost[i]
    ///
    /// # Complexity
    /// - Time: O(n * k + n * log(x)) for DP and sliding window operations
    /// - Space: O(n * k) for DP table
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let x = x as usize;
        let k = k as usize;
        let nums: Vec<i64> = nums.iter().map(|&v| v as i64).collect();

        let mut lo: BTreeMap<i64, i64> = BTreeMap::new();
        let mut hi: BTreeMap<i64, i64> = BTreeMap::new();
        let mut lo_size: i64 = 0;
        let mut lo_sum: i64 = 0;
        let mut hi_sum: i64 = 0;

        let add = |v: i64,
                   lo: &mut BTreeMap<i64, i64>,
                   hi: &mut BTreeMap<i64, i64>,
                   lo_size: &mut i64,
                   lo_sum: &mut i64,
                   hi_sum: &mut i64| {
            if lo.is_empty() || v <= *lo.keys().next_back().unwrap() {
                *lo.entry(v).or_insert(0) += 1;
                *lo_size += 1;
                *lo_sum += v;
            } else {
                *hi.entry(v).or_insert(0) += 1;
                *hi_sum += v;
            }
        };

        let remove = |v: i64,
                      lo: &mut BTreeMap<i64, i64>,
                      hi: &mut BTreeMap<i64, i64>,
                      lo_size: &mut i64,
                      lo_sum: &mut i64,
                      hi_sum: &mut i64| {
            if let Some(cnt) = lo.get_mut(&v) {
                *cnt -= 1;
                if *cnt == 0 {
                    lo.remove(&v);
                }
                *lo_size -= 1;
                *lo_sum -= v;
            } else if let Some(cnt) = hi.get_mut(&v) {
                *cnt -= 1;
                if *cnt == 0 {
                    hi.remove(&v);
                }
                *hi_sum -= v;
            }
        };

        let balance = |lo: &mut BTreeMap<i64, i64>,
                       hi: &mut BTreeMap<i64, i64>,
                       lo_size: &mut i64,
                       lo_sum: &mut i64,
                       hi_sum: &mut i64,
                       target: i64| {
            while *lo_size < target && !hi.is_empty() {
                let &v = hi.keys().next().unwrap();
                *hi.get_mut(&v).unwrap() -= 1;
                if hi[&v] == 0 {
                    hi.remove(&v);
                }
                *hi_sum -= v;
                *lo.entry(v).or_insert(0) += 1;
                *lo_size += 1;
                *lo_sum += v;
            }
            while *lo_size > target {
                let &v = lo.keys().next_back().unwrap();
                *lo.get_mut(&v).unwrap() -= 1;
                if lo[&v] == 0 {
                    lo.remove(&v);
                }
                *lo_size -= 1;
                *lo_sum -= v;
                *hi.entry(v).or_insert(0) += 1;
                *hi_sum += v;
            }
        };

        let target_lo = ((x + 1) / 2) as i64;
        let mut cost = vec![0i64; n];

        for i in 0..n {
            add(
                nums[i],
                &mut lo,
                &mut hi,
                &mut lo_size,
                &mut lo_sum,
                &mut hi_sum,
            );
            if i >= x {
                remove(
                    nums[i - x],
                    &mut lo,
                    &mut hi,
                    &mut lo_size,
                    &mut lo_sum,
                    &mut hi_sum,
                );
            }
            balance(
                &mut lo,
                &mut hi,
                &mut lo_size,
                &mut lo_sum,
                &mut hi_sum,
                target_lo,
            );
            if i >= x - 1 {
                let median = *lo.keys().next_back().unwrap();
                cost[i] = median * lo_size - lo_sum + hi_sum - median * (x as i64 - lo_size);
            }
        }

        let mut dp: Vec<Vec<i64>> = (0..=n)
            .map(|_| {
                let mut row = vec![i64::MAX / 2; k + 1];
                row[0] = 0;
                row
            })
            .collect();

        for i in 1..=n {
            for j in 1..=k {
                dp[i][j] = dp[i - 1][j];
                if i >= x {
                    dp[i][j] = dp[i][j].min(dp[i - x][j - 1] + cost[i - 1]);
                }
            }
        }

        dp[n][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2),
            8
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2), 3);
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1, 1], 2, 2), 0);
    }

    #[test]
    fn test_single_subarray() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3], 3, 1), 2);
    }
}
