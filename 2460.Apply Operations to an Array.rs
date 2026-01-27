impl Solution {
    /// Applies doubling operations on equal adjacent elements, then moves zeros to end.
    ///
    /// # Intuition
    /// First pass: when adjacent elements are equal, double the left and zero the right.
    /// Second pass: move all non-zero elements to the front (partition around zero).
    ///
    /// # Approach
    /// 1. Iterate through pairs, doubling left element when equal to right
    /// 2. Use a write pointer to compact non-zero elements to the front
    ///
    /// # Complexity
    /// - Time: O(n) — two linear passes
    /// - Space: O(1) — in-place modification
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] <<= 1;
                nums[i + 1] = 0;
            }
        }

        let mut write = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, write);
                write += 1;
            }
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
    }

    #[test]
    fn test_no_operations() {
        assert_eq!(Solution::apply_operations(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(
            Solution::apply_operations(vec![2, 2, 2, 2]),
            vec![4, 4, 0, 0]
        );
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::apply_operations(vec![0, 0, 0]), vec![0, 0, 0]);
    }
}
