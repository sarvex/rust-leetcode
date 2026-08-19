impl Solution {
    /// Interval DP with two-pointer and auxiliary max arrays.
    ///
    /// # Intuition
    /// For each interval, Alice splits it in two. Bob discards the larger half.
    /// As we fix `left` and extend `right`, the balanced split point moves
    /// monotonically rightward, allowing amortized O(1) split-point tracking
    /// per interval via a two-pointer instead of binary search.
    ///
    /// # Approach
    /// 1. Define `max_left[l][r]` = max over `k in [l,r]` of `dp[l][k] + sum(l..=r)`.
    /// 2. Define `max_right[l][r]` = max over `k in [l,r]` of `dp[k][r] + sum(l..=r)`.
    /// 3. For each fixed `left`, sweep `right` from `left+1` to `n-1`, maintaining
    ///    pointer `i` as the largest split where `left_sum <= total/2`.
    /// 4. Use `max_left[left][i]` for left-keep splits, `max_right[i+2][right]` for
    ///    right-keep splits, and handle the equal-sum case with `max_right[i+1][right]`.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n²)
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut f = vec![vec![0; n]; n];
        let mut maxl = vec![vec![0; n]; n];
        let mut maxr = vec![vec![0; n]; n];

        for left in (0..n).rev() {
            maxl[left][left] = stone_value[left];
            maxr[left][left] = stone_value[left];
            let mut total = stone_value[left];
            let mut suml = 0;
            let mut i = left as i32 - 1;

            for right in (left + 1)..n {
                total += stone_value[right];

                while i + 1 < right as i32 && (suml + stone_value[(i + 1) as usize]) * 2 <= total {
                    suml += stone_value[(i + 1) as usize];
                    i += 1;
                }

                if left as i32 <= i {
                    f[left][right] = f[left][right].max(maxl[left][i as usize]);
                }

                if (i + 1) < right as i32 {
                    f[left][right] = f[left][right].max(maxr[(i + 2) as usize][right]);
                }

                if suml * 2 == total {
                    f[left][right] = f[left][right].max(maxr[(i + 1) as usize][right]);
                }

                maxl[left][right] = maxl[left][right - 1].max(total + f[left][right]);
                maxr[left][right] = maxr[left + 1][right].max(total + f[left][right]);
            }
        }

        f[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::stone_game_v(vec![6, 2, 3, 4, 5, 5]), 18);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::stone_game_v(vec![7, 7, 7, 7, 7, 7, 7]), 28);
    }

    #[test]
    fn test_single_stone() {
        assert_eq!(Solution::stone_game_v(vec![4]), 0);
    }

    #[test]
    fn test_two_stones() {
        assert_eq!(Solution::stone_game_v(vec![3, 5]), 3);
    }

    #[test]
    fn test_two_equal_stones() {
        assert_eq!(Solution::stone_game_v(vec![5, 5]), 5);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(Solution::stone_game_v(vec![1, 2, 3, 4]), 4);
    }
}
