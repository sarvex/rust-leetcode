impl Solution {
    /// Counts stable subsequences using parity-based dynamic programming.
    ///
    /// # Intuition
    /// A subsequence becomes unstable when it has 3+ consecutive elements of the same
    /// parity. We track subsequences by their "ending state" - the parity of the last
    /// element and how many consecutive elements of that parity appear at the end.
    ///
    /// # Approach
    /// Use four scalar variables:
    /// - e2 = subsequences ending with 2 consecutive evens
    /// - e1 = subsequences ending with 1 consecutive even
    /// - o1 = subsequences ending with 1 consecutive odd
    /// - o2 = subsequences ending with 2 consecutive odds
    ///
    /// For each element, update only the relevant parity states using tuple assignment.
    ///
    /// # Complexity
    /// - Time: O(n) - single pass through array
    /// - Space: O(1) - only 4 state variables
    pub fn count_stable_subsequences(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let (e2, e1, o1, o2) =
            nums.iter()
                .fold((0i64, 0i64, 0i64, 0i64), |(e2, e1, o1, o2), &num| {
                    if num & 1 == 0 {
                        ((e2 + e1) % MOD, (1 + e1 + o1 + o2) % MOD, o1, o2)
                    } else {
                        (e2, e1, (1 + e2 + e1 + o1) % MOD, (o1 + o2) % MOD)
                    }
                });

        ((e2 + e1 + o1 + o2) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_consecutive_odds() {
        assert_eq!(Solution::count_stable_subsequences(vec![1, 3, 5]), 6);
    }

    #[test]
    fn test_mixed_parities_one_unstable() {
        assert_eq!(Solution::count_stable_subsequences(vec![2, 3, 4, 2]), 14);
    }

    #[test]
    fn test_single_odd_element() {
        assert_eq!(Solution::count_stable_subsequences(vec![1]), 1);
    }

    #[test]
    fn test_single_even_element() {
        assert_eq!(Solution::count_stable_subsequences(vec![2]), 1);
    }

    #[test]
    fn test_two_evens_within_limit() {
        assert_eq!(Solution::count_stable_subsequences(vec![2, 4]), 3);
    }

    #[test]
    fn test_two_different_parities() {
        assert_eq!(Solution::count_stable_subsequences(vec![1, 2]), 3);
    }

    #[test]
    fn test_perfectly_alternating() {
        assert_eq!(Solution::count_stable_subsequences(vec![1, 2, 3, 4]), 15);
    }

    #[test]
    fn test_four_consecutive_odds() {
        assert_eq!(Solution::count_stable_subsequences(vec![1, 3, 5, 7]), 10);
    }
}
