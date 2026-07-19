impl Solution {
    /// Count gcd-pair frequencies with an in-place sieve and answer queries via partition_point.
    ///
    /// # Intuition
    /// Reuse a single array through three in-place passes: first accumulate divisor counts
    /// (how many elements are divisible by `d`), then convert to pair counts using
    /// inclusion-exclusion, then build a prefix sum — all without extra allocations.
    ///
    /// # Approach
    /// 1. `cnt[v]` = frequency of `v` in `nums`.
    /// 2. Forward sieve: for each `d`, add `cnt[2d] + cnt[3d] + ...` into `cnt[d]`, so
    ///    `cnt[d]` = count of elements divisible by `d`.
    /// 3. Convert to C(cnt[d], 2) in-place.
    /// 4. Reverse sieve (inclusion-exclusion): subtract `cnt[2d] + cnt[3d] + ...` from
    ///    `cnt[d]`, so `cnt[d]` = pairs with gcd exactly `d`.
    /// 5. Prefix-sum `cnt` so `cnt[d]` = number of gcd-pairs with gcd ≤ `d`.
    /// 6. Answer each query with `partition_point` on the prefix array.
    ///
    /// # Complexity
    /// - Time: O(M log M + Q log M), where M = max(nums)
    /// - Space: O(M)
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let m = *nums.iter().max().unwrap() as usize;

        // Pass 1: frequency table.
        let mut cnt = vec![0_i64; m + 1];
        for &num in &nums {
            cnt[num as usize] += 1;
        }

        // Pass 2: forward sieve — cnt[d] becomes count of elements divisible by d.
        for d in 1..=m {
            let mut j = d * 2;
            while j <= m {
                cnt[d] += cnt[j];
                j += d;
            }
        }

        // Pass 3: convert to pair counts C(cnt[d], 2) in-place.
        for d in 1..=m {
            cnt[d] = cnt[d] * (cnt[d] - 1) / 2;
        }

        // Pass 4: reverse sieve (inclusion-exclusion) — cnt[d] becomes pairs with gcd exactly d.
        for d in (1..=m).rev() {
            let mut j = d * 2;
            while j <= m {
                cnt[d] -= cnt[j];
                j += d;
            }
        }

        // Pass 5: prefix sum — cnt[d] becomes pairs with gcd <= d.
        for d in 1..=m {
            cnt[d] += cnt[d - 1];
        }

        // Answer queries: find the smallest d where the cumulative count exceeds q.
        queries
            .iter()
            .map(|&q| cnt.partition_point(|&x| x <= q) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::gcd_values(vec![2, 3, 4], vec![0, 2, 2]),
            vec![1, 2, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::gcd_values(vec![4, 4, 2, 1], vec![5, 3, 1, 0]),
            vec![4, 2, 1, 1]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::gcd_values(vec![2, 2], vec![0, 0]), vec![2, 2]);
    }

    #[test]
    fn all_same_values() {
        assert_eq!(
            Solution::gcd_values(vec![6, 6, 6], vec![0, 1, 2]),
            vec![6, 6, 6]
        );
    }
}
