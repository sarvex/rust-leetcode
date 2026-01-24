impl Solution {
    /// Counts stable subsequences using parity-based dynamic programming
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
        let (mut e2, mut e1, mut o1, mut o2) = (0i64, 0i64, 0i64, 0i64);

        for &num in &nums {
            if num & 1 == 0 {
                (e2, e1) = ((e2 + e1) % MOD, (1 + e1 + o1 + o2) % MOD);
            } else {
                (o1, o2) = ((1 + e2 + e1 + o1) % MOD, (o1 + o2) % MOD);
            }
        }

        ((e2 + e1 + o1 + o2) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn all_odd_numbers() {
        // [1,3,5] - subsequence [1,3,5] is unstable (3 consecutive odds)
        // Valid: [1], [3], [5], [1,3], [1,5], [3,5] = 6
        assert_eq!(Solution::count_stable_subsequences(vec![1, 3, 5]), 6);
    }

    #[test]
    fn mixed_parities() {
        // [2,3,4,2] - only [2,4,2] is unstable (3 consecutive evens)
        // Total subsequences: 2^4 - 1 = 15, minus 1 unstable = 14
        assert_eq!(Solution::count_stable_subsequences(vec![2, 3, 4, 2]), 14);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::count_stable_subsequences(vec![1]), 1);
        assert_eq!(Solution::count_stable_subsequences(vec![2]), 1);
    }

    #[test]
    fn two_elements_same_parity() {
        // [2,4] - both even, but only 2 consecutive is fine
        // Valid: [2], [4], [2,4] = 3
        assert_eq!(Solution::count_stable_subsequences(vec![2, 4]), 3);
    }

    #[test]
    fn two_elements_different_parity() {
        // [1,2] - all subsequences valid
        // Valid: [1], [2], [1,2] = 3
        assert_eq!(Solution::count_stable_subsequences(vec![1, 2]), 3);
    }

    #[test]
    fn alternating_parities() {
        // [1,2,3,4] - perfectly alternating, all 2^4-1=15 subsequences valid
        assert_eq!(Solution::count_stable_subsequences(vec![1, 2, 3, 4]), 15);
    }

    #[test]
    fn four_same_parity() {
        // [1,3,5,7] - many unstable subsequences with 3+ consecutive odds
        // Stable: single(4) + pairs(6) + some triples
        // Triples with gap: none possible (all odd)
        // So only singles and pairs = 4 + 6 = 10
        assert_eq!(Solution::count_stable_subsequences(vec![1, 3, 5, 7]), 10);
    }
}
