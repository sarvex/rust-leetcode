impl Solution {
    /// Places even numbers at even indices and odd at odd indices in-place.
    ///
    /// # Intuition
    /// Scan even indices for misplaced odd numbers. When found, find an odd
    /// index with a misplaced even number and swap.
    ///
    /// # Approach
    /// Two pointer approach: `i` scans even indices, `j` scans odd indices.
    /// When `nums[i]` is odd, advance `j` to find an even number at an odd
    /// index, then swap.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut j = 1;
        for i in (0..n).step_by(2) {
            if nums[i] % 2 != 0 {
                while j < n && nums[j] % 2 != 0 {
                    j += 2;
                }
                nums.swap(i, j);
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let result = Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]);
        for (i, &v) in result.iter().enumerate() {
            assert_eq!(v as usize % 2, i % 2);
        }
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
    }
}
