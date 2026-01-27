impl Solution {
    /// Sorts array so even numbers come before odd numbers using two pointers.
    ///
    /// # Intuition
    /// Two pointers from both ends swap misplaced elements in-place.
    ///
    /// # Approach
    /// Left pointer advances past evens, right pointer retreats past odds.
    /// When both stop, swap and continue.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut l, mut r) = (0, nums.len().wrapping_sub(1));
        while l < r {
            if nums[l] % 2 == 0 {
                l += 1;
            } else if nums[r] % 2 == 1 {
                r = r.wrapping_sub(1);
            } else {
                nums.swap(l, r);
                l += 1;
                r = r.wrapping_sub(1);
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
        let result = Solution::sort_array_by_parity(vec![3, 1, 2, 4]);
        assert!(result.iter().take_while(|&&x| x % 2 == 0).count() == 2);
    }

    #[test]
    fn test_all_even() {
        assert_eq!(Solution::sort_array_by_parity(vec![2, 4, 6]), vec![2, 4, 6]);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
    }
}
