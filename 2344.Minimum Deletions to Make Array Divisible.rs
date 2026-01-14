impl Solution {
    /// Minimum Deletions to Make Array Divisible
    ///
    /// # Intuition
    /// A number divides all elements in `numsDivide` if and only if it divides
    /// their GCD. This reduces the problem to finding the smallest element in
    /// `nums` that divides `gcd(numsDivide)`.
    ///
    /// # Approach
    /// 1. Compute the GCD of all elements in `numsDivide`
    /// 2. Find minimum element in `nums` that divides the GCD (single pass)
    /// 3. Count elements smaller than this minimum (single pass)
    ///
    /// # Complexity
    /// - Time: O(n + m log M) where n = nums.len(), m = numsDivide.len(), M = max(numsDivide)
    /// - Space: O(1)
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let target_gcd = nums_divide.into_iter().fold(0, Self::gcd);

        let min_valid = nums.iter().filter(|&&num| target_gcd % num == 0).min();

        match min_valid {
            Some(&min) => nums.iter().filter(|&&num| num < min).count() as i32,
            None => -1,
        }
    }

    #[inline]
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 3, 2, 4, 3];
        let nums_divide = vec![9, 6, 9, 3, 15];
        assert_eq!(Solution::min_operations(nums, nums_divide), 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 3, 6];
        let nums_divide = vec![8, 2, 6, 10];
        assert_eq!(Solution::min_operations(nums, nums_divide), -1);
    }

    #[test]
    fn test_single_element_divides() {
        let nums = vec![1];
        let nums_divide = vec![5, 10, 15];
        assert_eq!(Solution::min_operations(nums, nums_divide), 0);
    }

    #[test]
    fn test_all_same_elements() {
        let nums = vec![3, 3, 3];
        let nums_divide = vec![9, 12, 15];
        assert_eq!(Solution::min_operations(nums, nums_divide), 0);
    }

    #[test]
    fn test_no_valid_divisor() {
        let nums = vec![7, 11, 13];
        let nums_divide = vec![4, 8, 12];
        assert_eq!(Solution::min_operations(nums, nums_divide), -1);
    }

    #[test]
    fn test_delete_multiple_elements() {
        let nums = vec![2, 2, 2, 5, 5];
        let nums_divide = vec![25, 50, 75];
        assert_eq!(Solution::min_operations(nums, nums_divide), 3);
    }

    #[test]
    fn test_large_gcd() {
        let nums = vec![100, 50, 25, 5];
        let nums_divide = vec![1000, 500, 250];
        assert_eq!(Solution::min_operations(nums, nums_divide), 0);
    }
}
