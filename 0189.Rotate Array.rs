impl Solution {
    /// Rotates an array to the right by k steps using the three-reversal technique.
    ///
    /// # Intuition
    /// Reversing the entire array, then reversing the first k and last n-k segments
    /// achieves the rotation in-place without extra space.
    ///
    /// # Approach
    /// 1. Compute effective rotation `k % n`.
    /// 2. Reverse the entire array.
    /// 3. Reverse the first k elements.
    /// 4. Reverse the remaining elements.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k as usize) % n;
        if k == 0 || n <= 1 {
            return;
        }
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_by_three() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn rotate_by_two() {
        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn rotate_by_length() {
        let mut nums = vec![1, 2, 3];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![1, 2, 3]);
    }
}
