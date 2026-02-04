impl Solution {
    /// Choose the larger endpoint, which Alice can force.
    ///
    /// # Intuition
    /// Any valid move deletes a contiguous block, so at least one of the current
    /// endpoints always remains. The player to move can end the game by deleting
    /// everything except the preferred endpoint.
    ///
    /// # Approach
    /// The maximizer can always finish immediately by leaving the larger of the
    /// two endpoints, guaranteeing at least that value. Conversely, after any
    /// move, at least one of the previous endpoints remains, so the opponent can
    /// force a result no better than that endpoint. Therefore, Alice's optimal
    /// play yields the maximum of the first and last elements.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn final_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let first = nums[0];
        let last = *nums.last().unwrap();
        first.max(last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::final_element(vec![1, 5, 2]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::final_element(vec![3, 7]), 7);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::final_element(vec![42]), 42);
    }

    #[test]
    fn test_large_middle_ignored() {
        assert_eq!(Solution::final_element(vec![1, 100, 100, 1]), 1);
    }
}
