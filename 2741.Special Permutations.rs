impl Solution {
    /// Count permutations where adjacent elements are divisible.
    ///
    /// # Intuition
    /// Bitmask DP: track which elements have been placed and which was last.
    /// Transition from state `(mask, k)` to `(mask | (1<<j), j)` when
    /// `nums[j] % nums[k] == 0` or vice versa.
    ///
    /// # Approach
    /// 1. Let `f[mask][j]` = number of permutations using elements in `mask`
    ///    ending with element `j`.
    /// 2. Base case: single-element masks have value 1.
    /// 3. Enumerate all masks and transitions.
    /// 4. Sum `f[(1<<n)-1][j]` over all `j`.
    ///
    /// # Complexity
    /// - Time: O(2^n · n²)
    /// - Space: O(2^n · n)
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = nums.len();
        let m = 1 << n;
        let mut f = vec![vec![0i32; n]; m];

        for mask in 1..m {
            for j in 0..n {
                if (mask >> j) & 1 == 0 {
                    continue;
                }
                let prev_mask = mask ^ (1 << j);
                if prev_mask == 0 {
                    f[mask][j] = 1;
                    continue;
                }
                f[mask][j] = (0..n)
                    .filter(|&k| {
                        (prev_mask >> k) & 1 == 1
                            && (nums[j] % nums[k] == 0 || nums[k] % nums[j] == 0)
                    })
                    .fold(f[mask][j], |acc, k| (acc + f[prev_mask][k]) % MOD);
            }
        }

        f[m - 1].iter().fold(0, |acc, &x| (acc + x) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_divisible_pairs() {
        assert_eq!(Solution::special_perm(vec![2, 3, 6]), 2);
    }

    #[test]
    fn all_ones() {
        assert_eq!(Solution::special_perm(vec![1, 4, 3]), 2);
    }
}
