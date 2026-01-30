
impl Solution {
    /// Finds minimum deletions so the smallest remaining element divides all of numsDivide.
    ///
    /// # Intuition
    /// A number divides all elements in `numsDivide` iff it divides their GCD.
    /// This reduces to finding the smallest element in `nums` that divides `gcd(numsDivide)`.
    ///
    /// # Approach
    /// 1. Compute GCD of all elements in `numsDivide`
    /// 2. Find minimum element in `nums` that divides the GCD
    /// 3. Count elements smaller than this minimum
    ///
    /// # Complexity
    /// - Time: O(n + m log M) where n = nums.len(), m = numsDivide.len(), M = max(numsDivide)
    /// - Space: O(1)
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let target_gcd = nums_divide.into_iter().fold(0, Self::gcd);

        nums.iter()
            .filter(|num| target_gcd % **num == 0)
            .min()
            .map_or(-1, |&min_val| {
                nums.iter().filter(|num| **num < min_val).count() as i32
            })
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
        assert_eq!(
            Solution::min_operations(vec![2, 3, 2, 4, 3], vec![9, 6, 9, 3, 15]),
            2
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::min_operations(vec![4, 3, 6], vec![8, 2, 6, 10]),
            -1
        );
    }

    #[test]
    fn test_single_element_divides() {
        assert_eq!(Solution::min_operations(vec![1], vec![5, 10, 15]), 0);
    }

    #[test]
    fn test_all_same_elements() {
        assert_eq!(Solution::min_operations(vec![3, 3, 3], vec![9, 12, 15]), 0);
    }

    #[test]
    fn test_no_valid_divisor() {
        assert_eq!(
            Solution::min_operations(vec![7, 11, 13], vec![4, 8, 12]),
            -1
        );
    }

    #[test]
    fn test_delete_multiple_elements() {
        assert_eq!(
            Solution::min_operations(vec![2, 2, 2, 5, 5], vec![25, 50, 75]),
            3
        );
    }

    #[test]
    fn test_large_gcd() {
        assert_eq!(
            Solution::min_operations(vec![100, 50, 25, 5], vec![1000, 500, 250]),
            0
        );
    }
}
