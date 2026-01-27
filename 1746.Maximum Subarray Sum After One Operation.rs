impl Solution {
    /// DP tracking max subarray with optional single-element squaring.
    ///
    /// # Intuition
    /// Maintain two DP states: `f` for the best subarray ending here with no
    /// squaring, and `g` for the best subarray ending here with exactly one
    /// element squared. At each step we can either start fresh or extend.
    ///
    /// # Approach
    /// 1. `f = max(f, 0) + nums[i]` — standard Kadane's.
    /// 2. `g = max(max(f_prev, 0) + nums[i]², g_prev + nums[i])` — either
    ///    square the current element or extend a subarray that already used its square.
    /// 3. Track the global maximum of `g`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_sum_after_operation(nums: Vec<i32>) -> i32 {
        let (mut f, mut g, mut result) = (0i64, 0i64, i64::MIN);
        for &e in &nums {
            let e = e as i64;
            let new_f = f.max(0) + e;
            let new_g = (f.max(0) + e * e).max(g + e);
            f = new_f;
            g = new_g;
            result = result.max(g);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::max_sum_after_operation(vec![2, -1, -4, -3]), 17);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::max_sum_after_operation(vec![1, -1, 1, 1, -1, -1, 1]),
            4
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_sum_after_operation(vec![5]), 25);
    }
}
