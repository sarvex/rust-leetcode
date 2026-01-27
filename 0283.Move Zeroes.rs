impl Solution {
    /// Moves all zeroes to the end while maintaining relative order of non-zero elements.
    ///
    /// # Intuition
    /// Use a write pointer to track the next position for a non-zero element.
    /// Swap non-zero elements into place as they are encountered.
    ///
    /// # Approach
    /// 1. Maintain a write index starting at 0.
    /// 2. For each non-zero element, swap it with the write position and advance.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut write = 0;
        for read in 0..nums.len() {
            if nums[read] != 0 {
                nums.swap(read, write);
                write += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn single_zero() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn no_zeroes() {
        let mut nums = vec![1, 2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }
}
