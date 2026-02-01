impl Solution {
    /// Counts permutations whose prefix inversion counts match all requirements.
    ///
    /// # Intuition
    /// Represent a permutation by its inversion sequence `inv[i]`, the number of previous elements
    /// greater than `perm[i]`. The inversion count of the prefix ending at `i` is the sum of
    /// `inv[0..=i]`, so each requirement fixes a prefix sum of this sequence.
    ///
    /// # Approach
    /// Use dynamic programming over positions and current prefix inversion sum. Let `dp[s]` be the
    /// number of valid inversion sequences for the first `i` positions with sum `s`. For position
    /// `i`, the value `inv[i]` can be any integer in `0..=i`, so:
    /// `next[s] = sum_{x=0..min(i, s)} dp[s - x]`. Compute this in O(R) per position with a sliding
    /// window, where `R` is the required inversion count at `n - 1`. If a requirement exists for
    /// index `i`, keep only the matching sum.
    ///
    /// # Complexity
    /// - Time: O(n * R), with `R <= 400`
    /// - Space: O(R)
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let mut required = vec![None; n];

        for pair in requirements {
            if pair.len() != 2 {
                continue;
            }
            let end = pair[0] as usize;
            let count = pair[1] as usize;
            if end >= n {
                return 0;
            }
            if let Some(existing) = required[end] {
                if existing != count {
                    return 0;
                }
            } else {
                required[end] = Some(count);
            }
        }

        let total_required = match required[n - 1] {
            Some(count) => count,
            None => return 0,
        };

        let mut last_required = 0usize;
        let mut has_last = false;
        for (i, &count) in required.iter().enumerate() {
            if let Some(count) = count {
                let max_possible = i * (i + 1) / 2;
                if count > max_possible || count > total_required {
                    return 0;
                }
                if has_last && count < last_required {
                    return 0;
                }
                last_required = count;
                has_last = true;
            }
        }

        let mut dp = vec![0i64; total_required + 1];
        dp[0] = 1;

        for i in 0..n {
            let mut next = vec![0i64; total_required + 1];
            let mut window_sum = 0i64;

            for s in 0..=total_required {
                window_sum += dp[s];
                if window_sum >= MOD {
                    window_sum -= MOD;
                }
                if s > i {
                    window_sum -= dp[s - i - 1];
                    if window_sum < 0 {
                        window_sum += MOD;
                    }
                }
                next[s] = window_sum;
            }

            if let Some(required_count) = required[i] {
                let keep = if required_count <= total_required {
                    next[required_count]
                } else {
                    0
                };
                next.fill(0);
                if required_count <= total_required {
                    next[required_count] = keep;
                }
            }

            dp = next;
        }

        dp[total_required] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let requirements = vec![vec![2, 2], vec![0, 0]];
        assert_eq!(Solution::number_of_permutations(3, requirements), 2);
    }

    #[test]
    fn test_example_2() {
        let requirements = vec![vec![2, 2], vec![1, 1], vec![0, 0]];
        assert_eq!(Solution::number_of_permutations(3, requirements), 1);
    }

    #[test]
    fn test_example_3() {
        let requirements = vec![vec![0, 0], vec![1, 0]];
        assert_eq!(Solution::number_of_permutations(2, requirements), 1);
    }

    #[test]
    fn test_only_total_requirement() {
        let requirements = vec![vec![2, 1]];
        assert_eq!(Solution::number_of_permutations(3, requirements), 2);
    }

    #[test]
    fn test_impossible_requirement() {
        let requirements = vec![vec![0, 1], vec![2, 1]];
        assert_eq!(Solution::number_of_permutations(3, requirements), 0);
    }
}
