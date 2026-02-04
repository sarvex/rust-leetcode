impl Solution {
    /// Count bounded knapsack combinations with dynamic programming.
    ///
    /// # Intuition
    /// Each question type contributes `0..=count` copies of its score. Processing types one by one
    /// lets us build the number of ways to reach every total up to `target`.
    ///
    /// # Approach
    /// 1. Initialize `dp[0] = 1` for the empty selection.
    /// 2. For each type, create a fresh `next` array.
    /// 3. For every reachable score, add it to `next` for all usable counts of that type.
    /// 4. Return `dp[target]` modulo `1_000_000_007`.
    ///
    /// # Complexity
    /// - Time: O(n * target * max_count)
    /// - Space: O(target)
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let target = target as usize;
        let mut dp = vec![0i64; target + 1];
        dp[0] = 1;

        for question_type in types {
            let count = question_type[0] as usize;
            let marks = question_type[1] as usize;
            let mut next = vec![0i64; target + 1];

            for score in 0..=target {
                let base = dp[score];
                if base == 0 {
                    continue;
                }

                for used in 0..=count {
                    let new_score = score + used * marks;
                    if new_score > target {
                        break;
                    }
                    let mut updated = next[new_score] + base;
                    if updated >= MOD {
                        updated -= MOD;
                    }
                    next[new_score] = updated;
                }
            }

            dp = next;
        }

        (dp[target] % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let target = 6;
        let types = vec![vec![6, 1], vec![3, 2], vec![2, 3]];
        assert_eq!(Solution::ways_to_reach_target(target, types), 7);
    }

    #[test]
    fn example_two() {
        let target = 5;
        let types = vec![vec![50, 1], vec![50, 2], vec![50, 5]];
        assert_eq!(Solution::ways_to_reach_target(target, types), 4);
    }

    #[test]
    fn example_three() {
        let target = 18;
        let types = vec![vec![6, 1], vec![3, 2], vec![2, 3]];
        assert_eq!(Solution::ways_to_reach_target(target, types), 1);
    }

    #[test]
    fn unreachable_target() {
        let target = 1;
        let types = vec![vec![2, 2]];
        assert_eq!(Solution::ways_to_reach_target(target, types), 0);
    }

    #[test]
    fn mixed_counts() {
        let target = 3;
        let types = vec![vec![1, 1], vec![1, 2]];
        assert_eq!(Solution::ways_to_reach_target(target, types), 1);
    }
}
