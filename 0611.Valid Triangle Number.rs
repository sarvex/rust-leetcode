impl Solution {
    /// Counts valid triangle triplets using sorting and two pointers.
    ///
    /// # Intuition
    /// After sorting, fix the largest side and use two pointers to count pairs
    /// whose sum exceeds it. This satisfies the triangle inequality.
    ///
    /// # Approach
    /// 1. Sort the array.
    /// 2. For each element as the largest side (right to left), use two pointers.
    /// 3. If nums[left] + nums[right] > nums[i], all pairs in [left, right) are valid.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(1) in-place sort
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut count = 0;
        for i in (2..n).rev() {
            let (mut left, mut right) = (0, i - 1);
            while left < right {
                if nums[left] + nums[right] > nums[i] {
                    count += right - left;
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    }

    #[test]
    fn test_no_triangle() {
        assert_eq!(Solution::triangle_number(vec![1, 1, 10]), 0);
    }

    #[test]
    fn test_four() {
        assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
    }
}
