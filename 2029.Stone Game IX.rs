impl Solution {
    /// Determines if Alice wins Stone Game IX using modular arithmetic analysis.
    ///
    /// # Intuition
    /// Only the remainder of each stone modulo 3 matters (0, 1, or 2). A stone with
    /// remainder 0 doesn't change the running sum mod 3 but effectively swaps whose
    /// "turn strategy" it is. We reduce the problem to counting stones by their mod-3
    /// class and analyzing the two possible opening moves Alice can make.
    ///
    /// # Approach
    /// 1. Count stones by remainder mod 3 into three buckets: `cnt[0]`, `cnt[1]`, `cnt[2]`.
    /// 2. If `cnt[0]` is even, the zeros don't disrupt the order. Alice wins if both
    ///    `cnt[1]` and `cnt[2]` are non-zero (she can always force a win by picking the
    ///    smaller group first and forcing Bob into a losing position).
    /// 3. If `cnt[0]` is odd, zeros swap the advantage once. Alice wins only if the
    ///    difference between `cnt[1]` and `cnt[2]` is greater than 2, giving her enough
    ///    margin to survive the swap.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut cnt = [0i32; 3];
        for s in &stones {
            cnt[(*s % 3) as usize] += 1;
        }

        if cnt[0] % 2 == 0 {
            cnt[1] > 0 && cnt[2] > 0
        } else {
            (cnt[1] - cnt[2]).abs() > 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::stone_game_ix(vec![2, 1]));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::stone_game_ix(vec![2]));
    }

    #[test]
    fn test_example_3() {
        assert!(!Solution::stone_game_ix(vec![5, 1, 2, 4, 3]));
    }

    #[test]
    fn test_all_zeros() {
        // All stones mod 3 == 0, Alice must pick a zero first making sum 0 mod 3 → she loses
        assert!(!Solution::stone_game_ix(vec![3, 6, 9]));
    }

    #[test]
    fn test_single_stone() {
        // Single stone with value 1: Alice picks it, sum = 1, not divisible by 3, no stones left → Bob wins
        assert!(!Solution::stone_game_ix(vec![1]));
    }

    #[test]
    fn test_large_difference() {
        // cnt[1] = 5, cnt[2] = 1, cnt[0] = 1 (odd) → |5-1| = 4 > 2 → Alice wins
        assert!(Solution::stone_game_ix(vec![1, 1, 1, 1, 1, 2, 3]));
    }

    #[test]
    fn test_even_zeros_both_nonzero() {
        // cnt[0] = 2, cnt[1] = 1, cnt[2] = 1 → even zeros, both nonzero → Alice wins
        assert!(Solution::stone_game_ix(vec![3, 6, 1, 2]));
    }
}
