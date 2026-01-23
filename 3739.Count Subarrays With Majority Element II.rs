impl Solution {
    /// Linear Time Counting with Running Prefix Sum
    ///
    /// # Intuition
    /// Transform the problem by mapping each element to +1 if it equals target, -1 otherwise.
    /// A subarray has target as majority iff its transformed sum is positive. Since prefix
    /// sums change by exactly ±1 at each step, we can maintain a running count of previous
    /// prefix values less than current in O(1) per step.
    ///
    /// # Approach
    /// 1. Transform array: +1 for target elements, -1 for others
    /// 2. Track count of each prefix sum value seen so far
    /// 3. Maintain running sum of counts for values < current prefix
    /// 4. When prefix increases by 1: left_count gains count[old_prefix]
    /// 5. When prefix decreases by 1: left_count loses count[new_prefix]
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of nums
    /// - Space: O(n) for the count array
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let n = nums.len();
        let offset = n as i64;
        let mut count = vec![0i64; 2 * n + 1];

        let mut result = 0i64;
        let mut prefix = 0i64;
        let mut left_count = 0i64;

        count[(prefix + offset) as usize] = 1;

        for &num in &nums {
            if num == target {
                left_count += count[(prefix + offset) as usize];
                prefix += 1;
            } else {
                prefix -= 1;
                left_count -= count[(prefix + offset) as usize];
            }

            result += left_count;
            count[(prefix + offset) as usize] += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 3], 4), 0);
    }

    #[test]
    fn test_single_element_match() {
        assert_eq!(Solution::count_majority_subarrays(vec![5], 5), 1);
    }

    #[test]
    fn test_single_element_no_match() {
        assert_eq!(Solution::count_majority_subarrays(vec![5], 3), 0);
    }

    #[test]
    fn test_all_target() {
        assert_eq!(Solution::count_majority_subarrays(vec![2, 2, 2], 2), 6);
    }

    #[test]
    fn test_no_target() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 3, 5], 2), 0);
    }

    #[test]
    fn test_alternating() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 1, 2, 1], 2), 3);
    }
}
