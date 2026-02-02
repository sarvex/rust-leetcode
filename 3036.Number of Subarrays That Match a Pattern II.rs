impl Solution {
    /// Count matches via KMP on adjacent comparisons.
    ///
    /// # Intuition
    /// A subarray of size `m + 1` matches the pattern exactly when each adjacent
    /// pair comparison between its elements matches the corresponding `pattern`
    /// entry. This turns the problem into counting occurrences of `pattern`
    /// inside the derived comparison sequence of `nums`.
    ///
    /// # Approach
    /// Build the KMP prefix table for `pattern`. Then scan `nums` once,
    /// computing the comparison between `nums[i]` and `nums[i + 1]` on the fly
    /// and feeding it into the KMP matcher, counting all matches including
    /// overlaps.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(m)
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        if pattern.is_empty() || nums.len() < pattern.len() + 1 {
            return 0;
        }

        let m = pattern.len();
        let mut lps = vec![0usize; m];
        let mut len = 0usize;

        for i in 1..m {
            while len > 0 && pattern[i] != pattern[len] {
                len = lps[len - 1];
            }
            if pattern[i] == pattern[len] {
                len += 1;
                lps[i] = len;
            }
        }

        let mut count = 0i32;
        let mut matched = 0usize;

        for i in 0..nums.len() - 1 {
            let cmp = if nums[i + 1] > nums[i] {
                1
            } else if nums[i + 1] < nums[i] {
                -1
            } else {
                0
            };

            while matched > 0 && cmp != pattern[matched] {
                matched = lps[matched - 1];
            }
            if cmp == pattern[matched] {
                matched += 1;
                if matched == m {
                    count += 1;
                    matched = lps[matched - 1];
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let pattern = vec![1, 1];
        assert_eq!(Solution::count_matching_subarrays(nums, pattern), 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 4, 4, 1, 3, 5, 5, 3];
        let pattern = vec![1, 0, -1];
        assert_eq!(Solution::count_matching_subarrays(nums, pattern), 2);
    }

    #[test]
    fn test_all_equal() {
        let nums = vec![5, 5, 5, 5];
        let pattern = vec![0, 0];
        assert_eq!(Solution::count_matching_subarrays(nums, pattern), 2);
    }

    #[test]
    fn test_overlapping_increasing() {
        let nums = vec![1, 2, 3, 4];
        let pattern = vec![1, 1];
        assert_eq!(Solution::count_matching_subarrays(nums, pattern), 2);
    }

    #[test]
    fn test_single_step_pattern() {
        let nums = vec![1, 2, 2, 1];
        let pattern = vec![1];
        assert_eq!(Solution::count_matching_subarrays(nums, pattern), 1);
    }
}
