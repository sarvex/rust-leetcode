
impl Solution {
    /// Finds the shortest subarray that, when sorted, sorts the entire array.
    ///
    /// # Intuition
    /// Scan left-to-right tracking the running max to find the rightmost element
    /// out of place. Scan right-to-left tracking the running min to find the
    /// leftmost element out of place.
    ///
    /// # Approach
    /// 1. Forward pass: track max; any element below max marks the right boundary.
    /// 2. Backward pass: track min; any element above min marks the left boundary.
    /// 3. Return right - left + 1, or 0 if already sorted.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut left, mut right) = (-1i32, -1i32);
        let (mut max_val, mut min_val) = (i32::MIN, i32::MAX);
        for i in 0..n {
            if nums[i] < max_val {
                right = i as i32;
            } else {
                max_val = nums[i];
            }
            if nums[n - 1 - i] > min_val {
                left = (n - 1 - i) as i32;
            } else {
                min_val = nums[n - 1 - i];
            }
        }
        if right == -1 {
            0
        } else {
            right - left + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
            5
        );
    }

    #[test]
    fn test_sorted() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
    }

    #[test]
    fn test_reverse_sorted() {
        assert_eq!(Solution::find_unsorted_subarray(vec![5, 4, 3, 2, 1]), 5);
    }

    #[test]
    fn test_two_elements_sorted() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2]), 0);
    }

    #[test]
    fn test_two_elements_unsorted() {
        assert_eq!(Solution::find_unsorted_subarray(vec![2, 1]), 2);
    }

    #[test]
    fn test_unsorted_at_beginning() {
        assert_eq!(Solution::find_unsorted_subarray(vec![3, 2, 1, 4, 5]), 3);
    }

    #[test]
    fn test_unsorted_at_end() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 5, 4, 3]), 3);
    }

    #[test]
    fn test_duplicate_values() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 3, 2, 2, 2]), 4);
    }
}
