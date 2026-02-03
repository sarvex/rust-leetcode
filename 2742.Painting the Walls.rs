impl Solution {
    /// Minimum cost using a 1D knapsack over covered walls.
    ///
    /// # Intuition
    /// Paying for wall `i` occupies the paid painter for `time[i]` units, which
    /// lets the free painter paint `time[i]` other walls. So paying for wall `i`
    /// effectively covers `time[i] + 1` walls. We want the cheapest set of paid
    /// walls whose total coverage reaches `n`.
    ///
    /// # Approach
    /// Use a 0/1 knapsack DP where `dp[k]` is the minimum cost to cover `k` walls.
    /// For each wall, decide to pay for it and update coverage in reverse to avoid
    /// reusing the same wall. Clamp coverage at `n`.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n)
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        let inf = i64::from(i32::MAX);
        let mut dp = vec![inf; n + 1];
        dp[0] = 0;

        for (&wall_cost, &wall_time) in cost.iter().zip(time.iter()) {
            let coverage = wall_time as usize + 1;
            for covered in (0..=n).rev() {
                if dp[covered] == inf {
                    continue;
                }
                let next = (covered + coverage).min(n);
                let candidate = dp[covered] + i64::from(wall_cost);
                if candidate < dp[next] {
                    dp[next] = candidate;
                }
            }
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_wall_painting() {
        assert_eq!(Solution::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]), 3);
    }

    #[test]
    fn expensive_but_time_efficient() {
        assert_eq!(Solution::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]), 4);
    }

    #[test]
    fn single_wall_requires_paid_painter() {
        assert_eq!(Solution::paint_walls(vec![7], vec![3]), 7);
    }

    #[test]
    fn choose_cheapest_paid_walls() {
        assert_eq!(Solution::paint_walls(vec![10, 1, 1], vec![3, 1, 1]), 2);
    }

    #[test]
    fn one_paid_wall_covers_all() {
        assert_eq!(Solution::paint_walls(vec![8, 5, 2], vec![2, 2, 3]), 2);
    }
}
