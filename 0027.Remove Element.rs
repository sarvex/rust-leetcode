impl Solution {
    /// Two-pointer in-place removal of target value from array.
    ///
    /// # Intuition
    /// A write pointer collects all elements that are not equal to the target,
    /// overwriting removed positions. Elements beyond the write pointer are
    /// irrelevant to the result.
    ///
    /// # Approach
    /// Iterate with a read pointer. When the element differs from the target,
    /// copy it to the write position and advance. Return the write pointer
    /// as the new logical length.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — in-place modification
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut write = 0;

        for read in 0..nums.len() {
            if nums[read] != val {
                nums[write] = nums[read];
                write += 1;
            }
        }

        write as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_threes() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        assert_eq!(&nums[..2], &[2, 2]);
    }

    #[test]
    fn remove_twos() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
        assert_eq!(&nums[..5], &[0, 1, 3, 0, 4]);
    }

    #[test]
    fn empty_array() {
        let mut nums: Vec<i32> = vec![];
        assert_eq!(Solution::remove_element(&mut nums, 1), 0);
    }

    #[test]
    fn all_removed() {
        let mut nums = vec![1, 1, 1];
        assert_eq!(Solution::remove_element(&mut nums, 1), 0);
    }

    #[test]
    fn none_removed() {
        let mut nums = vec![1, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 4), 3);
    }
}
