impl Solution {
    /// Catalan number dynamic programming for counting unique BSTs.
    ///
    /// # Intuition
    /// The number of structurally unique BSTs with values [1..n] is the nth
    /// Catalan number. Choosing root `k` splits into `f[k-1]` left subtrees
    /// and `f[n-k]` right subtrees, yielding the recurrence
    /// `f[n] = Σ f[j] × f[n-j-1]`.
    ///
    /// # Approach
    /// Initialize `f[0] = 1` (empty tree). For each size `i` from 1 to n,
    /// sum all products `f[j] × f[i-j-1]` for j in 0..i.
    ///
    /// # Complexity
    /// - Time: O(n^2) — nested summation
    /// - Space: O(n) — DP array
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0i32; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in 0..i {
                dp[i] += dp[j] * dp[i - j - 1];
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_equals_three() {
        assert_eq!(Solution::num_trees(3), 5);
    }

    #[test]
    fn n_equals_one() {
        assert_eq!(Solution::num_trees(1), 1);
    }

    #[test]
    fn n_equals_four() {
        assert_eq!(Solution::num_trees(4), 14);
    }

    #[test]
    fn n_equals_five() {
        assert_eq!(Solution::num_trees(5), 42);
    }
}
