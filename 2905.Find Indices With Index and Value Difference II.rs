impl Solution {
    /// Finds two indices satisfying both index and value difference constraints.
    ///
    /// # Intuition
    /// Track the running minimum and maximum indices as we scan. At each position
    /// `i`, the candidate partner `j = i - index_difference` is exactly far enough.
    /// Checking against the tracked min/max covers all valid pairs in O(n).
    ///
    /// # Approach
    /// 1. Maintain indices `mi` and `mx` for the minimum and maximum values seen
    ///    among elements at distance >= `index_difference`.
    /// 2. For each `i`, update `mi`/`mx` and check if either extreme pair meets
    ///    the value difference threshold.
    ///
    /// # Complexity
    /// - Time: O(n) single pass
    /// - Space: O(1)
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let index_difference = index_difference as usize;
        let mut mi = 0;
        let mut mx = 0;

        for i in index_difference..nums.len() {
            let j = i - index_difference;

            if nums[j] < nums[mi] {
                mi = j;
            }
            if nums[j] > nums[mx] {
                mx = j;
            }
            if nums[i] - nums[mi] >= value_difference {
                return vec![mi as i32, i as i32];
            }
            if nums[mx] - nums[i] >= value_difference {
                return vec![mx as i32, i as i32];
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found_indices() {
        let result = Solution::find_indices(vec![5, 1, 4, 1], 2, 4);
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn test_no_valid_pair() {
        let result = Solution::find_indices(vec![2, 1], 0, 0);
        assert_eq!(result, vec![0, 0]);
    }
}
