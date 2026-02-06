impl Solution {
    /// Minimum cost by comparing three strategies: only type 1+2, mixed type-3 with 1+2, or only type 3.
    ///
    /// # Intuition
    /// Type 3 satisfies one unit of both requirements. We can cover needs with (1) only type 1 and 2,
    /// (2) some type 3 plus the rest from type 1 and 2, or (3) only type 3 (max(need1, need2) items).
    ///
    /// # Approach
    /// Compute three candidate costs: (1) need1*cost1 + need2*cost2, (2) min(need1,need2)*costBoth +
    /// remainder from type 1 and 2, (3) max(need1,need2)*costBoth. Return the minimum. Use i64 to avoid overflow.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn minimum_cost(
        cost1: i32,
        cost2: i32,
        cost_both: i32,
        need1: i32,
        need2: i32,
    ) -> i64 {
        let (need1, need2) = (need1 as i64, need2 as i64);
        let (c1, c2, cb) = (cost1 as i64, cost2 as i64, cost_both as i64);
        let min_n = need1.min(need2);
        let max_n = need1.max(need2);

        let only_separate = need1 * c1 + need2 * c2;
        let mixed = min_n * cb + (need1 - min_n) * c1 + (need2 - min_n) * c2;
        let only_both = max_n * cb;

        only_separate.min(mixed).min(only_both)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::minimum_cost(3, 2, 1, 3, 2),
            3
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::minimum_cost(5, 4, 15, 2, 3),
            22
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::minimum_cost(5, 4, 15, 0, 0),
            0
        );
    }

    #[test]
    fn test_mixed_optimal() {
        // Type 3 cheaper than sum: use type 3 for overlap
        assert_eq!(
            Solution::minimum_cost(10, 10, 5, 2, 2),
            10
        );
    }

    #[test]
    fn test_all_type3_optimal() {
        assert_eq!(
            Solution::minimum_cost(100, 100, 1, 5, 3),
            5
        );
    }
}
