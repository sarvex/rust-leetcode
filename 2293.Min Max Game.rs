impl Solution {
    /// Simulates the min-max game reducing array to a single element.
    ///
    /// # Intuition
    /// Each round halves the array, alternately taking min and max of consecutive
    /// pairs. We can reduce in-place to avoid extra allocations.
    ///
    /// # Approach
    /// Repeatedly halve the array length. For each pair at index `i`, apply `min`
    /// when `i` is even and `max` when `i` is odd, storing the result in-place.
    ///
    /// # Complexity
    /// - Time: O(n) total across all rounds (geometric series)
    /// - Space: O(1) auxiliary (mutates in place)
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        while n > 1 {
            n >>= 1;
            for i in 0..n {
                nums[i] = match i % 2 {
                    0 => nums[i << 1].min(nums[(i << 1) | 1]),
                    _ => nums[i << 1].max(nums[(i << 1) | 1]),
                };
            }
        }
        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::min_max_game(vec![3]), 3);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::min_max_game(vec![5, 2]), 2);
    }

    #[test]
    fn test_four_elements() {
        assert_eq!(Solution::min_max_game(vec![1, 2, 3, 4]), 1);
    }
}
