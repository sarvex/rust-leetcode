impl Solution {
    /// Minimum cost to paint all walls using paid and free painters.
    ///
    /// # Intuition
    /// The paid painter paints wall `i` in `time[i]` units at `cost[i]`, during
    /// which the free painter can paint `time[i]` walls. This is a knapsack-like
    /// problem: pick a subset of walls for the paid painter such that total time
    /// covers all remaining walls, minimizing total cost.
    ///
    /// # Approach
    /// 1. Use top-down DP with memoization: `dfs(i, j)` where `i` is current wall
    ///    index and `j` tracks remaining capacity offset.
    /// 2. If remaining walls can all be painted for free, return 0.
    /// 3. If out of walls, return infinity.
    /// 4. Choose to pay for wall `i` (gaining `time[i]` free capacity) or skip it.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n²)
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut memo = vec![vec![-1i32; n << 1 | 1]; n];
        Self::dfs(&mut memo, 0, n as i32, n as i32, &time, &cost)
    }

    fn dfs(memo: &mut [Vec<i32>], i: i32, j: i32, n: i32, time: &[i32], cost: &[i32]) -> i32 {
        if n - i <= j - n {
            return 0;
        }
        if i >= n {
            return 1 << 30;
        }
        let (ui, uj) = (i as usize, j as usize);
        if memo[ui][uj] != -1 {
            return memo[ui][uj];
        }
        let pay = Self::dfs(memo, i + 1, j + time[ui], n, time, cost) + cost[ui];
        let skip = Self::dfs(memo, i + 1, j - 1, n, time, cost);
        memo[ui][uj] = pay.min(skip);
        memo[ui][uj]
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
}
