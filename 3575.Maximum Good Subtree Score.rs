impl Solution {
    /// Maximum good subtree score using bitmask DP with subset enumeration.
    ///
    /// # Intuition
    /// A good subset requires each digit 0-9 to appear at most once across all
    /// selected node values. We use bitmask DP where a 10-bit mask tracks which
    /// digits are used. For each subtree, compute the maximum sum achievable
    /// for each possible digit configuration.
    ///
    /// # Approach
    /// 1. Build adjacency list from parent array and compute digit masks for each value
    /// 2. DFS computes dp[mask] = max sum achievable using exactly the digits in mask
    /// 3. Merge child DPs using subset enumeration (only combine non-overlapping masks)
    /// 4. For each subtree, add the maximum score (best mask) to the result
    ///
    /// # Complexity
    /// - Time: O(n * 3^10) where 3^10 ≈ 59049 for subset enumeration per merge
    /// - Space: O(n * 1024) for DP tables during recursion
    pub fn good_subtree_sum(vals: Vec<i32>, par: Vec<i32>) -> i32 {
        let n = vals.len();

        let mut children = vec![vec![]; n];
        par.iter()
            .enumerate()
            .skip(1)
            .for_each(|(i, p)| children[*p as usize].push(i));

        let digit_mask = |mut val: i64| -> Option<usize> {
            let mut mask = 0usize;
            while val > 0 {
                let digit = (val % 10) as usize;
                if (mask >> digit) & 1 == 1 {
                    return None;
                }
                mask |= 1 << digit;
                val /= 10;
            }
            Some(mask)
        };

        let masks: Vec<Option<usize>> = vals.iter().map(|&v| digit_mask(v as i64)).collect();

        let mut result = 0i64;

        fn dfs(
            u: usize,
            children: &[Vec<usize>],
            vals: &[i32],
            masks: &[Option<usize>],
            result: &mut i64,
        ) -> Vec<i64> {
            const NEG_INF: i64 = i64::MIN / 2;
            const MOD: i64 = 1_000_000_007;

            let mut dp = vec![NEG_INF; 1024];
            dp[0] = 0;

            if let Some(mask) = masks[u] {
                dp[mask] = dp[mask].max(vals[u] as i64);
            }

            let dp = children[u].iter().fold(dp, |dp, &child| {
                let child_dp = dfs(child, children, vals, masks, result);
                let mut new_dp = vec![NEG_INF; 1024];

                for mask in 0..1024 {
                    if dp[mask] == NEG_INF {
                        continue;
                    }
                    let complement = 1023 ^ mask;
                    let mut submask = complement;
                    loop {
                        if child_dp[submask] != NEG_INF {
                            let combined = mask | submask;
                            new_dp[combined] = new_dp[combined].max(dp[mask] + child_dp[submask]);
                        }
                        if submask == 0 {
                            break;
                        }
                        submask = (submask - 1) & complement;
                    }
                }
                new_dp
            });

            let max_score = *dp.iter().max().unwrap();
            *result = (*result + max_score) % MOD;

            dp
        }

        dfs(0, &children, &vals, &masks, &mut result);

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::good_subtree_sum(vec![2, 3], vec![-1, 0]), 8);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::good_subtree_sum(vec![1, 5, 2], vec![-1, 0, 0]),
            15
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::good_subtree_sum(vec![34, 1, 2], vec![-1, 0, 1]),
            42
        );
    }

    #[test]
    fn test_example_4() {
        assert_eq!(
            Solution::good_subtree_sum(vec![3, 22, 5], vec![-1, 0, 1]),
            18
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::good_subtree_sum(vec![123], vec![-1]), 123);
    }

    #[test]
    fn test_repeated_digit_value() {
        assert_eq!(Solution::good_subtree_sum(vec![11], vec![-1]), 0);
    }

    #[test]
    fn test_all_same_digit() {
        // All nodes have same digit, can only pick one per subtree
        // Subtrees: {0,1,2}->1, {1}->1, {2}->1, sum=3
        assert_eq!(Solution::good_subtree_sum(vec![1, 1, 1], vec![-1, 0, 0]), 3);
    }

    #[test]
    fn test_large_values() {
        // 123456789 uses digits 1-9, can't combine with node 1 (digit 1)
        // Subtree {0,1}: max=123456789, Subtree {1}: max=1
        assert_eq!(
            Solution::good_subtree_sum(vec![123456789, 1], vec![-1, 0]),
            123456789 + 1
        );
    }
}
