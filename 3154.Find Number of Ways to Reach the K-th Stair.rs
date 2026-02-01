impl Solution {
    /// Combinatorial counting by choosing down-move gaps.
    ///
    /// # Intuition
    /// After `j` upward jumps, the total upward distance is `2^j - 1`, so the final stair is fully
    /// determined by how many single-step downs are inserted. Downs cannot be consecutive, which
    /// means at most one down can be placed in each gap between jumps (including before the first
    /// and after the last jump).
    ///
    /// # Approach
    /// For each `j >= 0`, compute the required number of downs `d = 2^j - k`. If `0 <= d <= j + 1`,
    /// the number of valid sequences is `C(j + 1, d)` by choosing which gaps contain a down. Sum
    /// these counts across all feasible `j`. Once `2^j - k > j + 1`, no larger `j` can work.
    ///
    /// # Complexity
    /// - Time: O(log k)
    /// - Space: O(1)
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        let target = k as i64;
        let mut total = 0_i64;

        for jump in 0..=60_u32 {
            let up_total = 1_i64 << jump;
            let downs = up_total - target;
            if downs < 0 {
                continue;
            }

            let max_downs = jump as i64 + 1;
            if downs > max_downs {
                if up_total > target {
                    break;
                }
                continue;
            }

            total += Self::binomial(max_downs, downs);
        }

        total as i32
    }

    fn binomial(n: i64, r: i64) -> i64 {
        if r < 0 || r > n {
            return 0;
        }
        let r = r.min(n - r);
        let mut result = 1_i64;
        for i in 1..=r {
            result = result * (n - r + i) / i;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_k0() {
        assert_eq!(Solution::ways_to_reach_stair(0), 2);
    }

    #[test]
    fn test_example_k1() {
        assert_eq!(Solution::ways_to_reach_stair(1), 4);
    }

    #[test]
    fn test_k2() {
        assert_eq!(Solution::ways_to_reach_stair(2), 4);
    }

    #[test]
    fn test_k3() {
        assert_eq!(Solution::ways_to_reach_stair(3), 3);
    }
}
