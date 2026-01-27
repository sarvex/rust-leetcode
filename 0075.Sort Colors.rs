impl Solution {
    /// Dutch national flag three-way partition for sorting colors.
    ///
    /// # Intuition
    /// With only three distinct values (0, 1, 2), a single pass can
    /// partition the array into three sections using three pointers:
    /// zeros to the left, twos to the right, ones in the middle.
    ///
    /// # Approach
    /// Maintain pointers `low` (boundary of 0s), `mid` (current scanner),
    /// and `high` (boundary of 2s). When `nums[mid]` is 0, swap with
    /// `low` and advance both. When 2, swap with `high` and decrement
    /// `high` (don't advance `mid` since the swapped value is unexamined).
    /// When 1, just advance `mid`.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — in-place swaps
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0usize;
        let mut mid = 0usize;
        let mut high = nums.len();

        while mid < high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                }
                2 => {
                    high -= 1;
                    nums.swap(mid, high);
                }
                _ => mid += 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn already_sorted() {
        let mut nums = vec![0, 1, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn all_same() {
        let mut nums = vec![1, 1, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1, 1, 1]);
    }

    #[test]
    fn single_element() {
        let mut nums = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2]);
    }
}
