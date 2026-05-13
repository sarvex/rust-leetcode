impl Solution {
    /// Difference array over candidate target sums.
    ///
    /// # Intuition
    /// For each pair `(a, b) = (nums[i], nums[n-1-i])` with `a <= b`, the number
    /// of moves needed to make their sum equal `T` depends only on where `T`
    /// falls relative to a few breakpoints:
    ///
    /// - `T == a + b`                                  -> 0 moves
    /// - `T in [1 + a, a + b)` or `T in (a + b, b + limit]` -> 1 move
    ///   (replace exactly one of the two elements)
    /// - `T in [2, 1 + a)` or `T in (b + limit, 2 * limit]` -> 2 moves
    ///   (replace both elements)
    ///
    /// So each pair contributes a piecewise-constant cost function over
    /// `T in [2, 2 * limit]`. Summing all those step functions and taking the
    /// minimum yields the answer. We build the sum efficiently via a difference
    /// array indexed by `T`.
    ///
    /// # Approach
    /// Initialize `diff` with `2 * (n / 2)` at index `2` (every pair starts at
    /// cost 2). For each pair `(lo, hi)` with `lo <= hi`:
    /// - subtract 1 on `[1 + lo, lo + hi]`           (drop from 2 -> 1)
    /// - subtract 1 on `[lo + hi, lo + hi]`           (drop from 1 -> 0 at the exact sum)
    /// - add 1 on `(hi + limit, 2 * limit]`           (rise back from 1 -> 2)
    ///
    /// Sweep the prefix sum across `T = 2..=2 * limit` and track the minimum.
    ///
    /// # Complexity
    /// - Time: O(n + limit)
    /// - Space: O(limit)
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let limit = limit as usize;

        // diff covers T in [0, 2 * limit + 1]; index 2 * limit + 1 is a sentinel.
        let mut diff = vec![0i32; 2 * limit + 2];

        // Every pair starts costing 2 moves before any range adjustments.
        diff[2] += 2 * (n as i32 / 2);
        diff[2 * limit + 1] -= 2 * (n as i32 / 2);

        for i in 0..n / 2 {
            let a = nums[i] as usize;
            let b = nums[n - 1 - i] as usize;
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };

            // Drop cost from 2 -> 1 over [1 + lo, hi + limit].
            diff[1 + lo] -= 1;
            diff[hi + limit + 1] += 1;

            // Drop cost from 1 -> 0 exactly at T = lo + hi.
            diff[lo + hi] -= 1;
            diff[lo + hi + 1] += 1;
        }

        diff.iter()
            .take(2 * limit + 1)
            .skip(2)
            .scan(0i32, |running, &delta| {
                *running += delta;
                Some(*running)
            })
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_moves(vec![1, 2, 4, 3], 4), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_moves(vec![1, 2, 2, 1], 2), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_moves(vec![1, 2, 1, 2], 2), 0);
    }

    #[test]
    fn test_minimum_pair_already_complementary() {
        assert_eq!(Solution::min_moves(vec![1, 1], 1), 0);
    }

    #[test]
    fn test_minimum_pair_needs_one_move() {
        // Pair (1, 2): possible sums with one move at limit=2 are 2,3,4. Already 3 -> 0 moves.
        assert_eq!(Solution::min_moves(vec![1, 2], 2), 0);
    }

    #[test]
    fn test_all_equal_already_complementary() {
        assert_eq!(Solution::min_moves(vec![3, 3, 3, 3, 3, 3], 5), 0);
    }

    #[test]
    fn test_extreme_limits() {
        // Pairs from outside in: (nums[0], nums[3]) = (1, 1) sum = 2,
        // (nums[1], nums[2]) = (limit, limit) sum = 2 * limit.
        // No single T is reachable by both pairs without moves, and aligning them
        // requires replacing both elements of one pair -> 2 moves.
        assert_eq!(
            Solution::min_moves(vec![1, 100_000, 100_000, 1], 100_000),
            2
        );
    }

    #[test]
    fn test_two_pairs_one_move_each_target() {
        // nums = [1, 4, 5, 8], limit = 8
        // Pair (1, 8): sum = 9, range [2, 16].
        // Pair (4, 5): sum = 9, range [5, 13].
        // T = 9 is achievable with 0 moves for both pairs.
        assert_eq!(Solution::min_moves(vec![1, 4, 5, 8], 8), 0);
    }

    #[test]
    fn test_forces_two_moves_some_pair() {
        // nums = [1, 1, 1, 10], limit = 10
        // Pair0 (1, 10): sum = 11. Pair1 (1, 1): sum = 2.
        // For Pair1 (1,1), T can be in [2, 11] with at most 1 move (replace one 1 with up to 10).
        // For Pair0 (1,10), T in [2, 20] with at most 1 move.
        // Both can hit T = 11 with: Pair0 -> 0 moves, Pair1 -> 1 move. Total = 1.
        assert_eq!(Solution::min_moves(vec![1, 1, 1, 10], 10), 1);
    }

    #[test]
    fn test_large_input_performance() {
        // Stress: n = 10^5, limit = 10^5, alternating values.
        let n = 100_000usize;
        let limit = 100_000i32;
        let nums: Vec<i32> = (0..n).map(|i| if i % 2 == 0 { 1 } else { limit }).collect();
        // Every pair is (1, limit) (or (limit, 1)) -> sum = 1 + limit, already complementary.
        assert_eq!(Solution::min_moves(nums, limit), 0);
    }
}
