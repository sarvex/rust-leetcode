impl Solution {
    /// Stack-allocated in-place DP with iterator-driven inner loop.
    ///
    /// # Intuition
    /// Factor `best_k = max(left_k, above_k)` once, then derive all three
    /// states from `best_k + x` and `best_{k-1}`. This cuts per-cell work
    /// from 7 max + 5 add to 5 max + 3 add.
    ///
    /// # Approach
    /// Fixed `[[i32; 3]; 501]` on stack avoids heap allocation. A `prev`
    /// register tracks the left neighbor, and `zip` iteration eliminates
    /// bounds checks on both `dp` and `row`.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(n)
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let n = coins[0].len();
        let mut dp = [[i32::MIN / 2; 3]; 501];

        dp[1] = [0, 0, 0];
        for row in &coins {
            let mut prev = dp[0];
            for (cell, x) in dp[1..=n].iter_mut().zip(row.iter()) {
                let x = *x;
                let above = *cell;
                let best0 = prev[0].max(above[0]);
                let best1 = prev[1].max(above[1]);
                let best2 = prev[2].max(above[2]);
                *cell = [best0 + x, (best1 + x).max(best0), (best2 + x).max(best1)];
                prev = *cell;
            }
        }

        dp[n][2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::maximum_amount(vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]]),
            8
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::maximum_amount(vec![vec![10, 10, 10], vec![10, 10, 10]]),
            40
        );
    }

    #[test]
    fn test_single_cell_positive() {
        assert_eq!(Solution::maximum_amount(vec![vec![5]]), 5);
    }

    #[test]
    fn test_single_cell_negative() {
        assert_eq!(Solution::maximum_amount(vec![vec![-5]]), 0);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(
            Solution::maximum_amount(vec![vec![-1, -2], vec![-3, -4]]),
            -1
        );
    }

    #[test]
    fn test_no_neutralization_needed() {
        assert_eq!(
            Solution::maximum_amount(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            21
        );
    }

    #[test]
    fn test_single_row_with_robbers() {
        assert_eq!(Solution::maximum_amount(vec![vec![1, -2, 3]]), 4);
    }

    #[test]
    fn test_single_column() {
        assert_eq!(
            Solution::maximum_amount(vec![vec![1], vec![-2], vec![3]]),
            4
        );
    }
}
