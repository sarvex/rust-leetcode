impl Solution {
    /// Minimum Travel Time After K Merge Operations
    ///
    /// # Intuition
    /// When we merge sign i into sign i+1, the time value accumulates. The key observation
    /// is that if we keep signs at positions s0, s1, ..., sm, the segment from si to si+1
    /// uses the accumulated time at si. This accumulated time equals the sum of times from
    /// all merged signs since the previous kept sign.
    ///
    /// # Approach
    /// Use 3D dynamic programming:
    /// - dp[i][j][t] = minimum cost to reach sign i, having performed j merges, with
    ///   accumulated time t at sign i
    /// - Transition: from sign i, try keeping next sign at nxt, accumulating times of
    ///   removed signs (i+1 to nxt-1) into nxt
    /// - Segment cost = distance(i to nxt) × accumulated_time_at_i
    ///
    /// # Complexity
    /// - Time: O(n² × k × T) where T = sum of times (≤100)
    /// - Space: O(n × k × T)
    pub fn min_travel_time(_l: i32, n: i32, k: i32, position: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let max_time = 101;

        let mut dp = vec![vec![vec![i32::MAX; max_time]; k + 1]; n];
        dp[0][0][time[0] as usize] = 0;

        let prefix: Vec<i32> = std::iter::once(0)
            .chain(time.iter().scan(0, |acc, &t| {
                *acc += t;
                Some(*acc)
            }))
            .collect();

        for i in 0..n - 1 {
            for j in 0..=k {
                for t in 0..max_time {
                    let curr = dp[i][j][t];
                    if curr == i32::MAX {
                        continue;
                    }

                    for nxt in i + 1..n {
                        let merges = nxt - i - 1;
                        let new_j = j + merges;
                        if new_j > k {
                            break;
                        }

                        let new_t = (prefix[nxt + 1] - prefix[i + 1]) as usize;
                        let segment_cost = (position[nxt] - position[i]) * t as i32;

                        dp[nxt][new_j][new_t] = dp[nxt][new_j][new_t].min(curr + segment_cost);
                    }
                }
            }
        }

        (0..max_time)
            .filter_map(|t| (dp[n - 1][k][t] < i32::MAX).then_some(dp[n - 1][k][t]))
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::min_travel_time(10, 4, 1, vec![0, 3, 8, 10], vec![5, 8, 3, 6]),
            62
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::min_travel_time(5, 5, 1, vec![0, 1, 2, 3, 5], vec![8, 3, 9, 3, 3]),
            34
        );
    }

    #[test]
    fn test_no_merges() {
        assert_eq!(
            Solution::min_travel_time(10, 4, 0, vec![0, 3, 8, 10], vec![5, 8, 3, 6]),
            61
        );
    }

    #[test]
    fn test_two_signs() {
        assert_eq!(
            Solution::min_travel_time(5, 2, 0, vec![0, 5], vec![2, 1]),
            10
        );
    }

    #[test]
    fn test_max_merges() {
        assert_eq!(
            Solution::min_travel_time(10, 4, 2, vec![0, 3, 8, 10], vec![5, 8, 3, 6]),
            50
        );
    }
}
