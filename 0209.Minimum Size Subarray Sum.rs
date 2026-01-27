impl Solution {
    /// Finds the minimum length subarray with sum >= target using sliding window.
    ///
    /// # Intuition
    /// Expand the window by advancing the right pointer. Once the sum meets
    /// the target, shrink from the left to find the minimum valid window.
    ///
    /// # Approach
    /// 1. Maintain a sliding window with left and right pointers.
    /// 2. Add elements at the right end.
    /// 3. While the window sum >= target, record the length and shrink from the left.
    /// 4. Return 0 if no valid subarray exists.
    ///
    /// # Complexity
    /// - Time: O(n) — each element is added and removed at most once
    /// - Space: O(1)
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_len = n + 1;
        let mut sum = 0;
        let mut left = 0;
        for right in 0..n {
            sum += nums[right];
            while sum >= target {
                min_len = min_len.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
        if min_len == n + 1 {
            0
        } else {
            min_len as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn single_element_sufficient() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn no_valid_subarray() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}
