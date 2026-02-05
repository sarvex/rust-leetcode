impl Solution {
    /// Smallest prefix length to remove so the remaining suffix is strictly increasing.
    ///
    /// # Intuition
    /// The longest strictly-increasing suffix is found by scanning from the end;
    /// the minimum prefix length is then the index where that suffix starts.
    ///
    /// # Approach
    /// 1. Start with the suffix as the last element (always strictly increasing).
    /// 2. Move the start index left while `nums[i] < nums[i + 1]`.
    /// 3. Return that start index as the prefix length to remove.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let mut start = n - 1;
        for i in (0..n - 1).rev() {
            if nums[i] >= nums[i + 1] {
                break;
            }
            start = i;
        }
        start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::minimum_prefix_length(vec![1, -1, 2, 3, 3, 4, 5]),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_prefix_length(vec![4, 3, -2, -5]), 3);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::minimum_prefix_length(vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::minimum_prefix_length(vec![42]), 0);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::minimum_prefix_length(vec![5, 5, 5, 5]), 3);
    }

    #[test]
    fn test_strictly_decreasing() {
        assert_eq!(Solution::minimum_prefix_length(vec![3, 2, 1]), 2);
    }
}
