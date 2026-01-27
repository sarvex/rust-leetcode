impl Solution {
    /// Find the maximum sum score — the larger of prefix or suffix sum at any index.
    ///
    /// # Intuition
    /// At each index, the sum score is max(prefix_sum, suffix_sum). Track running
    /// prefix and suffix totals in a single pass.
    ///
    /// # Approach
    /// 1. Compute total sum as the initial suffix.
    /// 2. For each element, update the prefix sum and compare both prefix and suffix
    ///    against the running maximum, then shrink the suffix.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn maximum_sum_score(nums: Vec<i32>) -> i64 {
        let mut prefix = 0i64;
        let mut suffix: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut max_score = i64::MIN;

        for &x in &nums {
            prefix += x as i64;
            max_score = max_score.max(prefix).max(suffix);
            suffix -= x as i64;
        }

        max_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::maximum_sum_score(vec![4, 3, -2, 5]), 10);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::maximum_sum_score(vec![-3]), -3);
    }

    #[test]
    fn all_negative() {
        assert_eq!(Solution::maximum_sum_score(vec![-1, -2, -3]), -1);
    }
}
