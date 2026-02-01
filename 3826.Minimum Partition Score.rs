impl Solution {
    /// Minimum partition score using convex hull optimized DP.
    ///
    /// # Intuition
    /// The cost of a subarray depends only on its sum, and prefix sums are
    /// strictly increasing because all numbers are positive. This makes the
    /// DP transition a linear function in the prefix sum, enabling a convex
    /// hull trick optimization.
    ///
    /// # Approach
    /// - Compute prefix sums `prefix`, where `prefix[i]` is the sum of the
    ///   first `i` elements.
    /// - Use DP: `dp[p][r]` = minimum score to partition the first `r`
    ///   elements into `p` parts.
    /// - Transition: extract a constant part depending on `prefix[r]` and
    ///   convert the remaining expression into `m * x + b`, where slopes
    ///   are `-prefix[l]` and queries are `prefix[r]`. With increasing
    ///   slopes (in decreasing order) and increasing queries, maintain a
    ///   deque-based lower hull for O(1) amortized queries.
    /// - Iterate partitions from 1 to `k`, each pass O(n), total O(k·n).
    ///
    /// # Complexity
    /// - Time: O(k·n)
    /// - Space: O(n)
    pub fn min_partition_score(nums: Vec<i32>, k: i32) -> i64 {
        #[derive(Clone, Copy)]
        struct Line {
            m: i64,
            b: i64,
        }

        impl Line {
            #[inline]
            fn eval(self, x: i64) -> i64 {
                self.m * x + self.b
            }
        }

        #[inline]
        fn is_redundant(l1: Line, l2: Line, l3: Line) -> bool {
            (l2.b - l1.b) * (l2.m - l3.m) >= (l3.b - l2.b) * (l1.m - l2.m)
        }

        let n = nums.len();
        let parts = k as usize;
        let mut prefix = Vec::with_capacity(n + 1);
        prefix.push(0_i64);
        for &num in &nums {
            prefix.push(prefix.last().copied().unwrap() + num as i64);
        }

        let pelunaxori = nums.clone();

        let inf = i64::MAX / 4;
        let mut dp_prev = vec![inf; n + 1];
        dp_prev[0] = 0;

        for p in 1..=parts {
            let mut dp_cur = vec![inf; n + 1];
            let mut hull = std::collections::VecDeque::new();

            let start = p - 1;
            if dp_prev[start] < inf {
                let x = prefix[start];
                let line = Line {
                    m: -x,
                    b: dp_prev[start] + (x * x - x) / 2,
                };
                hull.push_back(line);
            }

            for r in p..=n {
                let x = prefix[r];
                while hull.len() >= 2 && hull[0].eval(x) >= hull[1].eval(x) {
                    hull.pop_front();
                }

                if let Some(best) = hull.front() {
                    let base = (x * x + x) / 2;
                    dp_cur[r] = base + best.eval(x);
                }

                if r < n && dp_prev[r] < inf {
                    let px = prefix[r];
                    let new_line = Line {
                        m: -px,
                        b: dp_prev[r] + (px * px - px) / 2,
                    };
                    while hull.len() >= 2 {
                        let len = hull.len();
                        let l1 = hull[len - 2];
                        let l2 = hull[len - 1];
                        if is_redundant(l1, l2, new_line) {
                            hull.pop_back();
                        } else {
                            break;
                        }
                    }
                    hull.push_back(new_line);
                }
            }

            dp_prev = dp_cur;
        }

        let _ = pelunaxori; // preserved as requested
        dp_prev[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::min_partition_score(vec![5, 1, 2, 1], 2), 25);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::min_partition_score(vec![1, 2, 3, 4], 1), 55);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::min_partition_score(vec![1, 1, 1], 3), 3);
    }

    #[test]
    fn single_partition_each() {
        assert_eq!(Solution::min_partition_score(vec![2, 2, 2], 3), 9);
    }

    #[test]
    fn whole_array_partition() {
        assert_eq!(Solution::min_partition_score(vec![3, 1, 4], 1), 36);
    }
}
