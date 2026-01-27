impl Solution {
    /// Cyclic sort placing each value at its correct index for first missing positive.
    ///
    /// # Intuition
    /// The answer must be in `[1, n+1]`. By placing each value `v` at index
    /// `v-1` (when `1 <= v <= n`), the first index where `nums[i] != i+1`
    /// reveals the missing positive.
    ///
    /// # Approach
    /// Iterate through the array, swapping `nums[i]` to its target position
    /// `nums[i]-1` as long as the value is in range and not already placed.
    /// Then scan for the first mismatch.
    ///
    /// # Complexity
    /// - Time: O(n) — each element swapped at most once
    /// - Space: O(1) — in-place rearrangement
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[i] != nums[nums[i] as usize - 1] {
                let target = nums[i] as usize - 1;
                nums.swap(i, target);
            }
        }

        (0..n)
            .find(|&i| nums[i] != (i + 1) as i32)
            .map_or((n + 1) as i32, |i| (i + 1) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn missing_in_middle() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn consecutive_from_one() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }

    #[test]
    fn single_element_one() {
        assert_eq!(Solution::first_missing_positive(vec![1]), 2);
    }

    #[test]
    fn single_element_not_one() {
        assert_eq!(Solution::first_missing_positive(vec![2]), 1);
    }
}
