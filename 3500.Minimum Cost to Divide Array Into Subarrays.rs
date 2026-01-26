impl Solution {
    /// Convex Hull Trick optimized DP with cost reformulation
    ///
    /// # Intuition
    /// Reformulate the cost by separating the subarray index contribution. When making
    /// a cut after position c, all subsequent subarrays have index increased by 1,
    /// contributing k * (C[n] - C[c+1]) to total cost.
    ///
    /// # Approach
    /// Total cost = Σ[S[r+1] * (C[r+1] - C[l])] + k*C[n] + k*Σ(C[n] - C[cut+1])
    /// Define dp[i] = min cost for first i elements (excluding final k*C[n]).
    /// Transition: dp[i] = min(S[i]*C[i], S[i]*C[i] + k*C[n] + min_j{dp[j] - (S[i]+k)*C[j]})
    /// This is CHT form with decreasing slopes -C[j] and increasing queries S[i]+k.
    ///
    /// # Complexity
    /// - Time: O(n) - single pass with amortized O(1) CHT operations
    /// - Space: O(n) - prefix sums and convex hull storage
    pub fn minimum_cost(nums: Vec<i32>, cost: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as i64;

        // Prefix sums
        let mut s = vec![0i64; n + 1];
        let mut c = vec![0i64; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i] as i64;
            c[i + 1] = c[i] + cost[i] as i64;
        }
        let c_n = c[n];

        // CHT: lines (slope, intercept), monotonic deque with decreasing slopes
        let mut hull: Vec<(i64, i64)> = Vec::with_capacity(n);
        let mut ptr = 0usize;

        // Check if middle line is dominated
        let dominated = |a: (i64, i64), b: (i64, i64), c: (i64, i64)| -> bool {
            (b.1 - a.1) as i128 * (b.0 - c.0) as i128 >= (c.1 - b.1) as i128 * (a.0 - b.0) as i128
        };

        let mut dp = 0i64;

        for i in 1..=n {
            // Option 1: single subarray [0, i-1], no cuts
            let opt1 = s[i] * c[i];

            // Option 2: at least one cut, query CHT for min{dp[j] - (S[i]+k)*C[j]}
            let opt2 = if !hull.is_empty() {
                let x = s[i] + k;
                while ptr + 1 < hull.len()
                    && hull[ptr].0 * x + hull[ptr].1 >= hull[ptr + 1].0 * x + hull[ptr + 1].1
                {
                    ptr += 1;
                }
                s[i] * c[i] + k * c_n + hull[ptr].0 * x + hull[ptr].1
            } else {
                i64::MAX
            };

            dp = opt1.min(opt2);

            // Add line for current position: slope = -c[i], intercept = dp
            let line = (-c[i], dp);
            while hull.len() >= 2 && dominated(hull[hull.len() - 2], hull[hull.len() - 1], line) {
                hull.pop();
            }
            hull.push(line);
            ptr = ptr.min(hull.len() - 1);
        }

        dp + k * c_n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 1, 4];
        let cost = vec![4, 6, 6];
        let k = 1;

        let result = Solution::minimum_cost(nums, cost, k);

        assert_eq!(result, 110);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 8, 5, 1, 14, 2, 2, 12, 1];
        let cost = vec![7, 2, 8, 4, 2, 2, 1, 1, 2];
        let k = 7;

        let result = Solution::minimum_cost(nums, cost, k);

        assert_eq!(result, 985);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![5];
        let cost = vec![3];
        let k = 2;

        let result = Solution::minimum_cost(nums, cost, k);

        // Single subarray: (5 + 2*1) * 3 = 21
        assert_eq!(result, 21);
    }

    #[test]
    fn test_two_elements_no_split() {
        let nums = vec![1, 1];
        let cost = vec![1, 1];
        let k = 100;

        let result = Solution::minimum_cost(nums, cost, k);

        // No split: (1+1 + 100*1) * (1+1) = 102 * 2 = 204
        // Split: (1 + 100) * 1 + (2 + 200) * 1 = 101 + 202 = 303
        assert_eq!(result, 204);
    }

    #[test]
    fn test_two_elements_with_split() {
        let nums = vec![1, 1];
        let cost = vec![100, 1];
        let k = 1;

        let result = Solution::minimum_cost(nums, cost, k);

        // No split: (2 + 1) * 101 = 303
        // Split: (1 + 1) * 100 + (2 + 2) * 1 = 200 + 4 = 204
        assert_eq!(result, 204);
    }

    #[test]
    fn test_uniform_arrays() {
        let nums = vec![1, 1, 1];
        let cost = vec![1, 1, 1];
        let k = 1;

        let result = Solution::minimum_cost(nums, cost, k);

        // Split [0,1],[2,2]: (2+1)*2 + (3+2)*1 = 11
        assert_eq!(result, 11);
    }

    #[test]
    fn test_large_k_favors_no_split() {
        let nums = vec![10, 20, 30];
        let cost = vec![1, 1, 1];
        let k = 1000;

        let result = Solution::minimum_cost(nums, cost, k);

        // Large k makes splitting expensive, single subarray optimal
        // (60 + 1000) * 3 = 3180
        assert_eq!(result, 3180);
    }

    #[test]
    fn test_minimum_values() {
        let nums = vec![1];
        let cost = vec![1];
        let k = 1;

        let result = Solution::minimum_cost(nums, cost, k);

        // (1 + 1) * 1 = 2
        assert_eq!(result, 2);
    }

    #[test]
    fn test_maximum_single_values() {
        let nums = vec![1000];
        let cost = vec![1000];
        let k = 1000;

        let result = Solution::minimum_cost(nums, cost, k);

        // (1000 + 1000) * 1000 = 2_000_000
        assert_eq!(result, 2_000_000);
    }

    #[test]
    fn test_increasing_sequence() {
        let nums = vec![1, 2, 3, 4];
        let cost = vec![1, 2, 3, 4];
        let k = 1;

        let result = Solution::minimum_cost(nums, cost, k);

        assert!(result > 0);
    }

    #[test]
    fn test_decreasing_cost() {
        let nums = vec![1, 1, 1, 1];
        let cost = vec![4, 3, 2, 1];
        let k = 1;

        let result = Solution::minimum_cost(nums, cost, k);

        assert!(result > 0);
    }
}
